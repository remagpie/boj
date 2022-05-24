use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let x = lines.next().unwrap()?.parse::<i64>().unwrap();
	let y = lines.next().unwrap()?.parse::<i64>().unwrap();
	if x > 0 && y > 0 {
		println!("1");
	} else if x < 0 && y > 0 {
		println!("2");
	} else if x < 0 && y < 0 {
		println!("3");
	} else {
		println!("4");
	}

	Ok(())
}