use std::collections::BTreeMap;
use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let len = lines.next().unwrap()?.parse::<usize>().unwrap();

	let line = lines.next().unwrap()?;
	let values: Vec<_> = line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
	let mut count = BTreeMap::new();
	for v in &values {
		if !count.contains_key(v) {
			count.insert(v, 0);
		}
		*count.get_mut(v).unwrap() += 1;
	}

	let mut acc = 0;
	let mut count_sum = BTreeMap::new();
	for (k, v) in count {
		count_sum.insert(k, acc);
		acc += v;
	}

	let mut result = Vec::new();
	let mut cache = BTreeMap::new();
	for v in &values {
		if !cache.contains_key(&v) {
			cache.insert(v, *count_sum.get(&v).unwrap());
		}
		result.push(cache.get(v).unwrap().to_string());
		cache.insert(v, cache.get(v).unwrap() + 1);
	}

	write!(out, "{}", result.join(" "))?;

	Ok(())
}