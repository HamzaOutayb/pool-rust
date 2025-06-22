pub fn number_logic(num: u32) -> bool {
    if num == 0 {return true}

    let s_numbers = num.to_string();
    let mut res: u32 = 0;
    for c in s_numbers.chars() {
       let n = (c.to_digit(10).expect("")).pow(s_numbers.len() as u32);
        res += n;
    }
    res == num
}