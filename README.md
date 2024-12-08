# goldendict-helper

Usally we use goldendict-ng with anki : https://xiaoyifang.github.io/goldendict-ng/topic_anki/


This project 

pros
      - 避免了 anki 中的 bug
              - https://github.com/xiaoyifang/goldendict-ng/discussions/1885
      - 不用打开 anki-connect
              - 相当于 headless anki
      - 比 anki 少点一次鼠标

cons
      - theme 不够好看

## Install

```
cargo install --path .
```

Setup `add_word` to goldendict-ng's program dictionary

## How to review

1. Start goldendict-ng


2. `~/.config/awesome/rc.lua`
```
{
    rule = { instance = "goldendict"},
    properties = {
        width = 1900,
        height = 800,
        floating = true,
        titlebars_enabled = false,
        requests_no_titlebar = true,
        x = 10,
        y = 200,
        focus = false,
        border_width = 0,
        ontop = true,
    }
}
```

3. `review`
