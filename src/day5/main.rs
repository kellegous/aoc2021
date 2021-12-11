use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pt {
	x: isize,
	y: isize,
}

impl Pt {
	fn new(x: isize, y: isize) -> Pt {
		Pt { x, y }
	}
}

struct Pts {
	fr: Pt,
	to: Pt,
	done: bool,
}

fn step(fr: isize, to: isize) -> isize {
	if fr == to {
		0
	} else if fr > to {
		-1
	} else {
		1
	}
}

impl Iterator for Pts {
	type Item = Pt;

	fn next(&mut self) -> Option<Pt> {
		if self.done {
			None
		} else {
			let Pt { x: xa, y: ya } = self.fr;
			let Pt { x: xb, y: yb } = self.to;
			self.done = &self.to == &self.fr;
			self.fr = Pt {
				x: xa + step(xa, xb),
				y: ya + step(ya, yb),
			};
			Some(Pt { x: xa, y: ya })
		}
	}
}

#[derive(Debug)]
struct Line {
	fr: Pt,
	to: Pt,
}

impl Line {
	fn new(fr: Pt, to: Pt) -> Line {
		Line { fr, to }
	}

	fn is_parallel_to_axis(&self) -> bool {
		self.fr.x == self.to.x || self.fr.y == self.to.y
	}

	fn points(&self) -> Pts {
		Pts {
			fr: self.fr,
			to: self.to,
			done: false,
		}
	}
}

fn bisect<'a>(s: &'a str, sep: &str) -> Option<(&'a str, &'a str)> {
	s.find(sep).map(|ix| (&s[..ix], &s[ix + sep.len()..]))
}

impl FromStr for Line {
	type Err = Box<dyn Error>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (fr, to) = match bisect(s, " -> ") {
			Some((a, b)) => (a, b),
			None => return Err(format!("invalid line: {}", s).into()),
		};

		let fr = match bisect(fr, ",") {
			Some((x, y)) => Pt::new(x.parse::<isize>()?, y.parse::<isize>()?),
			None => return Err(format!("invalid pt: {}", fr).into()),
		};

		let to = match bisect(to, ",") {
			Some((x, y)) => Pt::new(x.parse::<isize>()?, y.parse::<isize>()?),
			None => return Err(format!("invalid pt: {}", to).into()),
		};

		Ok(Line::new(fr, to))
	}
}

fn lines_from_reader<R: io::Read>(r: R) -> Result<Vec<Line>, Box<dyn Error>> {
	let r = io::BufReader::new(r);
	let mut lines = Vec::new();
	for line in r.lines() {
		lines.push(Line::from_str(&line?)?);
	}
	Ok(lines)
}

fn intersections_of<'a, I>(lines: I) -> HashSet<Pt>
where
	I: Iterator<Item = &'a Line>,
{
	let mut points = HashSet::new();
	let mut intersections = HashSet::new();
	for line in lines {
		for pt in line.points() {
			if !points.insert(pt) {
				intersections.insert(pt);
			}
		}
	}
	intersections
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day5")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let lines = lines_from_reader(File::open(
		matches.value_of("input").unwrap_or("data/day5/input.txt"),
	)?)?;

	println!(
		"Part 1: {}",
		intersections_of(lines.iter().filter(|l| l.is_parallel_to_axis())).len()
	);
	println!("Part 2: {}", intersections_of(lines.iter()).len());
	Ok(())
}
