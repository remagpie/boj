use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let lines = stdin.lock().lines();

	for line in lines {
		let line = line.unwrap();
		if line == "0" {
			break;
		}
		let mut values = vec![0];
		for word in line.split(' ').skip(1) {
			values.push(word.parse::<usize>().unwrap());
		}
		values.push(0);
		let mut right_low = vec![0; values.len()];
		let mut stack = Vec::new();
		for (i, value) in values.iter().enumerate() {
			while let Some((top_index, top_value)) = stack.last() {
				if value >= top_value {
					break;
				}
				right_low[*top_index] = i;
				stack.pop();
			}
			stack.push((i, *value));
		}
		let mut left_low = vec![0; values.len()];
		let mut stack = Vec::new();
		for (i, value) in values.iter().enumerate().rev() {
			while let Some((top_index, top_value)) = stack.last() {
				if value >= top_value {
					break;
				}
				left_low[*top_index] = i;
				stack.pop();
			}
			stack.push((i, *value));
		}
		let mut area = Vec::new();
		for (i, value) in values.iter().enumerate() {
			if *value > 0 {
				area.push(value * (right_low[i] - left_low[i] - 1));
			} else {
				area.push(0);
			}
		}

		writeln!(out, "{}", area.iter().max().unwrap())?;
	}

	Ok(())
}