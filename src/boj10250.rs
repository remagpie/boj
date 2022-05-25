use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let count = lines.next().unwrap()?.parse::<i64>().unwrap();
	for line in lines {
		let mut words = line.as_ref().unwrap().split(' ');
		let h = words.next().unwrap().parse::<i64>().unwrap();
		let w = words.next().unwrap().parse::<i64>().unwrap();
		let n = words.next().unwrap().parse::<i64>().unwrap();

		let floor = (n - 1) % h + 1;
		let index = (n - 1) / h;
		writeln!(out, "{}{:02}", floor, index + 1)?;
	}

	Ok(())
}