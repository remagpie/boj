use std::io::{self, BufRead, BufWriter, Write};

pub fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
	let mut out = BufWriter::new(stdout.lock());
	let mut lines = stdin.lock().lines();

	let line = lines.next().unwrap()?;
	let line = line.replace("c=", "0");
	let line = line.replace("c-", "1");
	let line = line.replace("dz=", "2");
	let line = line.replace("d-", "3");
	let line = line.replace("lj", "4");
	let line = line.replace("nj", "5");
	let line = line.replace("s=", "6");
	let line = line.replace("z=", "7");
	writeln!(out, "{}", line.len())?;

	Ok(())
}