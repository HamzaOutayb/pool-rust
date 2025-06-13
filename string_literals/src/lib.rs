pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    let l = pat.len();
    if l > v.len() {
        return false
    }
    let mut i = 0;
    while i < v.chars().count() {
        if i+l > v.chars().count() {
            return false
        }

        if pat == &v[i..i+l] {
            return true
        }
        i+=1
    }
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    for (i, char) in v.chars().enumerate() {
        if char == pat {
            return i
        }
    }
    0
}