use std::collections::HashMap;

type OptStrTuple = std::option::Option<(String, String)>;

pub fn interpret_tokens(input: &String) -> Vec<String> {
    let allowed_map = get_allowed_map();    
    let allowed_head = vec!["K","G","S","Z","T","D","J","N","H","B","P","F","M","Y","R","W"];
    let allowed_base = vec!["A","I","U","E","O"];
    let mut tokens: Vec<String> = Vec::new();

    let mut remain = input.to_owned();
    let mut opt_token = search_for_token(&remain, &allowed_map, &allowed_head, &allowed_base);
    while opt_token.is_some() {
        let token = opt_token.expect("Token was None, despite this being impossible!");
        tokens.push(token.0);
        remain = token.1;
        opt_token = search_for_token(&remain, &allowed_map, &allowed_head, &allowed_base);
    }

    tokens
}

fn search_for_token(
        remain: &String, 
        allowed_map: &HashMap<&str, Vec<&str>>, 
        allowed_head: &Vec<&str>, 
        allowed_base: &Vec<&str>) -> OptStrTuple {

    let one = length_one(&remain, &allowed_head, &allowed_base);
    if let Some(one) = one {
        return Some(one)
    }

    let two = length_two(&remain, &allowed_head, &allowed_base);
    if let Some(two) = two {
        return Some(two)
    }

    let three = length_three(&remain, &allowed_map);
    if let Some(three) = three {
        return Some(three)
    }
    None
}

fn get_allowed_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
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
}

/**
 * Split token to str slices at index, return as tuple.
 */
fn token_to_tuple(token: &str, split_index: usize) -> (&str,&str) {
    token.split_at(split_index)
}

/**
 * Create the return value for the token searching functions
 */
fn as_opt_str_tuple(token: &str, remain: &str) -> OptStrTuple {
    Some((String::from(token), String::from(remain)))
}

/**
 * Search for Tokens of length one.
 * Return a found token and a splice of remaining input string.
 */
fn length_one(input: &str, allowed_post_n: &Vec<&str>, allowed_base: &Vec<&str>) -> OptStrTuple {
    if input.len() < 1 {
        return None
    }

    let token = &input[..1];
    let remain = &input[1..];

    if allowed_base.contains(&token) {
        return as_opt_str_tuple(token,remain);
    }
    if token == "N" {
        if remain.len() < 1 {
            return as_opt_str_tuple(token, remain);
        }
        let r = &remain[..1];
        if allowed_post_n.contains(&r) {
            return as_opt_str_tuple(token,remain);
        }
    }
    None
}

/**
 * Search for Tokens of length two.
 * Return a found token and a splice of remaining input string.
 */
fn length_two(input: &str, allowed_head: &Vec<&str>, allowed_tail: &Vec<&str>) -> OptStrTuple {
    if input.len() < 2 {
        return None
    }

    let token = token_to_tuple(&input[..2], 1);
    let remain = &input[2..];

    if &token.0 == &token.1 && allowed_head.contains(&token.0)  {
        return as_opt_str_tuple("LTSU", &input[1..])
    }

    if allowed_head.contains(&token.0) && allowed_tail.contains(&token.1) {
        return as_opt_str_tuple(&input[..2], remain)
    }
    None
}

/**
 * Search for Tokens of length three.
 * Return a found token and a splice of remaining input string.
 */
fn length_three(input: &str, allowed_map: &HashMap<&str, Vec<&str>>) -> OptStrTuple {
    if input.len() < 3 {
        return None
    }

    let token = token_to_tuple(&input[..3], 2);
    let remain = &input[3..];

    if allowed_map.contains_key(token.0) 
        && allowed_map.get(token.0).expect("Oops").contains(&token.1) {
            return as_opt_str_tuple(&input[..3], remain);
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert_eq!(1, 1);
    }
}