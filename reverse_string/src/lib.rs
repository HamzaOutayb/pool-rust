pub fn rev_str(input: &str) -> String {
    let mut res = S
    for c in input.chars().rev() {
        res.push(c);
    }
    res
}