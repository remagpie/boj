use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	let mut counter = HashMap::new();
	for digit in line.split(' ') {
		if !counter.contains_key(digit) {
			counter.insert(digit, 0);
		}
		*counter.get_mut(digit).unwrap() += 1;
	}
	let max_count = counter.values().max().unwrap();

	let mut result = 0;
	for (digit, count) in &counter {
		result = std::cmp::max(result, match (max_count, count) {
			(3, 3) => 10000 + digit.parse::<i32>().unwrap() * 1000,
			(2, 2) => 1000 + digit.parse::<i32>().unwrap() * 100,
			(1, 1) => digit.parse::<i32>().unwrap() * 100,
			_ => 0,
		});
	}

	println!("{}", result);

	Ok(())
}