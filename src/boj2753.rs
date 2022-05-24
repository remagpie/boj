use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let value = lines.next().unwrap()?.parse::<i64>().unwrap();
	if (value % 4 == 0 && value % 100 != 0) || value % 400 == 0 {
		println!("1");
	} else {
		println!("0");
	}

	Ok(())
}