pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut v: Vec<u32> = vec![];
    let mut mult = false;
    for mut n in s.split_whitespace() {
        mult = false;
        if n.ends_with("k") {
            mult = true;
            n = &n[..n.len()-1];
        };

        let mut num = match n.parse::<f64>() {
            Ok(n) => n,
            Err(_) => 0.0,
        };

        if mult {num *= 1000.0};

        v.push(num as u32);
    }


    Box::new(v)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
