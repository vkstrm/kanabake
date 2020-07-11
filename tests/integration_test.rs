extern crate kana;

const SINGLE_HIRAGANA: fn(&str) -> String = |input: &str| -> String {
    let v = kana::transform(false, vec![String::from(input)]);
    v[0].to_owned()
};

const SINGLE_KATAKANA: fn(&str) -> String = |input: &str| -> String {
    let v = kana::transform(true, vec![String::from(input)]);
    v[0].to_owned()
};

#[test]
fn can_return_hiragana() {
    assert_eq!("あ", SINGLE_HIRAGANA("A"));
}

#[test]
fn can_return_katakana() {
    assert_eq!("ア", SINGLE_KATAKANA("A"));
}