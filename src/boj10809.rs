use std::collections::HashMap;
use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let mut index_map = HashMap::new();
	let line = lines.next().unwrap()?;
	for (i, alphabet) in line.chars().enumerate() {
		if !index_map.contains_key(&alphabet) {
			index_map.insert(alphabet, i);
		}
	}
	for c in 'a'..='z' {
		writeln!(out, "{}", match index_map.get(&c) {
			Some(i) => *i as i64,
			_ => -1,
		})?;
	}

	Ok(())
}