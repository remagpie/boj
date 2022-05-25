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
	let v = words.next().unwrap().parse::<i64>().unwrap();

	for i in ((v - a) / (a - b)).. {
		let sum = (a - b) * (i - 1) + a;
		if v <= sum {
			writeln!(out, "{}", i)?;
			break;
		}
	}

	Ok(())
}