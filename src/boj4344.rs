use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();
	let count = lines.next().unwrap()?.parse::<i64>().unwrap();

	for line in lines {
		let mut words = line.as_ref().unwrap().split(' ');
		let word_count = words.next().unwrap().parse::<usize>().unwrap();
		let values: Vec<_> = words.map(|w| w.parse::<usize>().unwrap()).collect();
		let sum = values.iter().sum();
		let count = values.iter().filter(|v| *v * values.len() > sum).count();

		writeln!(out, "{:.3}%", count as f64 / values.len() as f64 * 100.)?;
	}

	Ok(())
}