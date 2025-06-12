pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1
    }

    let mut res: u64 = 1;
    let mut nm = num;
    while nm > 0 {
        res *= nm;
        nm -= 1;
    }
    res
}