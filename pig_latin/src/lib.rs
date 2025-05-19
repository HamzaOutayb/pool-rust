pub fn pig_latin(text: &str) -> String {
    let vawols = "aeiou";
    if !vawols.contains(text.chars().next().unwrap()) && text[1..3] == "qu".to_string() {
        return text[3..].to_string() + &text[..3].to_string()+ "ay";
    }

    for (i, c) in text.chars().enumerate() {
        if "aeiou".contains(c) {
            return text[i..].to_string() + &text[..i].to_string() + "ay";
        }
    }
    text.to_string()
}
