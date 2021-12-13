use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Entry {
	patterns: Vec<String>,
	output: Vec<String>,
}

impl FromStr for Entry {
	type Err = Box<dyn Error>;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (l, r) = match s.split_once(" | ") {
			Some(p) => p,
			None => return Err(format!("invalid entry: {}", s).into()),
		};

		Ok(Entry {
			patterns: l.split_whitespace().map(|s| s.to_owned()).collect(),
			output: r.split_whitespace().map(|s| s.to_owned()).collect(),
		})
	}
}

fn read_entries<'a, R: io::Read>(r: R) -> Result<Vec<Entry>, Box<dyn Error>> {
	let r = BufReader::new(r);
	r.lines().map(|line| line?.parse::<Entry>()).collect()
}

fn part1(entries: &[Entry]) -> usize {
	entries.iter().fold(0, |s, e| {
		s + e
			.output
			.iter()
			.map(|s| match s.len() {
				2 | 3 | 4 | 7 => 1,
				_ => 0,
			})
			.sum::<usize>()
	})
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day08")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let entries = read_entries(File::open(
		matches.value_of("input").unwrap_or("data/day08/input.txt"),
	)?)?;

	println!("Part 1: {}", part1(&entries));

	Ok(())
}
