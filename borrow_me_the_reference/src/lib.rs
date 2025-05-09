pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    for c in s.chars().rev() {
        if c == '+' {
            result.pop();
            continue
        } else {
            result.push(c)
        }
    }

    let mut final_result = Vec::new();
    for c in result.into_iter().rev(){
        if c == '-' {
            final_result.pop();
            continue
        } else {
            final_result.push(c)
        }
    }
    *s = final_result.iter().collect()
}

pub fn do_operations(v: &mut [String]) {
    for elemn in v {
    if elemn.contains("+") {
       let res: Vec<&str> = elemn.split("+").collect();
       let num1: i32 = res[0].parse().expect("REASON");
       let num2: i32 = res[1].parse().expect("REASON");
       *elemn = (num1 + num2).to_string()
    } else if elemn.contains("-") {
        let res: Vec<&str> = elemn.split("-").collect();
        let num1: i32 = res[0].parse().expect("REASON");
        let num2: i32 = res[1].parse().expect("REASON");
        *elemn = (num1 - num2).to_string()
     }
   }
}