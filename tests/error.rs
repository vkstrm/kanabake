use kana;

#[test]
fn test_error_message_len_plus3() {
    let result = kana::to_hiragana("kicheese");
    assert_eq!(result.is_err(), true);
    let error = result.unwrap_err();
    assert_eq!(error.message(), "invalid sequence: \"che\"")
}

#[test]
fn test_error_message_len_3() {
    let result = kana::to_hiragana("kiche");
    assert_eq!(result.is_err(), true);
    let error = result.unwrap_err();
    assert_eq!(error.message(), "invalid sequence: \"che\"")
}

#[test]
fn test_error_message_len_2() {
    let result = kana::to_hiragana("kich");
    assert_eq!(result.is_err(), true);
    let error = result.unwrap_err();
    assert_eq!(error.message(), "invalid sequence: \"ch\"")
}

#[test]
fn test_error_message_len_1() {
    let result = kana::to_hiragana("kix");
    assert_eq!(result.is_err(), true);
    let error = result.unwrap_err();
    assert_eq!(error.message(), "invalid sequence: \"x\"")
}