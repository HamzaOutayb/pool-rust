pub fn num_to_ordinal(x: u32) -> String {
    if x % 100 == 11 || x % 100 == 12 || x % 100 == 13 {
        return format!("{}th", x);
    };

    let ordinal = match x % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    format!("{}{}", x, ordinal)
}