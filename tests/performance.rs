use kanabake;

#[test] // Run with cargo test --release -- --nocapture
fn performance() -> Result<(), kanabake::error::Error> {
    let hiragana_time = std::time::Instant::now();
    for _ in 0..1000 {
        kanabake::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
        kanabake::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
        kanabake::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
        kanabake::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
        kanabake::to_hiragana("aiueokakikukekosashisusesotachitsuteto")?;
    }
    let hiragana_end = hiragana_time.elapsed().as_millis();

    let katakana_time = std::time::Instant::now();
    for _ in 0..1000 {
        kanabake::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
        kanabake::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
        kanabake::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
        kanabake::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
        kanabake::to_katakana("aiueokakikukekosashisusesotachitsuteto")?;
    }
    let katakana_end = katakana_time.elapsed().as_millis();

    println!("hiragana: {:?}\nkatakana: {:?}", hiragana_end, katakana_end);

    Ok(())
}
