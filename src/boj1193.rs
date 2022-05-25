use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let value = lines.next().unwrap()?.parse::<i64>().unwrap();
	let mut sum = 0;
	for i in 0.. {
		let next_sum = sum + i;
		if sum < value && value <= next_sum {
			let numerator = value - sum;
			let denominator = (i + 1) - numerator;
			if i % 2 == 0 {
				writeln!(out, "{}/{}", numerator, denominator)?;
			} else {
				writeln!(out, "{}/{}", denominator, numerator)?;
			}
			break;
		}
		sum = next_sum;
	}

	Ok(())
}