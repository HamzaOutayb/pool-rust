pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
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