// Unsolved

use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let original = lines.next().unwrap()?;
	let count = lines.next().unwrap()?.parse::<usize>().unwrap();

	let mut result: Vec<_> = original.chars().collect();
	let mut cursor = result.len();
	for line in lines {
		let mut chars = line.as_ref().unwrap().chars();
		match chars.next().unwrap() {
			'L' => cursor = std::cmp::max(1, cursor) - 1,
			'D' => cursor = std::cmp::min(result.len(), cursor + 1),
			'B' => if cursor != 0 {
				result.remove(cursor - 1);
				cursor -= 1;
			},
			'P' => {
				result.insert(cursor, chars.skip(1).next().unwrap());
				cursor += 1;
			},
			_ => {},
		}
	}

	write!(out, "{}", result.iter().collect::<String>())?;

	Ok(())
}