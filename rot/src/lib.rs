pub fn rotate(input: &str, key: i8) -> String {
    let mut res: String = String::new();
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            if c.is_ascii_lowercase() {
                let ch = c as u8 - b'a';
                if key < 0 {
                    res.push((((ch as i8+ key)%26 )as u8 + b'a') as char );
                } else {
                    res.push(((((ch as i8+ key)%26 )%26) as u8 + b'a') as char );
                }
            } else if c.is_ascii_uppercase() {
                let ch = c as u8 - b'A';
                res.push(((((ch as i8+ key)%26+26 )%26) as u8 + b'A') as char );
            }
        } else {
            res.push(c)
        }
    }
    res
 }