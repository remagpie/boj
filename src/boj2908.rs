use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	let mut words = line.split(' ');
	let left_raw = words.next().unwrap();
	let right_raw = words.next().unwrap();
	let left = left_raw.chars().rev().collect::<String>().parse::<u64>().unwrap();
	let right = right_raw.chars().rev().collect::<String>().parse::<u64>().unwrap();

	writeln!(out, "{}", std::cmp::max(left, right))?;

	Ok(())
}