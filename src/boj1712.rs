use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	let mut words = line.split(' ');
	let a = words.next().unwrap().parse::<i64>().unwrap();
	let b = words.next().unwrap().parse::<i64>().unwrap();
	let c = words.next().unwrap().parse::<i64>().unwrap();

	let diff = c - b;
	if diff <= 0 {
		writeln!(out, "-1")?;
	} else {
		writeln!(out, "{}", a / diff + 1)?;
	}

	Ok(())
}