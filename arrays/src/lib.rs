pub fn sum(a: &[i32]) -> i32 {
    let mut res: i32 = 0;
    for nm in a {
       res += nm;
    }
    res
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}