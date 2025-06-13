pub fn initials(names: Vec<&str>) -> Vec<String> {
 let mut res: Vec<String> = Vec::new();

    for name in names {
        let mut s: Vec<String> = Vec::new();
        for e in name.split(" ") {
            s.push(e.chars().next().expect("dd").to_string()+".");
        }
        res.push(s.join(" "))
    }
    res
}