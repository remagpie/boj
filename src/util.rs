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