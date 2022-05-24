use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let count = lines.next().unwrap()?.parse::<i64>().unwrap();

	let mut max = -1000000;
	let mut min = 1000000;
	let line = lines.next().unwrap()?;
	for word in line.split(' ') {
		let value = word.parse::<i64>().unwrap();
		max = std::cmp::max(max, value);
		min = std::cmp::min(min, value);
	}
	writeln!(out, "{} {}", min, max)?;

	Ok(())
}