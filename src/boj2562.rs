use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let lines = stdin.lock().lines();

	let mut max = 0;
	let mut index = 0;
	for (i, line) in lines.enumerate() {
		let value = line.as_ref().unwrap().parse::<i64>().unwrap();
		if value > max {
			max = value;
			index = i;
		}
	}
	writeln!(out, "{}", max)?;
	writeln!(out, "{}", index + 1)?;

	Ok(())
}