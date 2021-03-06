mod sets;
mod tokens;
mod word;

type Sets = sets::Sets;

/**
 * Transform letters to kana and print to stdout
 */
pub fn transform_to_stdout(katakana_mode: bool, input_vec: &Vec<String>) {
    let words = internal_transform(katakana_mode, input_vec);
    for word in words {
        println!("{}",word.kana);
    }
}

/**
 * Transform letters to kana and return result
 */
pub fn transform(katakana_mode: bool, input_vec: &Vec<String>) -> Vec<String> {
    let words = internal_transform(katakana_mode, input_vec);
    let mut return_vec = Vec::new();
    for word in words {
        return_vec.push(word.kana);
    }
    return_vec
}

fn internal_transform(katakana_mode: bool, input_vec: &Vec<String>) -> Vec<word::Word> {
    let set: Sets; 
    if katakana_mode {
        set = Sets::new(sets::KanaType::Katakana);
    } else {
        set = Sets::new(sets::KanaType::Hiragana);
    }

    let mut words = word::init_word_collection(input_vec);

    for word in &mut words {
        word.set_tokens(tokens::interpret_tokens(&word.original));
        word.set_kana(&set.kana_dict);
    }

    words
}