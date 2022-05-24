use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	let mut words = line.split(' ');
	let n = words.next().unwrap().parse::<i64>().unwrap();
	let x = words.next().unwrap().parse::<i64>().unwrap();

	let mut result = Vec::new();
	let line = lines.next().unwrap()?;
	for word in line.split(' ') {
		let digit = word.parse::<i64>().unwrap();
		if digit < x {
			result.push(digit.to_string());
		}
	}
	writeln!(out, "{} ", result.join(" "))?;

	Ok(())
}