use std::collections::HashMap;

pub enum KanaType {
    Hiragana,
    Katakana,
}

pub struct CharacterSet {
    pub kana_dict: HashMap<&'static str, &'static str>,
}

impl CharacterSet {
    pub fn new(set_type: KanaType) -> CharacterSet {
        CharacterSet {
            kana_dict: match set_type {
                KanaType::Hiragana => hiragana(),
                KanaType::Katakana => katakana(),
            },
        }
    }

    pub fn get(&self, key: &str) -> &str {
        let s = self.kana_dict.get(&key).expect("No such token in map");
        s
    }
}

pub fn hiragana() -> HashMap<&'static str, &'static str> {
    let map: HashMap<&str, &str> = [
        ("A", "あ"),
        ("I", "い"),
        ("U", "う"),
        ("E", "え"),
        ("O", "お"),
        ("KA", "か"),
        ("KI", "き"),
        ("KYA", "きゃ"),
        ("KYU", "きゅ"),
        ("KYO", "きょ"),
        ("KU", "く"),
        ("KE", "け"),
        ("KO", "こ"),
        ("GA", "が"),
        ("GI", "ぎ"),
        ("GYA", "ぎゃ"),
        ("GYU", "ぎゅ"),
        ("GYO", "ぎょ"),
        ("GU", "ぐ"),
        ("GE", "げ"),
        ("GO", "ご"),
        ("SA", "さ"),
        ("SHI", "し"),
        ("SHA", "しゃ"),
        ("SHU", "しゅ"),
        ("SHO", "しょ"),
        ("SU", "す"),
        ("SE", "せ"),
        ("SO", "そ"),
        ("ZA", "ざ"),
        ("JI", "じ"),
        ("JA", "じゃ"),
        ("JU", "じゅ"),
        ("JO", "じょ"),
        ("JYA", "じゃ"),
        ("JYU", "じゅ"),
        ("JYO", "じょ"),
        ("ZU", "ず"),
        ("ZE", "ぜ"),
        ("ZO", "ぞ"),
        ("TA", "た"),
        ("CHI", "ち"),
        ("CHA", "ちゃ"),
        ("CHU", "ちゅ"),
        ("CHO", "ちょ"),
        ("TSU", "つ"),
        ("TE", "て"),
        ("TO", "と"),
        ("DA", "だ"),
        ("DI", "ぢ"),
        ("DYA", "ぢゃ"),
        ("DYU", "ぢゅ"),
        ("DYO", "ぢょ"),
        ("DU", "づ"),
        ("DZU", "づ"),
        ("DE", "で"),
        ("DO", "ど"),
        ("NA", "な"),
        ("NI", "に"),
        ("NYA", "にゃ"),
        ("NYU", "にゅ"),
        ("NYO", "にょ"),
        ("NU", "ぬ"),
        ("NE", "ね"),
        ("NO", "の"),
        ("HA", "は"),
        ("HI", "ひ"),
        ("HYA", "ひゃ"),
        ("HYU", "ひゅ"),
        ("HYO", "ひょ"),
        ("FU", "ふ"),
        ("HE", "へ"),
        ("HO", "ほ"),
        ("BA", "ば"),
        ("BI", "び"),
        ("BYA", "びゃ"),
        ("BYU", "びゅ"),
        ("BYO", "びょ"),
        ("BU", "ぶ"),
        ("BE", "べ"),
        ("BO", "ぼ"),
        ("PA", "ぱ"),
        ("PI", "ぴ"),
        ("PYA", "ぴゃ"),
        ("PYU", "ぴゅ"),
        ("PYO", "ぴょ"),
        ("PU", "ぷ"),
        ("PE", "ぺ"),
        ("PO", "ぽ"),
        ("MA", "ま"),
        ("MI", "み"),
        ("MYA", "みゃ"),
        ("MYU", "みゅ"),
        ("MYO", "みょ"),
        ("MU", "む"),
        ("ME", "め"),
        ("MO", "も"),
        ("YA", "や"),
        ("YU", "ゆ"),
        ("YO", "よ"),
        ("RA", "ら"),
        ("RI", "り"),
        ("RYA", "りゃ"),
        ("RYU", "りゅ"),
        ("RYO", "りょ"),
        ("RU", "る"),
        ("RE", "れ"),
        ("RO", "ろ"),
        ("WA", "わ"),
        ("WO", "を"),
        ("N", "ん"),
        ("LTSU", "っ"),
    ]
    .iter()
    .cloned()
    .collect();
    map
}

