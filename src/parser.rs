use lazy_static::lazy_static;
use std::collections::HashMap;

use super::error::Error;

const ALLOWED_HEAD: [&str; 16] = [
    "K", "G", "S", "Z", "T", "D", "J", "N", "H", "B", "P", "F", "M", "Y", "R", "W",
];
const ALLOWED_BASE: [&str; 5] = ["A", "I", "U", "E", "O"];
const LTSU: &str = "LTSU";

pub fn parse(input: &str) -> Result<Vec<&str>, Error> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let token = match search_for_token(input) {
        Some(token) => token,
        None => return Err(Error::new("invalid sequence")),
    };

    let mut tokens: Vec<&str> = Vec::new();
    tokens.push(token.0);
    let mut remain = token.1;

    while !remain.is_empty() {
        match search_for_token(remain) {
            Some(token) => {
                tokens.push(token.0);
                remain = token.1;
            }
            None => return Err(Error::from_remain(remain)),
        };
    }

    Ok(tokens)
}

fn search_for_token(remain: &str) -> Option<(&str, &str)> {
    let one = length_one(remain);
    if let Some(one) = one {
        return Some(one);
    }

    let two = length_two(remain);
    if let Some(two) = two {
        return Some(two);
    }

    let three = length_three(remain);
    if let Some(three) = three {
        return Some(three);
    }

    let four = length_four(remain);
    if let Some(four) = four {
        return Some(four);
    }

    None
}

/**
 * Search for Tokens of length one.
 * Return a found token and a splice of remaining input string.
 */
fn length_one(input: &str) -> Option<(&str, &str)> {
    if input.is_empty() {
        return None;
    }

    let token = &input[..1];
    let remain = &input[1..];
    if ALLOWED_BASE.contains(&token) {
        return Some((token, remain));
    }

    if token == "N" {
        if remain.is_empty() {
            return Some((token, remain));
        }
        // Next char is start of new token
        let r = &remain[..1];
        if ALLOWED_HEAD.contains(&r) {
            return Some((token, remain));
        }
    }

    None
}

fn length_two(input: &str) -> Option<(&str, &str)> {
    if input.len() < 2 {
        return None;
    }

    let token = input[..2].split_at(1);
    let remain = &input[2..];

    // "atta" -> あった
    if token.0 == token.1 && ALLOWED_HEAD.contains(&token.0) {
        return Some((LTSU, &input[1..]));
    }

    if ALLOWED_HEAD.contains(&token.0) && ALLOWED_BASE.contains(&token.1) {
        return Some((&input[..2], remain));
    }
    None
}

fn length_three(input: &str) -> Option<(&str, &str)> {
    if input.len() < 3 {
        return None;
    }

    let token = input[..3].split_at(2);
    let remain = &input[3..];

    if ALLOWED_MAP.contains_key(token.0)
        && ALLOWED_MAP
            .get(token.0)
            .expect("no token in map")
            .contains(&token.1)
    {
        return Some((&input[..3], remain));
    }

    None
}

fn length_four(input: &str) -> Option<(&str, &str)> {
    if input.len() < 4 {
        return None;
    }

    let token = &input[..4];
    let remain = &input[4..];

    if token == LTSU {
        return Some((LTSU, remain));
    }

    None
}

lazy_static! {
    static ref ALLOWED_MAP: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("KY", vec!["A", "U", "O"]);
        map.insert("GY", vec!["A", "U", "O"]);
        map.insert("SH", vec!["A", "I", "U", "O"]);
        map.insert("CH", vec!["A", "I", "U", "O"]);
        map.insert("TS", vec!["U"]);
        map.insert("DZ", vec!["U"]);
        map.insert("NY", vec!["A", "U", "O"]);
        map.insert("DY", vec!["A", "U", "O"]);
        map.insert("JY", vec!["A", "U", "O"]);
        map.insert("RY", vec!["A", "U", "O"]);
        map.insert("HY", vec!["A", "U", "O"]);
        map.insert("BY", vec!["A", "U", "O"]);
        map.insert("PY", vec!["A", "U", "O"]);
        map.insert("MY", vec!["A", "U", "O"]);
        map
    };
}
