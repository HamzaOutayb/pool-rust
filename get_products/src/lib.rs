pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return Vec::new();
    }
    arr.iter()
        .enumerate()
        .map(|(i, _)| {
            arr.iter()
            .enumerate()
            .filter(|(j,_)| *j != i)
            .map(|(_,&v)|v)
            .product()
        })
        .collect()
}