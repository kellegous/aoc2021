use std::collections::{HashMap, HashSet};
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
					card.set(i, j, v);
				}

				line.clear();
			}

			cards.push(card);
		}

		Ok(Input { draws, cards })
	}
}

#[derive(Debug)]
struct Card {
	tiles: [u8; 25],
}

impl Card {
	fn empty() -> Card {
		Card { tiles: [0; 25] }
	}

	fn set(&mut self, i: usize, j: usize, num: u8) {
		self.tiles[i * 5 + j] = num;
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

struct Game<'a> {
	draws: &'a [u8],
	cards: Vec<CardState<'a>>,
}

impl<'a> Game<'a> {
	fn from_input(input: &'a Input) -> Game {
		let cards = input
			.cards
			.iter()
			.map(|c| CardState::new(c))
			.collect::<Vec<CardState>>();
		Game {
			draws: &input.draws,
			cards,
		}
	}

	fn play(&mut self) -> Option<(u8, &CardState)> {
		let mut idx: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
		for (i, card) in self.cards.iter().enumerate() {
			for (j, tile) in card.tiles().iter().enumerate() {
				idx.entry(*tile).or_default().push((i, j));
			}
		}

		for &draw in self.draws {
			for (i, j) in idx.entry(draw).or_default().iter() {
				if self.cards[*i].mark(*j) {
					return Some((draw, &self.cards[*i]));
				}
			}
		}
		None
	}

	fn play_all(&mut self) -> Option<(u8, &CardState)> {
		let mut idx: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
		let mut has_won = HashSet::new();

		for (i, card) in self.cards.iter().enumerate() {
			for (j, tile) in card.tiles().iter().enumerate() {
				idx.entry(*tile).or_default().push((i, j));
			}
		}

		for &draw in self.draws {
			for (i, j) in idx.entry(draw).or_default().iter() {
				if self.cards[*i].mark(*j) {
					has_won.insert(*i);
					if has_won.len() == self.cards.len() {
						return Some((draw, &self.cards[*i]));
					}
				}
			}
		}
		None
	}
}

struct CardState<'a> {
	card: &'a Card,
	marks: [bool; 25],
}

impl<'a> CardState<'a> {
	fn new(card: &'a Card) -> CardState {
		CardState {
			card: card,
			marks: [false; 25],
		}
	}

	fn tiles(&self) -> &[u8] {
		&self.card.tiles
	}

	fn is_winning_row(&self, idx: usize) -> bool {
		let ix = (idx / 5) * 5;
		(ix..ix + 5).find(|&i| !self.marks[i]).is_none()
	}

	fn is_winning_col(&self, idx: usize) -> bool {
		let ix = idx % 5;
		(0..5).find(|&i| !self.marks[i * 5 + ix]).is_none()
	}

	fn mark(&mut self, idx: usize) -> bool {
		self.marks[idx] = true;
		self.is_winning_row(idx) || self.is_winning_col(idx)
	}

	fn sum_unmarked(&self) -> usize {
		self.tiles()
			.iter()
			.zip(&self.marks)
			.filter(|(_, &marked)| !marked)
			.map(|(&n, _)| n as usize)
			.sum()
	}
}

fn part1(input: &Input) -> Result<usize, Box<dyn Error>> {
	let mut game = Game::from_input(input);
	match game.play() {
		Some((draw, card)) => Ok(draw as usize * card.sum_unmarked()),
		None => Err(Box::new(PlayError::from("no winner found"))),
	}
}

fn part2(input: &Input) -> Result<usize, Box<dyn Error>> {
	let mut game = Game::from_input(input);
	match game.play_all() {
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

	let input = Input::from_reader(File::open(
		matches.value_of("input").unwrap_or("data/day4/input.txt"),
	)?)?;

	println!("Part 1: {}", part1(&input)?);
	println!("Part 2: {}", part2(&input)?);
	Ok(())
}
