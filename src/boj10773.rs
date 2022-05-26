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

	let mut stack = Vec::new();
	for _ in 0..test_count {
		match read_line!(lines, usize) {
			0 => { stack.pop(); },
			v => stack.push(v),
		}
	}
	writeln!(out, "{}", stack.iter().sum::<usize>())?;

	Ok(())
}