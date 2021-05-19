use kana;

#[test]
fn performance() -> Result<(), Box<dyn std::error::Error>> {
    let hiragana_time = std::time::Instant::now();
    for _ in 0..1000 {
        kana::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
        kana::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
        kana::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
        kana::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
        kana::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
    }
    let hiragana_end = hiragana_time.elapsed().as_millis();

    let katakana_time = std::time::Instant::now();
    for _ in 0..1000 {
        kana::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
        kana::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
        kana::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
        kana::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
        kana::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
    }
    let katakana_end = katakana_time.elapsed().as_millis();

    println!("hiragana: {:?}\nkatakana: {:?}", hiragana_end, katakana_end);

    Ok(())
}