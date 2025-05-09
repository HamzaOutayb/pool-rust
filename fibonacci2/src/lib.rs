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