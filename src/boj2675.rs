use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let count = lines.next().unwrap()?.parse::<i64>().unwrap();
	for line in lines {
		let mut words = line.as_ref().unwrap().split(' ');
		let repeat = words.next().unwrap().parse::<usize>().unwrap();
		let target = words.next().unwrap();
		for char in target.chars() {
			write!(out, "{}", char.to_string().repeat(repeat))?;
		}
		writeln!(out, "")?;
	}

	Ok(())
}