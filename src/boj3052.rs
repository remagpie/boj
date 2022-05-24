use std::collections::HashSet;
use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let lines = stdin.lock().lines();

	let mut set = HashSet::new();
	for line in lines {
		let value = line.as_ref().unwrap().parse::<usize>().unwrap();
		set.insert(value % 42);
	}
	writeln!(out, "{}", set.len())?;

	Ok(())
}