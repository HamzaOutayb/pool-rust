use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut h1 = HashMap::new();
    for c in s1.chars() {
        *h1.entry(c).or_insert(0)+= 1
    }

    let mut h2 = HashMap::new();
    for c in s2.chars() {
        *h2.entry(c).or_insert(0)+= 1
    }

    for (key,_) in &h1 {
        if h1.get(&key) != h2.get(&key) {
            return false
        }
    }

    true
}