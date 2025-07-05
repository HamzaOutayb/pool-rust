pub fn first_fifty_even_square() -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut i = 1i32;
    loop {
        if res.len() == 50 {
            return res
        }
        if i%2 == 0 {
            res.push(i*i);
        }
        i += 1;
    }
}