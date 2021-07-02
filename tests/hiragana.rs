use kana;

#[test]
fn test_hiragana_all() -> Result<(), kana::error::Error> {
    assert_eq!(kana::to_hiragana("aiueo")?, "あいうえお");
    assert_eq!(kana::to_hiragana("kakikukeko")?, "かきくけこ");
    assert_eq!(kana::to_hiragana("gagigugego")?, "がぎぐげご");
    assert_eq!(kana::to_hiragana("sashisuseso")?, "さしすせそ");
    assert_eq!(kana::to_hiragana("zajizuzezo")?, "ざじずぜぞ");
    assert_eq!(kana::to_hiragana("tachitsuteto")?, "たちつてと");
    assert_eq!(kana::to_hiragana("dadidzudedo")?, "だぢづでど");
    assert_eq!(kana::to_hiragana("naninuneno")?, "なにぬねの");
    assert_eq!(kana::to_hiragana("hahifuheho")?, "はひふへほ");
    assert_eq!(kana::to_hiragana("babibubebo")?, "ばびぶべぼ");
    assert_eq!(kana::to_hiragana("papipupepo")?, "ぱぴぷぺぽ");
    assert_eq!(kana::to_hiragana("mamimumemo")?, "まみむめも");
    assert_eq!(kana::to_hiragana("yayuyo")?, "やゆよ");
    assert_eq!(kana::to_hiragana("rarirurero")?, "らりるれろ");
    assert_eq!(kana::to_hiragana("wawo")?, "わを");
    Ok(())
}

#[test]
fn test_valid_hiragana() -> Result<(), kana::error::Error> {
    assert_eq!(kana::to_hiragana("")?, "");
    assert_eq!(kana::to_hiragana("a")?, "あ");
    assert_eq!(kana::to_hiragana("aaaa")?, "ああああ");
    assert_eq!(kana::to_hiragana("onna")?, "おんな");
    assert_eq!(kana::to_hiragana("du")?, "づ");
    assert_eq!(kana::to_hiragana("mitte")?, "みって");
    assert_eq!(kana::to_hiragana("nihongonogasuki")?, "にほんごのがすき");
    Ok(())
}

#[test]
fn test_hiragana_n() -> Result<(), kana::error::Error> {
    assert_eq!(kana::to_hiragana("n")?, "ん");
    assert_eq!(kana::to_hiragana("nna")?, "んな");
    assert_eq!(kana::to_hiragana("nnn")?, "んんん");
    Ok(())
}

#[test]
fn test_hiragana_variants() -> Result<(), kana::error::Error> {
    assert_eq!(kana::to_hiragana("dudi")?, "づぢ");
    assert_eq!(kana::to_hiragana("hufu")?, "ふふ");
    assert_eq!(kana::to_hiragana("altsuo")?, "あっお");
    Ok(())
}