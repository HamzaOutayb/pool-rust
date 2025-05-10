pub fn capitalize_first(input: &str) -> String {
    (input.chars().next().expect("fff").to_ascii_uppercase()).to_string()+&input[1..]
}

pub fn title_case(input: &str) -> String {
    let words: Vec<_> = input.split_whitespace().collect();
    let mut res = String::new();
    for (i, word) in words.into_iter().enumerate() {
        let capitalized = (word.chars().next().expect("fff").to_ascii_uppercase()).to_string()+&(word[1..]).to_string();
        if i != 0 {
            res.push(' ')
        }
        res.push_str(&capitalized)
    }
    res
}

pub fn change_case(input: &str) -> String {
    let mut res: String = String::new();

    for c in input.chars() {
            if c.is_uppercase() {
                res.push(c.to_ascii_lowercase());
            } else if c.is_lowercase() {
                res.push(c.to_ascii_uppercase());
            } else {
                res.push(c);
            }
    }
    res
}