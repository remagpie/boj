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
		let line = read_line!(lines);
		let mut words = line.split(' ');
		match words.next().unwrap() {
			"push" => stack.push(words.next().unwrap().parse::<i64>().unwrap()),
			"pop" => match stack.pop() {
				Some(v) => writeln!(out, "{}", v)?,
				None => writeln!(out, "-1")?,
			},
			"size" => writeln!(out, "{}", stack.len())?,
			"empty" => writeln!(out, "{}", if stack.is_empty() { 1 } else { 0 })?,
			"top" => match stack.last() {
				Some(v) => writeln!(out, "{}", v)?,
				None => writeln!(out, "-1")?,
			},
			_ => {},
		}
	}

	Ok(())
}