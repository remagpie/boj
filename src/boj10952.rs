use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let lines = stdin.lock().lines();
	for line in lines {
		let mut words = line.as_ref().unwrap().split(' ');
		let a = words.next().unwrap().parse::<i64>().unwrap();
		let b = words.next().unwrap().parse::<i64>().unwrap();
		writeln!(out, "{}", a + b)?;
	}

	Ok(())
}