pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i, el) in array.iter().enumerate() {
        match *el == key {
            true => return Some(i),
            false => continue,
        }
    }
    None
}