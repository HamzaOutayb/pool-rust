use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut biggest: i32 = 0;
    
    for (_, val) in h {
        if val > biggest {
            biggest = val
        }
    }
    
    biggest
}