use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let base = lines.next().unwrap()?.parse::<i64>().unwrap();
	for i in 1..10 {
		println!("{} * {} = {}", base, i, base * i);
	}

	Ok(())
}