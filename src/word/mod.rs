use std::collections::HashMap;

pub struct Word {
    pub original: String,
    pub tokens: Vec<String>,
    pub kana: String
}

impl Word {
    pub fn new(original: String) -> Word {
        Word { 
            original: original,
            tokens: Vec::new(),
            kana: String::new(), 
        }
    }
}

/**
 * Return a vector of Words initialised from input
 */
pub fn init_word_collection(input: Vec<String>) -> Vec<Word> {
    let mut v: Vec<Word> = Vec::new();
    for i in input {
        v.push(Word::new(i.to_owned()));
    }
    return v
}

/**
 * Set the kana field in a vector of Words.
 */
pub fn set_kana(words: &mut Vec<Word>, set: &HashMap<&str,&str>) {
    for word in words {
        word.kana = token_to_kana(&word.tokens, &set);
    }
}

/**
 * Return a string of Kana based on tokens and the set of characters
 */
fn token_to_kana(tokens: &Vec<String>, set: &HashMap<&str,&str>) -> String {
    let mut s = String::with_capacity(tokens.len() * 2);
    for token in tokens {
        s.push_str(set.get(&token.as_str()).expect("No such token in map"));
    }
    s
}