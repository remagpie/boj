use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let mut result = 0;
	for c in lines.next().unwrap()?.chars() {
		result += match c {
			'A' | 'B' | 'C' => 3,
			'D' | 'E' | 'F' => 4,
			'G' | 'H' | 'I' => 5,
			'J' | 'K' | 'L' => 6,
			'M' | 'N' | 'O' => 7,
			'P' | 'Q' | 'R' | 'S' => 8,
			'T' | 'U' | 'V' => 9,
			'W' | 'X' | 'Y' | 'Z' => 10,
			_ => 11,
		}
	}

	writeln!(out, "{}", result)?;

	Ok(())
}