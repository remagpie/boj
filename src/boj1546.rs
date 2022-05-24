use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let count = lines.next().unwrap()?.parse::<i64>().unwrap();

	let mut max = 0;
	let mut sum = 0;
	let mut count = 0;
	let line = lines.next().unwrap()?;
	for word in line.split(' ') {
		let value = word.parse::<i64>().unwrap();
		max = std::cmp::max(max, value);
		sum += value;
		count += 1;
	}
	let result = sum as f64 / (max * count) as f64;
	writeln!(out, "{}", result * 100.)?;

	Ok(())
}