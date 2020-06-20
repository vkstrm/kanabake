mod tokens;
mod sets;

/**
 * Turn input into kana and print
 */
pub fn transform(katakana_mode: &bool, input_vec: &Vec<String>) {
    let hiragana_set = sets::hiragana();
    let katakana_set = sets::katakana();
    let mut set: &std::collections::HashMap<&str,&str> = &hiragana_set;
    if *katakana_mode {
        set = &katakana_set;
    }

    for input in input_vec {
        let token_input = tokens::interpret_tokens(&input);
        let mut output = String::with_capacity(token_input.len() * 2);

        for token in token_input {
            output.push_str(set.get(&token.as_str()).expect("No such token in map"));
        }

        println!("{}",output);
    }
}