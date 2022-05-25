use std::io::{self, BufRead, BufWriter, Write};

const STEP: usize = 15;

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let line = lines.next().unwrap()?;
	let mut words = line.split(' ');
	let a_raw = words.next().unwrap();
	let b_raw = words.next().unwrap();

	let mut a_arr = Vec::new();
	let mut b_arr = Vec::new();
	for i in (0..a_raw.len()).step_by(STEP) {
		let start = std::cmp::max(a_raw.len(), i + STEP) - (i + STEP);
		let substr = &a_raw[start..(a_raw.len() - i)];
		a_arr.push(substr.parse::<u64>().unwrap());
	}
	for i in (0..b_raw.len()).step_by(STEP) {
		let start = std::cmp::max(b_raw.len(), i + STEP) - (i + STEP);
		let substr = &b_raw[start..(b_raw.len() - i)];
		b_arr.push(substr.parse::<u64>().unwrap());
	}

	let mut result = Vec::new();
	let mut carry = 0;
	for i in 0..std::cmp::max(a_arr.len(), b_arr.len()) {
		let a = a_arr.get(i).unwrap_or(&0);
		let b = b_arr.get(i).unwrap_or(&0);
		let sum = a + b + carry;
		result.push(sum % (10u64.pow(STEP as u32)));
		carry = sum / 10u64.pow(STEP as u32);
	}
	let mut result_iter = result.iter().rev();
	write!(out, "{}", result_iter.next().unwrap())?;
	for part in result_iter {
		write!(out, "{:015}", part)?;
	}

	Ok(())
}