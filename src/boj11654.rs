use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let c = lines.next().unwrap()?.chars().next().unwrap();
	writeln!(out, "{}", c as u32)?;

	Ok(())
}