pub fn katakana() -> HashMap<&'static str, &'static str> {
    let map: HashMap<&str, &str> = [
        ("A", "ア"),
        ("I", "イ"),
        ("U", "ウ"),
        ("E", "エ"),
        ("O", "オ"),
        ("KA", "カ"),
        ("KI", "キ"),
        ("KYA", "キャ"),
        ("KYU", "キュ"),
        ("KYO", "キョ"),
        ("KU", "ク"),
        ("KE", "ケ"),
        ("KO", "コ"),
        ("GA", "ガ"),
        ("GI", "ギ"),
        ("GYA", "ギャ"),
        ("GYU", "ギュ"),
        ("GYO", "ギョ"),
        ("GU", "グ"),
        ("GE", "ゲ"),
        ("GO", "ゴ"),
        ("SA", "サ"),
        ("SHI", "シ"),
        ("SHA", "シャ"),
        ("SHU", "シュ"),
        ("SHO", "ショ"),
        ("SU", "ス"),
        ("SE", "セ"),
        ("SO", "ソ"),
        ("ZA", "ザ"),
        ("JI", "ジ"),
        ("JA", "ジャ"),
        ("JU", "ジュ"),
        ("JO", "ジョ"),
        ("JYA", "ジャ"),
        ("JYU", "ジュ"),
        ("JYO", "ジョ"),
        ("ZU", "ズ"),
        ("ZE", "ゼ"),
        ("ZO", "ゾ"),
        ("TA", "タ"),
        ("CHI", "チ"),
        ("CHA", "チャ"),
        ("CHU", "チュ"),
        ("CHO", "チョ"),
        ("TSU", "ツ"),
        ("TE", "テ"),
        ("TO", "ト"),
        ("DA", "ダ"),
        ("DI", "ヂ"),
        ("DYA", "ヂャ"),
        ("DYU", "ヂュ"),
        ("DYO", "ヂョ"),
        ("DU", "ヅ"),
        ("DZU", "ヅ"),
        ("DE", "デ"),
        ("DO", "ド"),
        ("NA", "ナ"),
        ("NI", "ニ"),
        ("NYA", "ニャ"),
        ("NYU", "ニュ"),
        ("NYO", "ニョ"),
        ("NU", "ヌ"),
        ("NE", "ネ"),
        ("NO", "ノ"),
        ("HA", "ハ"),
        ("HI", "ヒ"),
        ("HYA", "ヒャ"),
        ("HYU", "ヒュ"),
        ("HYO", "ヒョ"),
        ("FU", "フ"),
        ("HE", "ヘ"),
        ("HO", "ホ"),
        ("BA", "バ"),
        ("BI", "ビ"),
        ("BYA", "ビャ"),
        ("BYU", "ビュ"),
        ("BYO", "ビョ"),
        ("BU", "ブ"),
        ("BE", "ベ"),
        ("BO", "ボ"),
        ("PA", "パ"),
        ("PI", "ピ"),
        ("PYA", "ピャ"),
        ("PYU", "ピュ"),
        ("PYO", "ピョ"),
        ("PU", "プ"),
        ("PE", "ペ"),
        ("PO", "ポ"),
        ("MA", "マ"),
        ("MI", "ミ"),
        ("MYA", "ミャ"),
        ("MYU", "ミュ"),
        ("MYO", "ミョ"),
        ("MU", "ム"),
        ("ME", "メ"),
        ("MO", "モ"),
        ("YA", "ヤ"),
        ("YU", "ユ"),
        ("YO", "ヨ"),
        ("RA", "ラ"),
        ("RI", "リ"),
        ("RYA", "リャ"),
        ("RYU", "リュ"),
        ("RYO", "リョ"),
        ("RU", "ル"),
        ("RE", "レ"),
        ("RO", "ロ"),
        ("WA", "ワ"),
        ("WO", "ヲ"),
        ("N", "ン"),
        ("LTSU", "ッ"),
    ]
    .iter()
    .cloned()
    .collect();
    map
}