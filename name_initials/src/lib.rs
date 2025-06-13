pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = Vec::<String>::new();
    for name in names{
        let mut sub = Vec::<String>::new();
        let mut last = ' ';
        for char in name.chars() {
            if last == ' ' {
                sub.push(format!("{}.", char.to_ascii_uppercase()));
            }
            last = char;
        }
        res.push(sub.join(" "))
    }
    res
}