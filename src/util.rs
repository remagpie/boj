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

pub struct LazySegmentTree<T, A> {
	default: T,
	root: Box<LazySegmentNode<T, A>>,
	apply: Box<dyn Fn (&T, &A) -> T>,
}

pub struct LazySegmentNode<T, A> {
	start: usize,
	middle: usize,
	end: usize,
	value: T,
	left: Option<Box<LazySegmentNode<T, A>>>,
	left_pending: Vec<A>,
	right: Option<Box<LazySegmentNode<T, A>>>,
	right_pending: Vec<A>,
}

impl<T: Clone, A: Clone> LazySegmentTree<T, A> {
	pub fn new(
		start: usize,
		end: usize,
		default: T,
		apply: Box<dyn Fn (&T, &A) -> T>,
	) -> Self {
		Self {
			default: default.clone(),
			root: Box::new(LazySegmentNode::new(start, end, default)),
			apply,
		}
	}
}

impl <T: Clone, A: Clone> LazySegmentNode<T, A> {
	pub fn new(start: usize, end: usize, value: T) -> Self {
		let middle = (start + end) / 2;
		Self {
			start,
			end,
			middle,
			value,
			left: None,
			left_pending: Vec::new(),
			right: None,
			right_pending: Vec::new(),
		}
	}

	pub fn update(&mut self, tree: &LazySegmentTree<T, A>, start: usize, end: usize, arguments: &[A]) {
		if self.start == start && self.end == end {
			self.left_pending.extend(arguments.iter().cloned());
			for argument in arguments {
				self.value = (&tree.apply)(&self.value, argument);
			}
			return;
		}

		if start < self.middle {
			if self.left.is_none() {
				self.left = Some(Box::new(LazySegmentNode::new(self.start, self.middle, tree.default.clone())));
			}
			let left = self.left.as_mut().unwrap();
			if !self.left_pending.is_empty() {
				left.update(tree, start, self.middle, &self.left_pending);
				self.left_pending = Vec::new();
			}
			left.update(tree, start, self.middle, arguments);
		}
		if self.middle < end {
			if self.right.is_none() {
				self.right = Some(Box::new(LazySegmentNode::new(self.start, self.middle, tree.default.clone())));
			}
			let right = self.right.as_mut().unwrap();
			if !self.right_pending.is_empty() {
				right.update(tree, start, self.middle, &self.right_pending);
				self.right_pending = Vec::new();
			}
			right.update(tree, self.middle, end, arguments);
		}
	}
}
