pub fn fibonacci(n: u32) -> u32 {
let mut a: u32=0;
let mut b: u32=1;
let mut i = 0;

    while i < n {
        let  c = a;
        a = b;
        b += c;

        i += 1;
    }
    a
}

// fn main() {
//     println!(
//         "The element in the position {} in fibonacci series is {}",
//         2,
//         fibonacci(2)
//     );
//     println!(
//         "The element in the position {} in fibonacci series is {}",
//         4,
//         fibonacci(4)
//     );
//     println!(
//         "The element in the position {} in fibonacci series is {}",
//         22,
//         fibonacci(22)
//     );
//     println!(
//         "The element in the position {} in fibonacci series is {}",
//         20,
//         fibonacci(20)
//     );
// }