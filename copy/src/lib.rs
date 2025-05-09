use std::f64::consts::E;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let log: f64 = c as f64;
    let exp: f64 = c as f64;
    let l: f64 = log.abs().log(E);
    let e: f64 = exp.exp();
    return (c, e, l)
}

pub fn str_function(a: String) -> (String, String) {
    let mut res: Vec<String>  = Vec::new();
        for nm in a.split(" ") {
        res.push(nm.parse::<f64>().unwrap().exp().to_string());
    }

    (a, res.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res: Vec<f64> = Vec::new();
    for nm in &b {
        res.push((nm.abs() as f64 ).log(E));
    }
    (b, res)
}