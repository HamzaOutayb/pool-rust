use roman_numbers_iter::RomanNumber;

fn main() {
	let mut number = RomanNumber::from(49);

	println!("{:?}", number);
	println!("{:?}", number.next());
}