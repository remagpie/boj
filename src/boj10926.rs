use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	println!("{}??!", line);

	Ok(())
}