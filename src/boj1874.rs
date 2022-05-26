use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let count = lines.next().unwrap()?.parse::<usize>().unwrap();

	let mut result = Vec::new();
	let mut stack = Vec::new();
	let mut max = 0;
	'outer: for line in lines {
		let value = line.unwrap().parse::<u64>().unwrap();
		if value > max {
			for i in (max + 1)..=value {
				stack.push(i);
				result.push('+');
			}
			max = value;
		}
		while let Some(top) = stack.pop() {
			result.push('-');
			if top == value {
				continue 'outer;
			}
		}

		
		writeln!(out, "NO")?;
		return Ok(());
	}

	for action in result {
		writeln!(out, "{}", action)?;
	}

	Ok(())
}