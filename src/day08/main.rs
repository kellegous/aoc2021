use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

const A: Pattern = Pattern { signals: 0x01 };
const B: Pattern = Pattern { signals: 0x02 };
const C: Pattern = Pattern { signals: 0x04 };
const D: Pattern = Pattern { signals: 0x08 };
const E: Pattern = Pattern { signals: 0x10 };
const F: Pattern = Pattern { signals: 0x20 };
const G: Pattern = Pattern { signals: 0x40 };

// used for counting bits in bitset
const BIT_COUNTS: &'static [usize] = &[0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

#[derive(Debug, Copy, Clone)]
struct Pattern {
	signals: u8,
}

impl Pattern {
	fn from_char(c: char) -> Result<Pattern, Box<dyn Error>> {
		match c {
			'a' => Ok(A),
			'b' => Ok(B),
			'c' => Ok(C),
			'd' => Ok(D),
			'e' => Ok(E),
			'f' => Ok(F),
			'g' => Ok(G),
			_ => Err(format!("invalid signal: {}", c).into()),
		}
	}

	fn from_str(s: &str) -> Result<Pattern, Box<dyn Error>> {
		let mut p = Pattern::empty();
		for c in s.chars() {
			p = p.union(Pattern::from_char(c)?);
		}
		Ok(p)
	}

	fn empty() -> Pattern {
		Pattern { signals: 0 }
	}

	fn union(&self, p: Pattern) -> Pattern {
		Pattern {
			signals: self.signals | p.signals,
		}
	}

	fn intersection(&self, p: Pattern) -> Pattern {
		Pattern {
			signals: self.signals & p.signals,
		}
	}

	fn to_string(&self) -> String {
		let mut s = String::new();
		if !self.intersection(A).is_empty() {
			s.push('a');
		}
		if !self.intersection(B).is_empty() {
			s.push('b');
		}
		if !self.intersection(C).is_empty() {
			s.push('c');
		}
		if !self.intersection(D).is_empty() {
			s.push('d');
		}
		if !self.intersection(E).is_empty() {
			s.push('e');
		}
		if !self.intersection(F).is_empty() {
			s.push('f');
		}
		if !self.intersection(G).is_empty() {
			s.push('g');
		}
		s
	}

	fn len(&self) -> usize {
		BIT_COUNTS[self.signals as usize & 0xf] + BIT_COUNTS[(self.signals >> 4) as usize & 0xf]
	}

	fn is_empty(&self) -> bool {
		self.signals == 0
	}
}

impl std::fmt::Display for Pattern {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

#[derive(Debug)]
struct Entry {
	patterns: Vec<Pattern>,
	output: Vec<Pattern>,
}

impl FromStr for Entry {
	type Err = Box<dyn Error>;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (l, r) = match s.split_once(" | ") {
			Some(p) => p,
			None => return Err(format!("invalid entry: {}", s).into()),
		};

		Ok(Entry {
			patterns: l
				.split_whitespace()
				.map(|s| Pattern::from_str(s))
				.collect::<Result<Vec<_>, _>>()?,
			output: r
				.split_whitespace()
				.map(|s| Pattern::from_str(s))
				.collect::<Result<Vec<_>, _>>()?,
		})
	}
}

impl std::fmt::Display for Entry {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for (i, p) in self.patterns.iter().enumerate() {
			if i == 0 {
				write!(f, "{}", p)?;
			} else {
				write!(f, " {}", p)?;
			}
		}
		write!(f, " | ")?;
		for (i, p) in self.output.iter().enumerate() {
			if i == 0 {
				write!(f, "{}", p)?;
			} else {
				write!(f, " {}", p)?;
			}
		}

		Ok(())
	}
}

impl Entry {
	fn get_decoder(&self) -> Decoder {
		let s1 = self.patterns.iter().find(|s| s.len() == 2).unwrap();
		let s4 = self.patterns.iter().find(|s| s.len() == 4).unwrap();
		Decoder { s1, s4 }
	}
}

#[derive(Debug)]
struct Decoder<'a> {
	// we only need to know the set of segments for 1 and 4 to deduce everything else
	s1: &'a Pattern,
	s4: &'a Pattern,
}

impl<'a> Decoder<'a> {
	fn decode(&self, p: Pattern) -> u8 {
		match p.len() {
			2 => 1,
			3 => 7,
			4 => 4,
			7 => 8,
			6 => {
				// patterns with 6 signals include 6, 0 and 9.
				// 6 only shares 1 signal with 1
				// 0 shares 3 segments with 4
				// 9 shares 4 segments with 4
				if self.s1.intersection(p).len() == 1 {
					6
				} else if self.s4.intersection(p).len() == 3 {
					0
				} else {
					9
				}
			}
			_ => {
				// patterns with 5 siganls include 3, 2, and 5
				// 3 shares 2 signals with 1
				// 2 shares 2 signals with 4
				// 5 shares 3 signals with 4
				if self.s1.intersection(p).len() == 2 {
					3
				} else if self.s4.intersection(p).len() == 2 {
					2
				} else {
					5
				}
			}
		}
	}
}

fn read_entries<'a, R: io::Read>(r: R) -> Result<Vec<Entry>, Box<dyn Error>> {
	let r = BufReader::new(r);
	r.lines().map(|line| line?.parse::<Entry>()).collect()
}

fn part1(entries: &[Entry]) -> usize {
	entries
		.iter()
		.map(|e| {
			e.output
				.iter()
				.map(|s| match s.len() {
					2 | 3 | 4 | 7 => 1,
					_ => 0,
				})
				.sum::<usize>()
		})
		.sum::<usize>()
}

fn part2(entries: &[Entry]) -> usize {
	entries
		.iter()
		.map(|e| {
			let decoder = e.get_decoder();
			e.output
				.iter()
				.enumerate()
				.map(|(i, &p)| (10_usize).pow(3 - i as u32) * decoder.decode(p) as usize)
				.sum::<usize>()
		})
		.sum()
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
	println!("Part 2: {}", part2(&entries));

	Ok(())
}
