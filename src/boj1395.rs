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

pub struct LazySegmentNode {
	start: usize,
	middle: usize,
	end: usize,
	value: usize,
	left: Option<Box<LazySegmentNode>>,
	left_pending: bool,
	right: Option<Box<LazySegmentNode>>,
	right_pending: bool,
}

impl LazySegmentNode {
	pub fn new(start: usize, end: usize, value: usize) -> Self {
		let middle = (start + end) / 2;
		Self {
			start,
			end,
			middle,
			value,
			left: None,
			left_pending: false,
			right: None,
			right_pending: false,
		}
	}

	pub fn update(&mut self, start: usize, end: usize) {
		if self.start == start && self.end == end {
			self.left_pending = !self.left_pending;
			self.right_pending = !self.right_pending;
			self.value = (self.end - self.start) - self.value;
			return;
		}

		if start < self.middle {
			if self.left.is_none() {
				self.left = Some(Box::new(LazySegmentNode::new(self.start, self.middle, 0)));
			}
			let left = self.left.as_mut().unwrap();
			if self.left_pending {
				left.update(left.start, left.end);
				self.left_pending = false;
			}
			let new_start = std::cmp::max(self.start, start);
			let new_end = std::cmp::min(self.middle, end);
			left.update(new_start, new_end);
		}
		if self.middle < end {
			if self.right.is_none() {
				self.right = Some(Box::new(LazySegmentNode::new(self.middle, self.end, 0)));
			}
			let right = self.right.as_mut().unwrap();
			if self.right_pending {
				right.update(right.start, right.end);
				self.right_pending = false;
			}
			let new_start = std::cmp::max(self.middle, start);
			let new_end = std::cmp::min(self.end, end);
			right.update(new_start, new_end);
		}

		let mut result = 0;
		result += match (&self.left, self.left_pending) {
			(Some(node), true) => (self.middle - self.start) - node.value,
			(Some(node), false) => node.value,
			(None, true) => self.middle - self.start,
			(None, false) => 0,
		};
		result += match (&self.right, self.right_pending) {
			(Some(node), true) => (self.end - self.middle) - node.value,
			(Some(node), false) => node.value,
			(None, true) => self.end - self.middle,
			(None, false) => 0,
		};
		self.value = result;
	}

	pub fn query(&self, start: usize, end: usize) -> usize {
		if self.start == start && self.end == end {
			return self.value;
		}

		let mut result = 0;
		if start < self.middle {
			let new_start = std::cmp::max(self.start, start);
			let new_end = std::cmp::min(self.middle, end);
			let left_result = match &self.left {
				Some(node) => node.query(new_start, new_end),
				None => 0,
			};
			result += match self.left_pending {
				true => (new_end - new_start) - left_result,
				false => left_result,
			};
		}
		if self.middle < end {
			let new_start = std::cmp::max(self.middle, start);
			let new_end = std::cmp::min(self.end, end);
			let right_result = match &self.right {
				Some(node) => node.query(new_start, new_end),
				None => 0,
			};
			result += match self.right_pending {
				true => (new_end - new_start) - right_result,
				false => right_result,
			};
		}

		result
	}
}


pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let (n, m) = read_words!(lines, usize, usize);
	let mut tree = LazySegmentNode::new(0, n, 0);
	for _ in 0..m {
		let (action, start, end) = read_words!(lines, usize, usize, usize);
		match action {
			0 => tree.update(start - 1, end),
			1 => writeln!(out, "{}", tree.query(start - 1, end))?,
			_ => {},
		}
	}

	Ok(())
}