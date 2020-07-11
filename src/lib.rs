mod sets;
mod tokens;
mod word;

type Sets = sets::Sets;

/**
 * Turn input into kana and print
 */
pub fn transform(katakana_mode: &bool, input_vec: Vec<String>) {
    let hiragana_set: Sets = Sets::new(sets::KanaType::Hiragana);
    let katakana_set: Sets = Sets::new(sets::KanaType::Katakana);

    let builder: WordBuilder;
    if *katakana_mode {
        builder = WordBuilder::new(katakana_set);
    } else {
        builder = WordBuilder::new(hiragana_set);
    }

    let mut words = word::init_word_collection(input_vec);

    for word in &mut words {
        builder.transform(word);
    }

    for word in words {
        println!("{}",word.kana);
    }
}

struct WordBuilder {
    set: Sets,
}

impl WordBuilder {

    pub fn new(set: Sets) -> WordBuilder {
        WordBuilder {
            set: set
        }
    }

    pub fn transform(&self, word: &mut word::Word) {
        word.set_tokens(tokens::interpret_tokens(&word.original));
        word.set_kana(&self.set.kana_dict);
    }
}