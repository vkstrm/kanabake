use std::collections::HashMap;
use lazy_static::lazy_static;

use super::error::Error;

pub struct Parser {
    allowed_head: Vec<&'static str>,
    allowed_base: Vec<&'static str>,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            allowed_head: vec!["K","G","S","Z","T","D","J","N","H","B","P","F","M","Y","R","W"],
            allowed_base: vec!["A","I","U","E","O"]
        }
    }

    pub fn parse<'a>(&self, input: &'a str) -> Result<Vec<&'a str>, Error> {
        if input.is_empty() {
            return Ok(Vec::new())
        }
        
        let mut opt_token = self.search_for_token(&input);
        if opt_token.is_none() {
            return Err(Error::new("invalid sequence"))
        }
    
        let mut tokens: Vec<&str> = Vec::new();
        let token = opt_token.unwrap();
        tokens.push(token.0);
        let mut remain = token.1;
        while remain.len() > 0 {
            opt_token = self.search_for_token(&remain);
            if opt_token.is_none() {
                return Err(Error::from_remain(remain))
            }
    
            let token = opt_token.unwrap();
            tokens.push(token.0);
            remain = token.1;
        }
    
        Ok(tokens)
    }

    fn search_for_token<'a>(&self, remain: &'a str) -> Option<(&'a str, &'a str)> {
        let one = self.length_one(remain);
        if let Some(one) = one {
            return Some(one)
        }

        let two = self.length_two(remain);
        if let Some(two) = two {
            return Some(two)
        }

        let three = self.length_three(remain);
        if let Some(three) = three {
            return Some(three)
        }
        None
    }

    /**
     * Search for Tokens of length one.
     * Return a found token and a splice of remaining input string.
     */
    fn length_one<'a>(&self, input: &'a str) -> Option<(&'a str, &'a str)> {
        if input.len() < 1 {
            return None
        }

        let token = &input[..1];
        let remain = &input[1..];
        if self.allowed_base.contains(&token) {
            return Some((token, remain));
        }
        if token == "N" {
            if remain.len() < 1 {
                return Some((token, remain));
            }
            let r = &remain[..1];
            if self.allowed_head.contains(&r) {
                return Some((token,remain));
            }
        }
        None
    }

    fn length_two<'a>(&self, input: &'a str) -> Option<(&'a str, &'a str)> {
        if input.len() < 2 {
            return None
        }

        let token = token_to_tuple(&input[..2], 1);
        let remain = &input[2..];

        if &token.0 == &token.1 && self.allowed_head.contains(&token.0)  {
            return Some(("LTSU", &input[1..]))
        }

        if self.allowed_head.contains(&token.0) && self.allowed_base.contains(&token.1) {
            return Some((&input[..2], remain))
        }
        None
    }

    fn length_three<'a>(&self, input: &'a str) -> Option<(&'a str, &'a str)> {
        if input.len() < 3 {
            return None
        }

        let token = token_to_tuple(&input[..3], 2);
        let remain = &input[3..];

        if ALLOWED_MAP.contains_key(token.0) 
            && ALLOWED_MAP.get(token.0).expect("Oops").contains(&token.1) {
                return Some((&input[..3], remain));
        }

        None
    }
}


lazy_static! {
    static ref ALLOWED_MAP: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("KY", vec!["A","U","O"]);
        map.insert("GY", vec!["A","U","O"]);
        map.insert("SH", vec!["A","I","U","O"]);
        map.insert("CH", vec!["A","I","U","O"]);
        map.insert("TS", vec!["U"]);
        map.insert("DZ", vec!["U"]);
        map.insert("NY", vec!["A","U","O"]);
        map.insert("DY", vec!["A","U","O"]);
        map.insert("JY", vec!["A","U","O"]);
        map.insert("RY", vec!["A","U","O"]);
        map.insert("HY", vec!["A","U","O"]);
        map.insert("BY", vec!["A","U","O"]);
        map.insert("PY", vec!["A","U","O"]);
        map.insert("MY", vec!["A","U","O"]);
        map
    };
}

/**
 * Split token to str slices at index, return as tuple.
 */
fn token_to_tuple(token: &str, split_index: usize) -> (&str,&str) {
    token.split_at(split_index)
}
