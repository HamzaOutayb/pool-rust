use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut letter = HashSet::new();
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            letter.insert(c.to_ascii_lowercase());
        }
    }

    letter.len() == 26
}