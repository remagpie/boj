use std::io::{self, BufRead, BufWriter, Write};

macro_rules! read_line {
	($lines: expr) => {$lines.next().unwrap()?};
	($lines: expr, $type: ty) => {$lines.next().unwrap()?.parse::<$type>().unwrap()};
}

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let len = read_line!(lines, usize);
	let line = read_line!(lines);
	let mut result = vec!["-1".to_owned(); len];
	let mut stack = Vec::new();
	for (i, word) in line.split(' ').enumerate() {
		let value = word.parse::<usize>().unwrap();
		while let Some((top_index, top_value)) = stack.last() {
			if value <= *top_value {
				break;
			}
			result[*top_index] = value.to_string();
			stack.pop();
		}
		stack.push((i, value));
	};
	writeln!(out, "{}", result.join(" "))?;

	Ok(())
}