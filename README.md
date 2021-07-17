# kanabake

Transform wapuro input into either Katakana or Hiragana.

## Usage
Add `kanabake` to your `Cargo.toml`
```toml
[dependencies]
kanabake = "0.1.0"
```

## Features
Currently the following functionality is available.
```rust
to_hiragana(&str) -> Result<String, Error>  // Ascii to hiragana
to_katakana(&str) -> Result<String, Error>  // Ascii to katakana
is_valid(&str) -> bool     // Can input be transformed?
```

## Examples

```rust
use kanabake;

let example = "konnichiha";
let hiragana = kanabake::to_hiragana(example)?;

assert_eq!(hiragana, "こんにちは");
```
