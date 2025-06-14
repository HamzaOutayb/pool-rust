pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    (input.chars().next().unwrap().to_string().to_uppercase() + &input[1..]).to_string()
}

pub fn title_case(input: &str) -> String {
    let mut res: String = String::new();
    let mut last = ' ';
    for char in input.chars() {
        if last == ' ' || last =='\t' {
            res.push(char.to_ascii_uppercase());
        } else {
            res.push(char);
        }
        last = char
    }
    res
}

pub fn change_case(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
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