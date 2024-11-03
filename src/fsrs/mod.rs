use crate::spaced_repetition::SpacedRepetiton;
use anyhow::Context;
use anyhow::Result;
use chrono::Utc;
use rs_fsrs::Card;
use rs_fsrs::Rating;
use sqlx::Row;
use sqlx::SqlitePool;

pub mod sqlite_history;

impl SpacedRepetiton for sqlite_history::SQLiteHistory {
    async fn next_to_review(&mut self) -> Result<String> {
        match  sqlx::query("SELECT rowid, word FROM fsrs WHERE timediff('now', substr(due, 2, length(due) - 2)) LIKE '+%' AND rowid > $1 ORDER BY RANDOM() LIMIT 1;")
                .bind(self.row_id)
                .fetch_one(&self.conn)
                .await {
                    Ok(row) => {
                        self.row_id = row.get(0);
                        let word: String = row.get(1);
                        Ok(word)
                    }
                    Err(_) => {
                        // search from start
                        let row = sqlx::query("SELECT rowid, word FROM fsrs WHERE timediff('now', substr(due, 2, length(due) - 2)) LIKE '+%' ORDER BY RANDOM() LIMIT 1;")
                        .fetch_one(&self.conn)
                        .await?;
                        self.row_id = row.get(0);
                        let word: String = row.get(1);
                        Ok(word)
                    }
                }
    }

    async fn update(&self, question: &str, rating: Rating) -> Result<()> {
        let old_card = get_word(&self.conn, question)
            .await
            .context("get old card fail")?;
        let scheduling_info = self.fsrs.next(old_card, Utc::now(), rating);
        update(&self.conn, question, scheduling_info.card)
            .await
            .context("update fail")?;
        Ok(())
    }

    async fn remove(&mut self, question: &str) -> Result<()> {
        sqlx::query("DELETE FROM fsrs WHERE word = $1")
            .bind(question)
            .fetch_one(&self.conn)
            .await?;
        Ok(())
    }
}

async fn update(pool: &SqlitePool, word: &str, card: Card) -> Result<()> {
    sqlx::query("UPDATE fsrs SET due = $2, stability = $3, difficulty = $4, elapsed_days = $5, scheduled_days = $6, reps = $7, lapses = $8, state = $9, last_review = $10 WHERE word = $1")
        .bind(word)
        .bind(serde_json::to_string(&card.due)?)
        .bind(card.stability)
        .bind(card.difficulty)
        .bind(card.elapsed_days)
        .bind(card.scheduled_days)
        .bind(card.reps)
        .bind(card.lapses)
        .bind(serde_json::to_string(&card.state)?)
        .bind(serde_json::to_string(&card.last_review)?)
        .execute(pool)
        .await?;
    Ok(())
}

async fn get_word(pool: &SqlitePool, word: &str) -> Result<Card> {
    let sqlite_row = sqlx::query("SELECT due, stability, difficulty, elapsed_days, scheduled_days, reps, lapses, state, last_review
    FROM fsrs WHERE word = $1")
        .bind(word)
        .fetch_one(pool)
        .await?;

    let card: Card = Card {
        due: serde_json::from_str(sqlite_row.get(0))?,
        stability: sqlite_row.get(1),
        difficulty: sqlite_row.get(2),
        elapsed_days: sqlite_row.get(3),
        scheduled_days: sqlite_row.get(4),
        reps: sqlite_row.get(5),
        lapses: sqlite_row.get(6),
        state: serde_json::from_str(sqlite_row.get(7))?,
        last_review: serde_json::from_str(sqlite_row.get(8))?,
    };
    Ok(card)
}
