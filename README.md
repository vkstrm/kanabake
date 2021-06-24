# kana

Transform wapuro input into either Katakana or Hiragana.

```rust
use kana;

let example = "konnichiha";
let hiragana = kana::to_hiragana(example)?;

assert_eq!(hiragana, "こんにちは");
```
