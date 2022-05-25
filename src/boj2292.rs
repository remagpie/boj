use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let value = lines.next().unwrap()?.parse::<i64>().unwrap();
	if value == 1 {
		writeln!(out, "1")?;
	} else {
		let mut buffer = 2;
		for i in 1.. {
			let next_buffer = buffer + 6 * i;
			if buffer <= value && value < next_buffer {
				writeln!(out, "{}", i + 1)?;
				break;
			}
			buffer = next_buffer;
		}
	}

	Ok(())
}