use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
struct Report {
	n: usize,
	vals: Vec<u16>,
}

fn segment(vals: &[u16], m: u16) -> (Vec<u16>, Vec<u16>) {
	let mut a = Vec::new();
	let mut b = Vec::new();
	for val in vals {
		if (val & m) != 0 {
			a.push(*val);
		} else {
			b.push(*val);
		}
	}
	(a, b)
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

	fn part2(&self) -> usize {
		let mask = 1 << (self.n - 1);
		let (mut a, mut b) = segment(&self.vals, mask);

		let mut m = mask >> 1;
		while a.len() > 1 {
			let (aa, bb) = segment(&a, m);
			a = if aa.len() >= bb.len() { aa } else { bb };
			m >>= 1;
		}

		let mut m = mask >> 1;
		while b.len() > 1 {
			let (aa, bb) = segment(&b, m);
			b = if aa.len() < bb.len() { aa } else { bb };
			m >>= 1;
		}

		a[0] as usize * b[0] as usize
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
	println!("Part #2: {}", report.part2());

	Ok(())
}
