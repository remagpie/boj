// Unsolved

use std::collections::{BTreeMap, VecDeque};
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

	let (n, m, k) = read_words!(lines, usize, usize, usize);

	let source = 2001;
	let dest = 2002;
	let bridge = 2003;
	let mut graph = BTreeMap::new();
	graph.insert(source, BTreeMap::new());
	graph.insert(dest, BTreeMap::new());
	graph.insert(bridge, BTreeMap::new());
	graph.get_mut(&source).unwrap().insert(bridge, k);
	for i in 0..n {
		let employee_vertex = i;
		graph.get_mut(&source).unwrap().insert(employee_vertex, 1);
		graph.get_mut(&bridge).unwrap().insert(employee_vertex, 1);
		graph.insert(employee_vertex, BTreeMap::new());

		let line = read_line!(lines);
		let mut words = line.split(' ');
		let count = words.next().unwrap().parse::<usize>().unwrap();
		for _ in 0..count {
			let work = words.next().unwrap().parse::<usize>().unwrap();
			let work_vertex = 1000 + work;
			graph.get_mut(&employee_vertex).unwrap().insert(work_vertex, 1);
			if !graph.contains_key(&work_vertex) {
				graph.insert(work_vertex, BTreeMap::new());
			}
			graph.get_mut(&work_vertex).unwrap().insert(dest, 1);
		}
	}

	println!("{:?}", graph);

	let mut result = 0;
	let mut residual = graph.clone();
	loop {
		let mut queue = VecDeque::new();
		let mut parent = BTreeMap::new();
		queue.push_front(source);
		'bfs: while let Some(vertex) = queue.pop_back() {
			for (target, capacity) in residual.get(&vertex).unwrap() {
				if parent.contains_key(target) || *target == source {
					continue;
				}
				if *capacity > 0 {
					parent.insert(*target, vertex);
					queue.push_front(*target);
					if *target == dest {
						break 'bfs;
					}
				}
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
			let capacity = subgraph.get(&vertex).unwrap();
			vertex = *parent_vertex;
			flow = std::cmp::min(flow, *capacity);
		}

		let mut vertex = dest;
		while vertex != source {
			let parent_vertex = parent.get(&vertex).unwrap();
			let subgraph = residual.get_mut(parent_vertex).unwrap();
			let capacity = *subgraph.get(&vertex).unwrap();
			subgraph.insert(vertex, capacity - flow);
			vertex = *parent_vertex;
		}

		result += flow;
	}

	writeln!(out, "{}", result)?;

	Ok(())
}