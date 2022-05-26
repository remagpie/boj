use std::io::{self, BufRead, BufWriter, Write};
use std::iter::Iterator;

macro_rules! read_line {
	($lines: expr) => {$lines.next().unwrap()?};
	($lines: expr, $type: ty) => {$lines.next().unwrap()?.parse::<$type>().unwrap()};
}

macro_rules! read_words {
	($lines: expr, $( $type: ty ),*) => {{
		let line = read_line!($lines);
		let mut words = line.split(' ');
		( $(words.next().unwrap().parse::<$type>().unwrap(), )*)
	}};
}

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let test_count = read_line!(lines, usize);
	for _ in 0..test_count {
		let (m, n, k) = read_words!(lines, usize, usize, usize);
		let mut farm = vec![vec![0; m]; n];
		for _ in 0..k {
			let (x, y) = read_words!(lines, usize, usize);
			farm[y][x] = 1;
		}

		let mut marker = 2;
		for y in 0..n {
			for x in 0..m {
				if farm[y][x] != 1 {
					continue;
				}

				let mut stack = Vec::new();
				stack.push((x, y));
				while let Some((cursor_x, cursor_y)) = stack.pop() {
					farm[cursor_y][cursor_x] = marker;
					if cursor_x > 0 && farm[cursor_y][cursor_x - 1] == 1 {
						stack.push((cursor_x - 1, cursor_y));
					}
					if cursor_x < m - 1 && farm[cursor_y][cursor_x + 1] == 1 {
						stack.push((cursor_x + 1, cursor_y));
					}
					if cursor_y > 0 && farm[cursor_y - 1][cursor_x] == 1 {
						stack.push((cursor_x, cursor_y - 1));
					}
					if cursor_y < n - 1 && farm[cursor_y + 1][cursor_x] == 1 {
						stack.push((cursor_x, cursor_y + 1));
					}
				}
				marker += 1;
			}
		}
		writeln!(out, "{}", marker - 2)?;
	}

	Ok(())
}