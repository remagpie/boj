use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let count = lines.next().unwrap()?.parse::<i64>().unwrap();

	let mut result = 0;
	let line = lines.next().unwrap()?;
	for digit in line.chars() {
		result += digit.to_digit(10).unwrap();
	}
	writeln!(out, "{}", result)?;

	Ok(())
}