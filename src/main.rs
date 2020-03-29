use std::env;
mod sets;
mod tokens;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("usuage: kana <sentence>
        Transform letters to hiragana (default) or katakana (with flag -k)");
        return
    }

    // TODO Command to print less clear part of character table
    // and what turns into what

    let hiragana_set = sets::hiragana();
    let katakana_set = sets::katakana();
    let mut set: &std::collections::HashMap<&str,&str> = &hiragana_set;

    let arg_res = check_args(&args);
    if let Some(a) = arg_res {
        if a { set = &katakana_set; }
    }

    let input_str: &String = &args[1];
    let input = input_str.to_ascii_uppercase();
    transform(&set, &input);
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
 * Check if arguments are good and what flag and whatnot.
 * Maybe unnessecary now but good sometime.
 */
fn check_args(args: &Vec<String>) -> std::option::Option<bool> {
    let mut k_mode = false;
    let k_flag_short = String::from("-k");
    let k_flag_long = String::from("--katakana");

    if args.contains(&k_flag_short) || args.contains(&k_flag_long) {
        k_mode = true;
    }

    // TODO Check with regex if input is OK
    
    let input_str: &String = &args[1];
    if !input_str.is_ascii() {
        println!("No weird letters please");
        return None
    }

    Some(k_mode)
}
