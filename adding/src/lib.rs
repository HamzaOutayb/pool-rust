pub fn add_curry(n: i128) -> impl Fn(i128) -> i128 {
    move |x: i128| x + n
}
