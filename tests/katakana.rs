use kana;

#[test]
fn test_valid_katakana() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(kana::to_katakana("a")?, "ア");
    assert_eq!(kana::to_katakana("kokonattsu")?, "ココナッツ");
    Ok(())
}