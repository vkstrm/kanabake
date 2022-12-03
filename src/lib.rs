#![warn(missing_docs)]
//! Funtionality for converting between roman character and japanese kana.
//!
//! Uses kana spelling, Wapuro romaji as input, and produces
//! either Katakana or Hiragana as output.
//!
pub mod error;
mod parser;
mod set;

use error::Error;
use parser::parse;
use set::CharacterSet;
use set::KanaType::{Hiragana, Katakana};

/// Transform wapuro romaji string to katakana string.
///
/// Valid input is ASCII only.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let romaji = "kokonattsu";
/// let katakana = kanabake::to_katakana(romaji)?;
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
        }
        Err(why) => Err(why),
    }
}

/// Transform wapuro romaji string to hiragana string.
///
/// Valid input is ASCII only.
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let romaji = "konnichiha";
/// let hiragana = kanabake::to_hiragana(romaji)?;
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
        }
        Err(why) => Err(why),
    }
}

/// Validate if input can be transformed to kana.
///
/// # Example
/// ```rust
/// # fn main() {
/// assert_eq!(kanabake::is_valid("konnichiha"), true);
/// assert_eq!(kanabake::is_valid("könnichiha"), false);
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

/// Check if all characters in string input are hiragana.
///
/// # Example
/// ```rust
/// # fn main() {
/// assert_eq!(kanabake::is_hiragana("あいうえお"), true);
/// # }
pub fn is_hiragana(input: &str) -> bool {
    let hir_start: u32 = 'ぁ' as u32;
    let hir_end: u32 = 'ゟ' as u32;
    for c in input.chars() {
        let unicode_val = c as u32;
        if unicode_val < hir_start || unicode_val > hir_end {
            return false;
        }
    }
    true
}

/// Check if all characters in string input are katakana.
///
/// # Example
/// ```rust
/// # fn main() {
/// assert_eq!(kanabake::is_katakana("アイウエオ"), true);
/// # }
pub fn is_katakana(input: &str) -> bool {
    let kat_start: u32 = '゠' as u32;
    let kat_end: u32 = 'ヿ' as u32;
    for c in input.chars() {
        let unicode_val = c as u32;
        if unicode_val < kat_start || unicode_val > kat_end {
            return false;
        }
    }
    true
}

fn validate_input(input: &str) -> Result<String, Error> {
    if input.is_empty() {
        return Ok(String::new());
    }

    if !input.is_ascii() {
        return Err(Error::new("non-ASCII character in input"));
    }

    Ok(input.trim().to_ascii_uppercase())
}

fn internal_transform(set: &CharacterSet, uppercase_input: String) -> Result<String, Error> {
    let tokens = tokenize(&uppercase_input)?;
    let mut word = String::new();
    for token in tokens {
        match set.get(token) {
            Some(kana) => word.push_str(kana),
            None => return Err(Error::new(&format!("token {} is not in map", token))),
        }
    }

    Ok(word)
}

fn tokenize(input: &str) -> Result<Vec<&str>, Error> {
    match parse(input) {
        Ok(tokens) => Ok(tokens),
        Err(why) => Err(why),
    }
}
