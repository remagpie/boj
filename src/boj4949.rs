use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let lines = stdin.lock().lines();

	'case: for line in lines {
		let line = line.unwrap();
		if line == ".".to_owned() {
			break;
		}

		let mut stack = Vec::new();
		for c in line.chars() {
			match c {
				'(' => stack.push('('),
				')' => match stack.last() {
					Some('(') => {stack.pop();},
					_ => {
						writeln!(out, "no")?;
						continue 'case;
					},
				}
				'[' => stack.push('['),
				']' => match stack.last() {
					Some('[') => {stack.pop();},
					_ => {
						writeln!(out, "no")?;
						continue 'case;
					},
				}
				_ => {},
			}
		};
		writeln!(out, "{}", match stack.is_empty() {
			true => "yes",
			false => "no",
		})?;
	}

	Ok(())
}