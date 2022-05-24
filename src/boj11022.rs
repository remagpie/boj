use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let count = lines.next().unwrap()?.parse::<i64>().unwrap();
	for i in 1..=count {
		let line = lines.next().unwrap()?;
		let mut words = line.split(' ');
		let a = words.next().unwrap().parse::<i64>().unwrap();
		let b = words.next().unwrap().parse::<i64>().unwrap();
		writeln!(out, "Case #{}: {} + {} = {}", i, a, b, a + b)?;
	}

	Ok(())
}