pub fn number_logic(num: u32) -> bool {
    if num == 0 {return true}
    let pow = num.to_string();
    let mut res: u32 = 0;
    for c in pow.chars() {
        res += (c.to_string().parse::<u32>().unwrap()).pow(pow.len() as u32)
    }
    res == num
}