use std::io;
use std::io::prelude::*;
use clap::{Arg, App};

mod sets;
mod tokens;


fn main() {
    let matches = App::new("kana")
        .version("1.0")
        .about("Turn letters into kana")
        .arg(
            Arg::with_name("input")
                .about("The word to transform")
                .required(true)
                .index(1)
                .conflicts_with("stdin")
        )
        .arg(
            Arg::with_name("stdin")
                .short('s')
                .long("stdin")
                .about("Use stdin for input")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("katakana")
                .short('k')
                .long("katakana")
                .about("Output as katakana")
                .takes_value(false)
        )
        .get_matches();

    let mut inputs: Vec<String> = Vec::new();
    if let Some(i) = matches.value_of("input") {
        inputs.push(String::from(i).to_ascii_uppercase());
    }

    match matches.occurrences_of("stdin") {
        1 => from_stdin(&mut inputs),
        _ => {} 
    }

    // validate_input(&inputs);
    
    let hiragana_set = sets::hiragana();
    let katakana_set = sets::katakana();
    let mut set: &std::collections::HashMap<&str,&str> = &hiragana_set;
    match matches.occurrences_of("katakana") {
        1 => set = &katakana_set,
        _ => {}
    }

    for input in inputs {
        transform(&set, &input);
    }
}

/**
 * Turn input into kana and print
 */
fn transform(set: &std::collections::HashMap<&str, &str>, input: &String) {
    let token_input = tokens::intepret_tokens(&input);
    let mut output = String::with_capacity(token_input.len() * 2);

    for token in token_input {
        output.push_str(set.get(&token.as_str()).expect("No such token in map"));
    }

    println!("{}",output);
}

/**
 * Collect input from standard in
 */
fn from_stdin(input_vec: &mut Vec<String>) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input_vec.push(line.unwrap().to_ascii_uppercase());
    }
}

// /**
//  * Check that no input string contains any illegal characters
//  */
// fn validate_input(input: & Vec<String>) {
//     return
// }