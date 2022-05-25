use std::collections::HashMap;
use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let mut count = HashMap::new();
	for c in lines.next().unwrap()?.chars() {
		let uppercase = c.to_ascii_uppercase();
		if !count.contains_key(&uppercase) {
			count.insert(uppercase, 0);
		}
		*count.get_mut(&uppercase).unwrap() += 1;
	}
	let mut entries: Vec<_> = count.iter().collect();
	entries.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));
	let (first_char, first_count) = entries[0];
	if entries.len() > 1 && entries[1].1 == first_count {
		writeln!(out, "?")?;
	} else {
		writeln!(out, "{}", first_char)?;
	}

	Ok(())
}