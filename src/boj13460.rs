use std::collections::{VecDeque, HashSet};
use std::io::{self, BufRead, BufWriter, Write};

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

struct BallMap {
	pub map: Vec<Vec<bool>>,
}

impl BallMap {
	pub fn new(width: usize, height: usize) -> BallMap {
		BallMap {
			map: vec![vec![false; width]; height],
		}
	}

	pub fn get(&self, position: (usize, usize)) -> bool {
		let (x, y) = position;
		self.map[y][x]
	}
}

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let (n, m) = read_words!(lines, usize, usize);
	let mut map = BallMap::new(m, n);
	let mut red_start = (0, 0);
	let mut blue_start = (0, 0);
	let mut dest = (0, 0);
	for y in 0..n {
		for (x, c) in read_line!(lines).chars().enumerate() {
			match c {
				'#' => map.map[y][x] = true,
				'R' => red_start = (x, y),
				'B' => blue_start = (x, y),
				'O' => dest = (x, y),
				_ => {},
			}
		}
	}

	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();
	queue.push_front((red_start, blue_start, 0));
	visited.insert((red_start, blue_start));
	let mut result = -1;
	'bfs: while let Some((red, blue, count)) = queue.pop_back() {
		'movement: for movement in [
			|(x, y)| (x - 1, y),
			|(x, y)| (x + 1, y),
			|(x, y)| (x, y + 1),
			|(x, y)| (x, y - 1),
		] {
			let mut finished = false;
			let mut new_red = red;
			let mut new_blue = blue;
			loop {
				let moved_red = movement(new_red);
				let moved_blue = movement(new_blue);
				let real_red = match map.get(moved_red) || moved_red == new_blue {
					true => new_red,
					false => moved_red,
				};
				let real_blue = match map.get(moved_blue) || (!finished && moved_blue == new_red) {
					true => new_blue,
					false => moved_blue,
				};

				if real_red == new_red && real_blue == new_blue {
					break;
				}

				if moved_blue == dest {
					continue 'movement;
				}
				if moved_red == dest {
					finished = true;
				}
				new_red = real_red;
				new_blue = real_blue;
			}
			if finished {
				result = count + 1;
				break 'bfs;
			}
			if count < 9 && !visited.contains(&(new_red, new_blue)) {
				queue.push_front((new_red, new_blue, count + 1));
			}
		}
	}

	writeln!(out, "{}", result)?;

	Ok(())
}