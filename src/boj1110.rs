use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let value = lines.next().unwrap()?.parse::<u64>().unwrap();
	let mut buffer = value;
	let mut count = 0;
	loop {
		let sum = buffer / 10 + buffer % 10;
		buffer = (buffer % 10) * 10 + sum % 10;

		count += 1;
		if value == buffer {
			break;
		}
	}

	writeln!(out, "{}", count)?;

	Ok(())
}