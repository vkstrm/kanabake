use kanabake;

#[test]
fn test_hiragana_all() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_hiragana("aiueo")?, "あいうえお");
    assert_eq!(kanabake::to_hiragana("kakikukeko")?, "かきくけこ");
    assert_eq!(kanabake::to_hiragana("gagigugego")?, "がぎぐげご");
    assert_eq!(kanabake::to_hiragana("sashisuseso")?, "さしすせそ");
    assert_eq!(kanabake::to_hiragana("zajizuzezo")?, "ざじずぜぞ");
    assert_eq!(kanabake::to_hiragana("tachitsuteto")?, "たちつてと");
    assert_eq!(kanabake::to_hiragana("dadidzudedo")?, "だぢづでど");
    assert_eq!(kanabake::to_hiragana("naninuneno")?, "なにぬねの");
    assert_eq!(kanabake::to_hiragana("hahifuheho")?, "はひふへほ");
    assert_eq!(kanabake::to_hiragana("babibubebo")?, "ばびぶべぼ");
    assert_eq!(kanabake::to_hiragana("papipupepo")?, "ぱぴぷぺぽ");
    assert_eq!(kanabake::to_hiragana("mamimumemo")?, "まみむめも");
    assert_eq!(kanabake::to_hiragana("yayuyo")?, "やゆよ");
    assert_eq!(kanabake::to_hiragana("rarirurero")?, "らりるれろ");
    assert_eq!(kanabake::to_hiragana("wawo")?, "わを");
    Ok(())
}

#[test]
fn test_valid_hiragana() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_hiragana("")?, "");
    assert_eq!(kanabake::to_hiragana("a")?, "あ");
    assert_eq!(kanabake::to_hiragana("aaaa")?, "ああああ");
    assert_eq!(kanabake::to_hiragana("onna")?, "おんな");
    assert_eq!(kanabake::to_hiragana("du")?, "づ");
    assert_eq!(kanabake::to_hiragana("mitte")?, "みって");
    assert_eq!(
        kanabake::to_hiragana("nihongonogasuki")?,
        "にほんごのがすき"
    );
    Ok(())
}

#[test]
fn test_hiragana_n() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_hiragana("n")?, "ん");
    assert_eq!(kanabake::to_hiragana("nna")?, "んな");
    assert_eq!(kanabake::to_hiragana("nnn")?, "んんん");
    Ok(())
}

#[test]
fn test_hiragana_variants() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_hiragana("dudi")?, "づぢ");
    assert_eq!(kanabake::to_hiragana("hufu")?, "ふふ");
    Ok(())
}

#[test]
fn test_hiragana_ltsu() -> Result<(), kanabake::error::Error> {
    assert_eq!(kanabake::to_hiragana("ltsu")?, "っ");
    assert_eq!(kanabake::to_hiragana("altsuo")?, "あっお");
    Ok(())
}

#[test]
fn test_is_hiragana() {
    assert_eq!(kanabake::is_hiragana("あいうえお"), true);
    assert_eq!(kanabake::is_hiragana("んんんん"), true);
    assert_eq!(kanabake::is_hiragana("ぎょうきょうわを"), true);
    assert_eq!(kanabake::is_hiragana("AIEUO"), false);
    assert_eq!(kanabake::is_hiragana("アイウエオ"), false);
}
