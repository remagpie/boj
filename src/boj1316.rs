use std::collections::HashSet;
use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let count = lines.next().unwrap()?.parse::<i64>().unwrap();

	let mut result = 0;
	'outer: for line in lines {
		let mut set = HashSet::new();
		let mut last_char = ' ';
		for c in line?.chars() {
			if c != last_char && set.contains(&c) {
				continue 'outer;
			}
			set.insert(c);
			last_char = c;
		}
		result += 1;
	}
	writeln!(out, "{}", result)?;

	Ok(())
}