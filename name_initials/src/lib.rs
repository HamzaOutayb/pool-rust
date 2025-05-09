pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = Vec::<String>::new();
    for name in names{
        let mut sub = Vec::<String>::new();
        for e in name.split(" ") {
                sub.push(e.chars().next().expect("dd").to_string() + ".")
        }
        res.push(sub.join(" "))
    }
    res
}