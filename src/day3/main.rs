use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
struct Report {
	g: u16,
	e: u16,
}

impl Report {
	fn from_reader<R: Read>(r: R) -> Result<Report, Box<dyn Error>> {
		let r = BufReader::new(r);
		// num_ones will hold the count of ones for each position
		let mut num_ones = Vec::new();
		let mut n = 0;
		for line in r.lines() {
			let line = line?;
			if num_ones.is_empty() {
				num_ones.resize(line.len(), 0);
			}

			let val = u16::from_str_radix(&line, 2)?;
			n += 1;

			for i in 0..num_ones.len() {
				let m = 1 << i;
				num_ones[i] += ((val & m) > 0) as usize;
			}
		}

		let mut g = 0u16;
		let lim = n >> 1;
		for (i, &v) in num_ones.iter().enumerate() {
			g |= ((v >= lim) as u16) << i;
		}

		Ok(Report {
			g,
			e: !g & ((1 << num_ones.len()) - 1),
		})
	}

	fn epsilon(&self) -> usize {
		self.e as usize
	}

	fn gamma(&self) -> usize {
		self.g as usize
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

	println!("Part #1: {}", report.gamma() * report.epsilon());

	Ok(())
}
