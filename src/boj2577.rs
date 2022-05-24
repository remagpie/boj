use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let lines = stdin.lock().lines();

	let mut acc = 1;
	for line in lines {
		let value = line.as_ref().unwrap().parse::<usize>().unwrap();
		acc = acc * value;
	}
	let mut count = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
	while acc > 0 {
		count[acc % 10] += 1;
		acc = acc / 10;
	}
	for i in 0..10 {
		writeln!(out, "{}", count[i])?;
	}

	Ok(())
}