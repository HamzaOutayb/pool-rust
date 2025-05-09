use  std::io;
fn main() {
    let text = String::from("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?\n");
    print!("{}", text);
    let mut trials = 1;
    loop {
        let mut input = String::new();
       let _ = io::stdin().read_line(&mut input);
        if input.trim() == "The letter e" {
            println!("Number of trials: {}", trials);
            break
        } else {
            trials += 1;
            print!("{}", text);
        }
    }
}