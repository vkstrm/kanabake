mod sets;
mod tokens;
mod word;

type Sets = sets::Sets;

/**
 * Transform letters to kana and print to stdout
 */
pub fn transform_to_stdout(katakana_mode: bool, input_vec: Vec<String>) {
    let words = internal_transform(katakana_mode, input_vec);
    print(&words);
}

/**
 * Transform letters to kana and return result
 */
pub fn transform(katakana_mode: bool, input_vec: Vec<String>) -> Vec<String> {
    let words = internal_transform(katakana_mode, input_vec);
    let mut return_vec = Vec::new();
    for word in words {
        return_vec.push(word.kana);
    }
    return_vec
}

fn internal_transform(katakana_mode: bool, input_vec: Vec<String>) -> Vec<word::Word> {
    let hiragana_set: Sets = Sets::new(sets::KanaType::Hiragana);
    let katakana_set: Sets = Sets::new(sets::KanaType::Katakana);

    let builder: WordBuilder;
    if katakana_mode {
        builder = WordBuilder::new(katakana_set);
    } else {
        builder = WordBuilder::new(hiragana_set);
    }

    let mut words = word::init_word_collection(input_vec);

    for word in &mut words {
        builder.transform(word);
    }

    words
}

fn print(words: &Vec<word::Word>) {
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