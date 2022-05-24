use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let count = lines.next().unwrap()?.parse::<i64>().unwrap();

	for line in lines {
		let mut result = 0;
		let mut acc = 0;
		for correct in line.as_ref().unwrap().chars() {
			match correct {
				'O' => {
					acc += 1;
					result += acc;
				},
				'X' => acc = 0,
				_ => {},
			}
		}

		writeln!(out, "{}", result)?;
	}

	Ok(())
}