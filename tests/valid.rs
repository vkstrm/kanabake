use kanabake;

#[test]
fn test_is_valid() {
    assert_eq!(kanabake::is_valid(""), true);
    assert_eq!(kanabake::is_valid("konnichiha"), true);
}

#[test]
fn test_is_invalid() {
    assert_eq!(kanabake::is_valid("x"), false);
    assert_eq!(kanabake::is_valid("konnichihä"), false);
    assert_eq!(kanabake::is_valid("kicheese"), false);
}
