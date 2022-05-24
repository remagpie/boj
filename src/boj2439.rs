use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let value = lines.next().unwrap()?.parse::<usize>().unwrap();
	for i in 1..=value {
		writeln!(out, "{}{}", " ".repeat(value - i), "*".repeat(i))?;
	}

	Ok(())
}