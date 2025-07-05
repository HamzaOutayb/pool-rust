use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T: Eq + Hash, U>(a: &'a [T], b: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut l: usize = 0;
    if a.len() > b.len() {
        l = b.len();
    } else {
        l = a.len();
    }

    let mut res = HashMap::new();

    for i in 0..l {
        res.insert(&a[i], &b[i]);
    }

    res
}
