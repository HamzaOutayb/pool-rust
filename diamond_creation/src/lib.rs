pub fn get_diamond(c: char) -> Vec<String> {
    let mut index: u8 = c as u8 - b'A';
    let mut res = vec![];
    let mut center_s: usize = 0;
    for i in (0..=index).rev() {
        let mut sub = String::new();
        for _ in 0..i {
            sub.push(' ');
        }
        sub.push(((b'A'+index) as u8 - i) as char);

        if center_s != 0 {
            sub.push_str(&" ".repeat(center_s));
        }

        let mut rev: String = sub.clone();
        rev.pop();
        rev = rev.chars().rev().collect();
        sub.push_str(&rev);
        res.push(sub);
       center_s += 1;
    }
    let mut a = res.clone();
    a.pop();
    a.reverse();
    res.extend(a);
    res
}