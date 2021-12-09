use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
struct Report {
	n: usize,
	vals: Vec<u16>,
}

impl Report {
	fn from_reader<R: Read>(r: R) -> Result<Report, Box<dyn Error>> {
		let r = BufReader::new(r);

		let mut vals = Vec::new();
		let mut n = 0;
		for line in r.lines() {
			let line = line?;
			n = line.len();
			vals.push(u16::from_str_radix(&line, 2)?);
		}

		Ok(Report { n, vals })
	}

	fn part1(&self) -> usize {
		let mut g = 0;
		let lim = self.vals.len() >> 1;
		for i in 0..self.n {
			let m = 1 << i;
			let mut c = 0;
			for val in &self.vals {
				c += ((val & m) > 0) as usize;
			}
			g |= ((c >= lim) as usize) << i;
		}

		g * (!g & ((1 << self.n) - 1))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day3")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let report = Report::from_reader(File::open(
		matches.value_of("input").unwrap_or("data/day3/input.txt"),
	)?)?;

	println!("Part #1: {}", report.part1());

	Ok(())
}
