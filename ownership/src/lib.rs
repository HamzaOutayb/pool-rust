pub fn first_subword(mut s: String) -> String {
    let mut res = String::new();
    for (i, c) in s.chars().enumerate() {
        if i == 0 || c != '_' && !c.is_uppercase() {
            res.push(c)
        } else {
            return res
        }
    }
    res
}