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

fn find_nearest_distance(points: &[(i32, i32)]) -> i32 {
	let len = points.len();
	if len <= 3 {
		let mut result = i32::MAX;
		for i in 0..(len - 1) {
			for j in (i + 1)..len {
				let distance =
					(points[i].0 - points[j].0) * (points[i].0 - points[j].0) +
					(points[i].1 - points[j].1) * (points[i].1 - points[j].1);
				result = std::cmp::min(result, distance);
			}
		}
		return result;
	}

	let middle = len / 2;
	let left_min = find_nearest_distance(&points[0..middle]);
	let right_min = find_nearest_distance(&points[middle..len]);
	let mut result = std::cmp::min(left_min, right_min);

	let mut strip_start = middle;
	while strip_start > 0 && (points[middle].0 - points[strip_start].0) < result {
		strip_start -= 1;
	}
	let mut strip_end = middle;
	while strip_end < len && (points[strip_end].0 - points[middle].0) < result {
		strip_end += 1;
	}
	let mut strip = points[strip_start..strip_end].to_owned();
	strip.sort_unstable_by_key(|(_, y)| *y);
	if strip.len() > 1 {
		for i in 0..(strip.len() - 1) {
			for j in (i + 1)..strip.len() {
				if (strip[j].1 - strip[i].1) >= result {
					break;
				}
				let distance =
					(strip[i].0 - strip[j].0) * (strip[i].0 - strip[j].0) +
					(strip[i].1 - strip[j].1) * (strip[i].1 - strip[j].1);
				result = std::cmp::min(result, distance);
			}
		}
	}

	result
}

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let count = read_line!(lines, usize);
	let mut points = Vec::new();
	for _ in 0..count {
		let (x, y) = read_words!(lines, i32, i32);
		points.push((x, y));
	}
	points.sort_unstable_by_key(|(x, _)| *x);
	let result = find_nearest_distance(&points);
	writeln!(out, "{}", result)?;

	Ok(())
}