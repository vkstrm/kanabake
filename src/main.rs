use std::env;
mod sets;
mod tokens;

struct Args {
    input: String,
    katakana: bool
}

fn main() {
    // TODO Proper argument parsing
    // TODO STDIN input
    let args: Vec<_> = env::args().collect();
    let p_args = check_args(&args);
    let parsed_args: Args;
    if let Some(p) = p_args {
        parsed_args = p;
    } else {
        println!("usuage: kana <sentence>
        Transform letters to hiragana (default) or katakana (with flag -k)");
        return
    }

    let hiragana_set = sets::hiragana();
    let katakana_set = sets::katakana();
    let mut set: &std::collections::HashMap<&str,&str> = &hiragana_set;
    if parsed_args.katakana {
        set = &katakana_set;
    }

    let input_str: &String = &parsed_args.input;
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
fn check_args(args: &Vec<String>) -> std::option::Option<Args> {
    let mut k_mode = false;
    let k_flag_short = String::from("-k");
    let k_flag_long = String::from("--katakana");

    if args.len() == 1 { return None; }

    if args.contains(&k_flag_short) || args.contains(&k_flag_long) {
        k_mode = true;
    }

    // TODO Check with regex if input is OK
    
    let input_str: &String = &args[1];
    if !input_str.is_ascii() {
        println!("No weird letters please");
        return None
    }

    let args = Args{
        katakana: k_mode,
        input: input_str.to_owned()
    };

    Some(args)
}
