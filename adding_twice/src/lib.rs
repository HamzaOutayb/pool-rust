pub fn add_curry(n: i128) -> impl Fn(i128) -> i128 {
    move |x: i128| x + n
}


pub fn twice<F>(func: F) -> impl Fn(i128) -> i128
where
    F: Fn(i128) -> i128 + Copy,
{
    move |x: i128| func(func(x))
}
