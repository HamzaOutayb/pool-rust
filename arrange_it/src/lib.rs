pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: Vec<String> = vec!["".to_string(); phrase.split_whitespace().count()];
    for word in phrase.split_whitespace() {
        let mut index: usize = 0;
        let mut w: String = String::new();
        for c in word.chars() {
            if c.is_ascii_digit() {
                index = c.to_string().parse().unwrap();
            } else {
                w.push(c)
            }
        }
        res[index-1] = w
    }
    res.join(" ")
}