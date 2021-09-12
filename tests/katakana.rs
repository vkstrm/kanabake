use kanabake;

#[test]
fn test_katakana_all() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_katakana("aiueo")?, "アイウエオ");
    assert_eq!(kanabake::to_katakana("kakikukeko")?, "カキクケコ");
    assert_eq!(kanabake::to_katakana("gagigugego")?, "ガギグゲゴ");
    assert_eq!(kanabake::to_katakana("sashisuseso")?, "サシスセソ");
    assert_eq!(kanabake::to_katakana("zajizuzezo")?, "ザジズゼゾ");
    assert_eq!(kanabake::to_katakana("tachitsuteto")?, "タチツテト");
    assert_eq!(kanabake::to_katakana("dadidzudedo")?, "ダヂヅデド");
    assert_eq!(kanabake::to_katakana("naninuneno")?, "ナニヌネノ");
    assert_eq!(kanabake::to_katakana("hahifuheho")?, "ハヒフヘホ");
    assert_eq!(kanabake::to_katakana("babibubebo")?, "バビブベボ");
    assert_eq!(kanabake::to_katakana("papipupepo")?, "パピプペポ");
    assert_eq!(kanabake::to_katakana("mamimumemo")?, "マミムメモ");
    assert_eq!(kanabake::to_katakana("yayuyo")?, "ヤユヨ");
    assert_eq!(kanabake::to_katakana("rarirurero")?, "ラリルレロ");
    assert_eq!(kanabake::to_katakana("wawo")?, "ワヲ");
    Ok(())
}

#[test]
fn test_valid_katakana() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_katakana("")?, "");
    assert_eq!(kanabake::to_katakana("a")?, "ア");
    assert_eq!(kanabake::to_katakana("aaaaa")?, "アアアアア");
    assert_eq!(kanabake::to_katakana("kokonattsu")?, "ココナッツ");
    Ok(())
}

#[test]
fn test_katakana_n() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_katakana("n")?, "ン");
    assert_eq!(kanabake::to_katakana("nna")?, "ンナ");
    assert_eq!(kanabake::to_katakana("nnn")?, "ンンン");
    Ok(())
}

#[test]
fn test_katakana_variants() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_katakana("dudi")?, "ヅヂ");
    assert_eq!(kanabake::to_katakana("hufu")?, "フフ");
    Ok(())
}

#[test]
fn test_katakana_ltsu() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_katakana("ltsu")?, "ッ");
    assert_eq!(kanabake::to_katakana("altsuo")?, "アッオ");
    Ok(())
}

#[test]
fn test_is_katakana() {
    assert_eq!(kanabake::is_katakana("アイウエオ"), true);
    assert_eq!(kanabake::is_katakana("ンンン"), true);
    assert_eq!(kanabake::is_katakana("ギョウキョウ"), true);
    assert_eq!(kanabake::is_katakana("AIEUO"), false);
    assert_eq!(kanabake::is_katakana("あいうえお"), false);
}