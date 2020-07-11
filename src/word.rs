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

    pub fn set_kana(&mut self, set: &HashMap<&str,&str>) {
        for token in &self.tokens {
            self.kana.push_str(set.get(&token.as_str()).expect("No such token in map"));
        }
    }

    pub fn set_tokens(&mut self, tokens: Vec<String>) {
        self.tokens = tokens;
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