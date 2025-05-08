pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1
    }

    let mut res: u64 = 1;
    let mut nm = num;
    while nm > 0 {
        res *= nm;
        nm -= 1;
    }
    res
}

// fn main() {
//     println!("The factorial of 0 = {}", factorial(0));
//     println!("The factorial of 1 = {}", factorial(1));
//     println!("The factorial of 5 = {}", factorial(5));
//     println!("The factorial of 10 = {}", factorial(10));
//     println!("The factorial of 19 = {}", factorial(19));
// }