use kana;

#[test]
fn test_valid_hiragana() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(kana::to_hiragana("")?, "");
    assert_eq!(kana::to_hiragana("a")?, "あ");
    assert_eq!(kana::to_hiragana("aaaa")?, "ああああ");
    assert_eq!(kana::to_hiragana("aiueo")?, "あいうえお");
    assert_eq!(kana::to_hiragana("nna")?, "んな");
    assert_eq!(kana::to_hiragana("nnn")?, "んんん");
    assert_eq!(kana::to_hiragana("onna")?, "おんな");
    assert_eq!(kana::to_hiragana("du")?, "づ");
    assert_eq!(kana::to_hiragana("mitte")?, "みって");
    assert_eq!(kana::to_hiragana("nihongonogasuki")?, "にほんごのがすき");
    Ok(())
}