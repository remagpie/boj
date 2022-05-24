use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let a = lines.next().unwrap()?.parse::<u32>().unwrap();
	let b_raw = lines.next().unwrap()?;
	let b_digits: Vec<_> = b_raw.chars().map(|c| c.to_digit(10).unwrap()).collect();

	println!("{}", a * b_digits[2]);
	println!("{}", a * b_digits[1]);
	println!("{}", a * b_digits[0]);
	println!("{}", a * (b_digits[0] * 100 + b_digits[1] * 10 + b_digits[2]));

	Ok(())
}