use std::io;
use std::io::prelude::*;
use clap::{Arg, App};

extern crate kana;


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

    let mut katakana: bool = false;
    match matches.occurrences_of("katakana") {
        1 => katakana = true,
        _ => {}
    }
    
    kana::transform_to_stdout(katakana, &inputs);
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