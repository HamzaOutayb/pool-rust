pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    if let Some(v) = slice.get(index) {
        v
    } else {
        panic!()
    }
}

// fn main() {
//     let mut groceries = vec![
//         "yogurt".to_string(),
//         "panettone".to_string(),
//         "bread".to_string(),
//         "cheese".to_string(),
//     ];
//     insert(&mut groceries, String::from("nuts"));
//     println!("groceries = {:?}", &groceries);
//     println!("groceries[1] = {:?}", at_index(&groceries, 1));
// }