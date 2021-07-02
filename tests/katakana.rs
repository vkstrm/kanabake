use kana;

#[test]
fn test_katakana_all() -> Result<(), kana::error::Error> {
    assert_eq!(kana::to_katakana("aiueo")?, "アイウエオ");
    assert_eq!(kana::to_katakana("kakikukeko")?, "カキクケコ");
    assert_eq!(kana::to_katakana("gagigugego")?, "ガギグゲゴ");
    assert_eq!(kana::to_katakana("sashisuseso")?, "サシスセソ");
    assert_eq!(kana::to_katakana("zajizuzezo")?, "ザジズゼゾ");
    assert_eq!(kana::to_katakana("tachitsuteto")?, "タチツテト");
    assert_eq!(kana::to_katakana("dadidzudedo")?, "ダヂヅデド");
    assert_eq!(kana::to_katakana("naninuneno")?, "ナニヌネノ");
    assert_eq!(kana::to_katakana("hahifuheho")?, "ハヒフヘホ");
    assert_eq!(kana::to_katakana("babibubebo")?, "バビブベボ");
    assert_eq!(kana::to_katakana("papipupepo")?, "パピプペポ");
    assert_eq!(kana::to_katakana("mamimumemo")?, "マミムメモ");
    assert_eq!(kana::to_katakana("yayuyo")?, "ヤユヨ");
    assert_eq!(kana::to_katakana("rarirurero")?, "ラリルレロ");
    assert_eq!(kana::to_katakana("wawo")?, "ワヲ");
    Ok(())
}

#[test]
fn test_valid_katakana() -> Result<(), kana::error::Error> {
    assert_eq!(kana::to_katakana("")?, "");
    assert_eq!(kana::to_katakana("a")?, "ア");
    assert_eq!(kana::to_katakana("aaaaa")?, "アアアアア");
    assert_eq!(kana::to_katakana("kokonattsu")?, "ココナッツ");
    Ok(())
}

#[test]
fn test_katakana_n() -> Result<(), kana::error::Error> {
    assert_eq!(kana::to_katakana("n")?, "ン");
    assert_eq!(kana::to_katakana("nna")?, "ンナ");
    assert_eq!(kana::to_katakana("nnn")?, "ンンン");
    Ok(())
}

#[test]
fn test_katakana_variants() -> Result<(), kana::error::Error> {
    assert_eq!(kana::to_katakana("dudi")?, "ヅヂ");
    assert_eq!(kana::to_katakana("hufu")?, "フフ");
    assert_eq!(kana::to_katakana("altsuo")?, "アッオ");
    Ok(())
}