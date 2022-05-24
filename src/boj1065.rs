use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let limit = lines.next().unwrap()?.parse::<i64>().unwrap();

	let mut result = 0;
	'outer: for i in 1..=limit {
		let diff = (i / 10) % 10 - i % 10;
		let mut buffer = i / 10;
		while buffer >= 10 {
			if (buffer / 10) % 10 - buffer % 10 != diff {
				continue 'outer;
			}
			buffer = buffer / 10;
		}
		result += 1;
	}
	writeln!(out, "{}", result)?;

	Ok(())
}