use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	let mut words = line.split(' ');
	let h = words.next().unwrap().parse::<i64>().unwrap();
	let m = words.next().unwrap().parse::<i64>().unwrap();
	let merged = h * 60 + m;
	let shifted = ((merged - 45) + 24 * 60) % (24 * 60);

	println!("{} {}", shifted / 60, shifted % 60);

	Ok(())
}