pub fn stars(n: u32) -> String {
    let mut res: String = String::new();
    let l: u32 = 2u32.pow(n);
    for _ in 0..l {
        res += "*";
    }
    res
}