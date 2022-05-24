use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let value = lines.next().unwrap()?.parse::<i64>().unwrap();
	if value < 60 {
		println!("F");
	} else if value < 70 {
		println!("D");
	} else if value < 80 {
		println!("C");
	} else if value < 90 {
		println!("B");
	} else {
		println!("A");
	}

	Ok(())
}