use std::io::{self, BufRead, BufWriter, Write};
use std::iter::Iterator;

macro_rules! read_line {
	($lines: expr) => {$lines.next().unwrap()?};
	($lines: expr, $type: ty) => {$lines.next().unwrap()?.parse::<$type>().unwrap()};
}

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let test_count = read_line!(lines, usize);
	for _ in 0..test_count {
		let line = read_line!(lines);
		let mut counter = 0;
		for c in line.chars() {
			match c {
				'(' => counter += 1,
				')' => {
					counter -= 1;
					if counter < 0 {
						break;
					}
				},
				_ => {},
			}
		}
		writeln!(out, "{}", match counter == 0 {
			true => "YES",
			false => "NO",
		})?;
	}

	Ok(())
}