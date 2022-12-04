use crate::kana::KanaType;

const HIR_START: u32 = 'ぁ' as u32;
const HIR_END: u32 = 'ゟ' as u32;

const KAT_START: u32 = '゠' as u32;
const KAT_END: u32 = 'ヿ' as u32;

pub fn is_kana(input: &str, kana: KanaType) -> bool {
    let (start, end) = match kana {
        KanaType::Hiragana => (HIR_START, HIR_END),
        KanaType::Katakana => (KAT_START, KAT_END),
    };

    for c in input.chars() {
        let unicode_val = c as u32;
        if unicode_val < start || unicode_val > end {
            return false;
        }
    }
    true
}
