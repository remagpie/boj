use std::collections::HashMap;
use std::io::{self, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());

	let mut reverse_map = HashMap::new();
	for i in 1..=10000 {
		if !reverse_map.contains_key(&i) {
			writeln!(out, "{}", i)?;
		}

		let mut next_value = i;
		let mut buffer = i;
		while buffer > 0 {
			next_value += buffer % 10;
			buffer = buffer / 10;
		}
		reverse_map.insert(next_value, i);
	}

	Ok(())
}