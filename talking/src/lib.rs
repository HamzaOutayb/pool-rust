pub fn talking(text: &str) -> &str {
    if text.trim() == "" {
        return "Just say something!"
    }
    let mut is_uppercase = false;
    for c in text.chars() {
        if c.is_ascii_alphabetic() && c.is_ascii_uppercase(){
                is_uppercase = true;
        } else if c.is_ascii_alphabetic() && c.is_ascii_lowercase() {
            is_uppercase = false;
            break
        }
    }

     if is_uppercase && &text[text.len()-1..text.len()] == "?" {
        return "Quiet, I am thinking!"
    }else if !is_uppercase && &text[text.len()-1..] == "?"{
        return "Sure."
    }  else if is_uppercase{
        return "There is no need to yell, calm down!"
    } else {
        return "Interesting"
    }
}