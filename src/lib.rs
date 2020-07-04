mod sets;
mod tokens;
mod word;

use std::collections::HashMap;

/**
 * Turn input into kana and print
 */
pub fn transform(katakana_mode: &bool, input_vec: Vec<String>) {
    let hiragana_set = sets::hiragana();
    let katakana_set = sets::katakana();
    let mut set: &HashMap<&str,&str> = &hiragana_set;
    if *katakana_mode {
        set = &katakana_set;
    }

    let mut words = word::init_word_collection(input_vec);
    set_tokens(&mut words);
    word::set_kana(&mut words, &set);

    for word in words {
        println!("{}",word.kana);
    }
}

/**
 * Set the token field in a collection of words
 */
pub fn set_tokens(words: &mut Vec<word::Word>) {
    for word in words {
        word.tokens = tokens::interpret_tokens(&word.original);
    }
}