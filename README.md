# kanabake

Transform wapuro input into either Katakana or Hiragana.

## Usage



## Examples

```rust
use kanabake;

let example = "konnichiha";
let hiragana = kanabake::to_hiragana(example)?;

assert_eq!(hiragana, "こんにちは");
```
