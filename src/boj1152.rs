use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	let line = line.trim();
	writeln!(out, "{}", match line.len() {
		0 => 0,
		_ => line.split(' ').count()
	})?;

	Ok(())
}