pub fn arrange_phrase(phrase: &str) -> String {
    let mut vec = vec!["".to_string(); phrase.split_whitespace().count()];
    for words in phrase.split_whitespace() {
        let mut index: usize = 0; 
        let mut word: String = String::new();
        for c in words.chars() {
            if c.is_ascii_digit() {
                index = c.to_string().parse().unwrap();
            } else {
                word.push(c)
            }
        }
        vec[index-1] = word
    }
    vec.join(" ")
}