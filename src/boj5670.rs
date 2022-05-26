use std::collections::HashMap;
use std::io::{self, BufRead, BufWriter, Write};
use std::iter::Iterator;

macro_rules! read_line {
	($lines: expr) => {$lines.next().unwrap()?};
	($lines: expr, $type: ty) => {$lines.next().unwrap()?.parse::<$type>().unwrap()};
}

struct TrieNode {
	child: HashMap<char, TrieNode>,
}

impl TrieNode {
	pub fn new() -> Self {
		Self { child: HashMap::new() }
	}

	pub fn insert(&mut self, values: &[char]) {
		if let Some(value) = values.first() {
			if !self.child.contains_key(value) {
				self.child.insert(*value, TrieNode::new());
			}
			self.child.get_mut(value).unwrap().insert(&values[1..]);
		} else {
			self.child.insert(' ', TrieNode::new());
		}
	}

	pub fn sum_depth(&self) -> (usize, usize) {
		let mut sum = 0;
		let mut leaves = 0;
		for (character, node) in &self.child {
			match character {
				' ' => {
					leaves += 1;
					sum += 1;
				},
				_ => {
					let (child_sum, child_leaves) = node.sum_depth();
					leaves += child_leaves;
					sum += child_sum + match self.child.len() {
						1 => 0,
						_ => child_leaves,
					};
				},
			};
		}

		(sum, leaves)
	}
}

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines().peekable();

	while lines.peek().is_some() {
		let n = read_line!(lines, usize);
		let mut root = TrieNode::new();
		for _ in 0..n {
			let line = read_line!(lines);
			let chars: Vec<_> = line.chars().collect();
			root.insert(&chars);
		}
		// root.print(0);
		let (mut sum, _) = root.sum_depth();
		if root.child.len() > 1 {
			sum -= n;
		}
		writeln!(out, "{:.2}", sum as f64 / n as f64)?;
	}

	Ok(())
}