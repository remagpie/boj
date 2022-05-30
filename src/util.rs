use std::collections::{HashMap, VecDeque};

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

pub struct FlowGraph<T> {
	vertex: HashMap<usize, T>,
	edge: HashMap<usize, HashMap<usize, usize>>,
}

impl<T> FlowGraph<T> {
	pub fn new() -> Self {
		Self {
			vertex: HashMap::new(),
			edge: HashMap::new(),
		}
	}

	pub fn insert_vertex(&mut self, index: usize, value: T) {
		self.vertex.insert(index, value);
	}

	pub fn insert_edge(&mut self, source: usize, dest: usize, flow: usize) {
		if !self.edge.contains_key(&source) {
			self.edge.insert(source, HashMap::new());
		}
		self.edge.get_mut(&source).unwrap().insert(dest, flow);
	}

	pub fn find_flow(&self, source: usize, dest: usize) -> usize {
		let mut result = 0;
		let mut residual = self.edge.clone();
		loop {
			let mut queue = VecDeque::new();
			let mut parent = HashMap::new();
			queue.push_front(source);
			while let Some(vertex) = queue.pop_back() {
				if !residual.contains_key(&vertex) {
					continue;
				}

				for (target, flow) in residual.get(&vertex).unwrap() {
					if parent.contains_key(target) || *target == source {
						continue;
					}
					if *flow > 0 {
						parent.insert(*target, vertex);
						queue.push_front(*target);
					}
				}

				if parent.contains_key(&dest) {
					break;
				}
			}

			if !parent.contains_key(&dest) {
				break;
			}

			let mut vertex = dest;
			let mut flow = usize::MAX;
			while vertex != source {
				let parent_vertex = parent.get(&vertex).unwrap();
				let subgraph = residual.get(parent_vertex).unwrap();
				let remaining_flow = subgraph.get(&vertex).unwrap();
				vertex = *parent_vertex;
				flow = std::cmp::min(flow, *remaining_flow);
			}

			let mut vertex = dest;
			while vertex != source {
				let parent_vertex = parent.get(&vertex).unwrap();
				let subgraph = residual.get_mut(parent_vertex).unwrap();
				let remaining_flow = *subgraph.get(&vertex).unwrap();
				subgraph.insert(vertex, remaining_flow - flow);
				vertex = *parent_vertex;
			}

			result += flow;
		}

		result
	}
}