use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut avg: f64 = 0.0;
    for v in list {
        avg += *v as f64
    }
    avg/list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut vec = list.to_vec();
    vec.sort();
    let mid = vec.len()/2;
    if vec.len()%2 == 0 {
        return (vec[mid-1]+vec[mid])/2
    }
    vec[mid]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counter = HashMap::new();
    for v in list {
        *counter.entry(v).or_insert(0) += 1
    }

    let mut res: i32 = 0;
    let mut valindx = 0;
    for (k,val) in counter {
        if val > valindx {
            println!("key: {}, val: {}", k, val);
            res = *k;
            valindx = val;
        }
    }
   res
}