#![warn(missing_docs)]
//! Funtionality for converting between roman character and japanese kana.
//! 
//! Uses kana spelling, or Wapuro romaji as input, and produces
//! either Katakana or Hiragana as output.
//! 
mod set;
mod tokens;
mod error;

use set::KanaType::{Hiragana, Katakana};
use set::{CharacterSet};
use error::Error;

/// String input to Katakana representation
/// 
/// From an ASCII Romaji input, get the UTF-8 Katakana 
/// representation of the same input.  
/// 
/// # Examples
/// 
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let romaji = "kokonattsu";
/// let katakana = kana::to_katakana(romaji)?;
/// 
/// assert_eq!(katakana, "ココナッツ");
/// # Ok(())
/// # }
/// ```
/// 
/// # Error
/// 
pub fn to_katakana(input: &str) -> Result<String, Error> {
    match validate_input(input) {
        Ok(uppercase_input) => {
            let set = CharacterSet::new(Katakana);
            internal_transform(&set, uppercase_input)
        },
        Err(why) => Err(why)
    }
}

/// Get Hiragana representation of string input
/// 
/// # Example
/// 
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let romaji = "konnichiha";
/// let hiragana = kana::to_hiragana(romaji)?;
/// 
/// assert_eq!(hiragana, "こんにちは");
/// # Ok(())
/// # }
/// ```
pub fn to_hiragana(input: &str) -> Result<String, Error> {
    match validate_input(input) {
        Ok(uppercase_input) => {
            let set = CharacterSet::new(Hiragana);
            internal_transform(&set, uppercase_input)
        },
        Err(why) => Err(why)
    }
}

/// Validate if input can be transformed to kana.
/// 
/// # Example
/// ```rust
/// # fn main() {
/// assert_eq!(kana::is_valid("konnichiha"), true);
/// assert_eq!(kana::is_valid("könnichiha"), false);
/// # }
pub fn is_valid(input: &str) -> bool {
    if let Ok(uppercase_input) = validate_input(input) {
        match tokenize(&uppercase_input) {
            Ok(_) => return true,
            Err(_) => return false,
        }   
    }

    false
}

fn validate_input(input: &str) -> Result<String, Error> {
    if input.is_empty() {
        return Ok(String::new())
    }

    if !input.is_ascii() {
        return Err(Error::new("non-ASCII character in input"))
    }

    Ok(input.trim().to_ascii_uppercase())
}

fn internal_transform(set: &CharacterSet, uppercase_input: String) -> Result<String, Error> {
    let tokens = tokenize(&uppercase_input)?;
    let mut word = String::new();
    for token in tokens {
        word.push_str(set.get(&token));
    }

    Ok(word)
}

fn tokenize(input: &str) -> Result<Vec<&str>, Error> {
    match tokens::parse(input) {
        Ok(tokens) => Ok(tokens),
        Err(why) => Err(why)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_hiragana() -> Result<(),Error> {
        assert_eq!(to_hiragana("")?, "");
        assert_eq!(to_hiragana("a")?, "あ");
        assert_eq!(to_hiragana("aaaa")?, "ああああ");
        assert_eq!(to_hiragana("aiueo")?, "あいうえお");
        assert_eq!(to_hiragana("nna")?, "んな");
        assert_eq!(to_hiragana("nnn")?, "んんん");
        assert_eq!(to_hiragana("du")?, "づ");
        assert_eq!(to_hiragana("mitte")?, "みって");
        assert_eq!(to_hiragana("nihongonogasuki")?, "にほんごのがすき");
        Ok(())
    }

    #[test]
    fn test_valid_katakana() -> Result<(),Error> {
        assert_eq!(to_katakana("kokonattsu")?, "ココナッツ");
        Ok(())
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("konnichiha"), true);
        assert_eq!(is_valid("konnichihä"), false);
    }
}