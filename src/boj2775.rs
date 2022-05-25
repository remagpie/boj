use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let count = lines.next().unwrap()?.parse::<i64>().unwrap();
	for _ in 0..count {
		let k = lines.next().unwrap()?.parse::<usize>().unwrap();
		let n = lines.next().unwrap()?.parse::<usize>().unwrap();

		let mut people = vec![vec![0; n]; k + 1];
		for index in 0..n {
			people[0][index] = index + 1;
		}
		for floor in 1..=k {
			people[floor][0] = people[floor - 1][0];
			for index in 1..n {
				people[floor][index] = people[floor][index - 1] + people[floor - 1][index];
			}
		}

		writeln!(out, "{}", people[k][n - 1])?;
	}

	Ok(())
}