use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	let mut words = line.split(' ');
	let a = words.next().unwrap().parse::<i64>().unwrap();
	let b = words.next().unwrap().parse::<i64>().unwrap();
	println!("{}", a - b);

	Ok(())
}