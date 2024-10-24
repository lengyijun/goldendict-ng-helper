use std::path::Path;
use std::path::PathBuf;

pub fn sort_str(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    // Sort the vector of characters
    chars.sort();

    // Collect the sorted characters back into a string
    chars.into_iter().collect()
}

pub fn groom_name(folder_name: &str) -> String {
    // remove ' in folder_name
    folder_name.replace(|c| c == '\'', "")
}

pub fn create_sub_dir(base_dir: &Path, prefer_name: String) -> PathBuf {
    let p = base_dir.join(&prefer_name);
    if !p.exists() {
        return p;
    }
    for i in 1.. {
        let p = base_dir.join(format!("{prefer_name}-{i}"));
        if !p.exists() {
            return base_dir.join(prefer_name);
        }
    }
    unreachable!()
}
