use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let year = lines.next().unwrap()?.parse::<i64>().unwrap();
	println!("{}", year - (2541 - 1998));

	Ok(())
}