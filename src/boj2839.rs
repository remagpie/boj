use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let mut count_3 = 0;
	let mut n = lines.next().unwrap()?.parse::<i64>().unwrap();
	while n % 5 != 0 {
		count_3 += 1;
		n -= 3;
	}

	if n >= 0 {
		writeln!(out, "{}", n / 5 + count_3)?;
	} else {
		writeln!(out, "-1")?;
	}

	Ok(())
}