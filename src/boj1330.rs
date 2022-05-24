use std::cmp::Ordering;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let line = lines.next().unwrap()?;
	let mut words = line.split(' ');
	let a = words.next().unwrap().parse::<i64>().unwrap();
	let b = words.next().unwrap().parse::<i64>().unwrap();
	match a.cmp(&b) {
		Ordering::Greater => println!(">"),
		Ordering::Less => println!("<"),
		Ordering::Equal => println!("=="),
	}

	Ok(())
}