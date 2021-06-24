use kana;

#[test]
fn test_is_valid() {
    assert_eq!(kana::is_valid(""), true);
    assert_eq!(kana::is_valid("konnichiha"), true);
}

#[test]
fn test_is_invalid() {
    assert_eq!(kana::is_valid("x"), false);
    assert_eq!(kana::is_valid("konnichihä"), false);
    assert_eq!(kana::is_valid("kicheese"), false);
}