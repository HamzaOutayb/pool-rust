pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = Vec::<String>::new();
    for name in names{
        let mut sub = Vec::<String>::new();
        for e in name.split_whitespace() {
            if let Some(first_charachter) = e.chars().next() {
                sub.push(first_charachter.to_string()+".")
            }
        }
        res.push(sub.join(" "))
    }
    res
}