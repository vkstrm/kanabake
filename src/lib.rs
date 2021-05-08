mod set;
mod tokens;

use set::KanaType::{Hiragana, Katakana};
use set::{CharacterSet};

pub fn to_katakana(input: &str) -> String {
    let set = CharacterSet::new(Katakana);
    internal_transform(&set, input)
}

pub fn to_hiragana(input: &str) -> String {
    let set = CharacterSet::new(Hiragana);
    internal_transform(&set, input)
}

fn internal_transform(set: &CharacterSet, input: &str) -> String {
    let tokens = tokens::interpret_tokens(input);
    let mut word = String::new();
    for token in tokens {
        word.push_str(set.get(&token));
    }

    word
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hiragana() {
        assert_eq!(to_hiragana("a"), "あ");
        assert_eq!(to_hiragana("konnichiha"), "こんにちは");
        assert_eq!(to_hiragana("toukyou"), "とうきょう");
        assert_eq!(to_hiragana("nihongonogasuki"), "にほんごのがすき")
    }

    #[test]
    fn test_katakana() {
        assert_eq!(to_katakana("kokonatsu"), "ココナツ");
    }
}