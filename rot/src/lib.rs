pub fn rotate(input: &str, key: i8) -> String {
    let mut res: String = String::new();
    let add: u8 = (key%26) as u8;
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            if c.is_ascii_lowercase() {
                let ch = (c as u8)-'a' as u8
                res.push(((ch - 'a' as u8)%26 + 'a' as u8) as char );
            } else if c.is_ascii_uppercase() {
                let ch = (c as u8)-'A' as u8
                res.push(((ch - 'A' as u8)%26 + 'A' as u8) as char );
            }
        }
    }
    res
}