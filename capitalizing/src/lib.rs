pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    (input.chars().next().expect("fff").to_ascii_uppercase()).to_string()+&input[1..]
}

pub fn title_case(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    let mut res: String = String::new();
    let mut lastc = ' ';
    for c in input.chars() {
        if lastc == ' ' || lastc == '\t' {
            res.push(c.to_ascii_uppercase());
        } else{
            res.push(c);
        }
        lastc = c
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