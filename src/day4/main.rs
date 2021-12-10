use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
struct Input {
	draws: Vec<u8>,
	cards: Vec<Card>,
}

impl Input {
	fn from_reader<R>(r: R) -> Result<Input, Box<dyn Error>>
	where
		R: Read,
	{
		let mut r = BufReader::new(r);

		let mut line = String::new();
		if r.read_line(&mut line)? == 0 {
			return Err(Box::new(io::Error::new(
				io::ErrorKind::UnexpectedEof,
				"eof before drawn",
			)));
		}

		let mut draws = Vec::new();
		for num in line.split(",").map(|v| v.trim().parse::<u8>()) {
			draws.push(num?);
		}
		line.clear();

		let mut cards = Vec::new();

		loop {
			if r.read_line(&mut line)? == 0 {
				break;
			}

			if &line != "\n" {
				return Err(Box::new(io::Error::new(
					io::ErrorKind::InvalidData,
					"card must begin with empty line",
				)));
			}
			line.clear();

			let mut card = Card::empty();

			for i in 0..5 {
				if r.read_line(&mut line)? == 0 {
					return Err(Box::new(io::Error::new(
						io::ErrorKind::UnexpectedEof,
						"eof while reading card",
					)));
				}

				for j in 0..5 {
					let offset = j * 3;
					let v = line[offset..offset + 2].trim().parse::<u8>()?;
					card.tiles[i * 5 + j] = Tile::new(v);
				}

				line.clear();
			}

			cards.push(card);
		}

		Ok(Input { draws, cards })
	}

	fn play(&mut self) -> Option<(u8, &Card)> {
		let mut idx: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
		for (i, card) in self.cards.iter().enumerate() {
			for (j, tile) in card.tiles.iter().enumerate() {
				idx.entry(tile.num).or_default().push((i, j));
			}
		}

		for &draw in self.draws.iter() {
			for (i, j) in idx.entry(draw).or_default().iter() {
				if self.cards[*i].mark(*j) {
					return Some((draw, &self.cards[*i]));
				}
			}
		}

		None
	}
}

#[derive(Debug)]
struct Card {
	tiles: [Tile; 25],
}

impl Card {
	fn empty() -> Card {
		Card {
			tiles: [Tile::default(); 25],
		}
	}

	fn has_winning_row(&self, idx: usize) -> bool {
		let ix = idx / 5;
		(ix..ix + 5).find(|&i| !self.tiles[i].has_mark).is_none()
	}

	fn has_winning_col(&self, idx: usize) -> bool {
		let ix = idx % 5;
		(0..5).find(|&i| !self.tiles[i * 5 + ix].has_mark).is_none()
	}

	fn mark(&mut self, idx: usize) -> bool {
		self.tiles[idx].has_mark = true;
		self.has_winning_col(idx) || self.has_winning_row(idx)
	}

	fn sum_unmarked(&self) -> usize {
		self.tiles
			.iter()
			.filter(|t| !t.has_mark)
			.map(|t| t.num as usize)
			.sum()
	}
}

impl std::fmt::Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for row in self.tiles.chunks(5) {
			let s = row
				.iter()
				.map(|t| {
					if t.has_mark {
						format!("[{:2}]", t.num)
					} else {
						format!(" {:2} ", t.num)
					}
				})
				.collect::<Vec<String>>()
				.join(" ");
			writeln!(f, "{}", s)?;
		}
		Ok(())
	}
}

// TODO(knorton): You know what's stupid? Mixing game state with game declaration. That's
// what's stupid. You need to undo that. It's as clear as 1 + 1 = 11.
#[derive(Clone, Copy, Debug)]
struct Tile {
	num: u8,
	has_mark: bool,
}

impl Tile {
	fn new(num: u8) -> Tile {
		Tile {
			num,
			has_mark: false,
		}
	}
}

impl Default for Tile {
	fn default() -> Self {
		Tile::new(0)
	}
}

#[derive(Debug)]
struct PlayError {
	desc: String,
}

impl std::fmt::Display for PlayError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "{}", self.desc)
	}
}

impl From<String> for PlayError {
	fn from(s: String) -> PlayError {
		PlayError { desc: s }
	}
}

impl From<&str> for PlayError {
	fn from(s: &str) -> PlayError {
		PlayError { desc: s.to_owned() }
	}
}

impl Error for PlayError {}

fn part1(input: &mut Input) -> Result<usize, Box<dyn Error>> {
	match input.play() {
		Some((draw, card)) => Ok(draw as usize * card.sum_unmarked()),
		None => Err(Box::new(PlayError::from("no winner found"))),
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day4")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let mut input = Input::from_reader(File::open(
		matches.value_of("input").unwrap_or("data/day4/input.txt"),
	)?)?;

	println!("Part 1: {}", part1(&mut input)?);
	Ok(())
}
