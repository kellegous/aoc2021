use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
struct Pt(i32, i32);

impl Pt {
	fn product(&self) -> i32 {
		self.0 * self.1
	}
}

#[derive(Debug)]
struct PtWithAim(i32, i32, i32);

impl PtWithAim {
	fn product(&self) -> i32 {
		self.0 * self.1
	}
}

#[derive(Debug)]
enum Command {
	Vertical(i32),
	Horizontal(i32),
}

impl Command {
	fn parse(s: &str) -> Result<Command, Box<dyn Error>> {
		if s.starts_with("forward") {
			Ok(Command::Horizontal(s[8..].parse::<i32>()?))
		} else if s.starts_with("down") {
			Ok(Command::Vertical(s[5..].parse::<i32>()?))
		} else if s.starts_with("up") {
			Ok(Command::Vertical(-s[3..].parse::<i32>()?))
		} else {
			Err(Box::new(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("invalid command: {}", s),
			)))
		}
	}

	fn apply(&self, pt: &Pt) -> Pt {
		match self {
			Command::Vertical(y) => Pt(pt.0, pt.1 + y),
			Command::Horizontal(x) => Pt(pt.0 + x, pt.1),
		}
	}

	fn apply_with_aim(&self, pt: &PtWithAim) -> PtWithAim {
		match self {
			Command::Vertical(y) => PtWithAim(pt.0, pt.1, pt.2 + y),
			Command::Horizontal(x) => PtWithAim(pt.0 + x, pt.1 + pt.2 * x, pt.2),
		}
	}
}

fn read_input<R: Read>(r: R) -> Result<Vec<Command>, Box<dyn Error>> {
	let r = BufReader::new(r);
	let mut cmds = Vec::new();
	for line in r.lines() {
		cmds.push(Command::parse(&line?)?);
	}
	Ok(cmds)
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day2")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let commands = read_input(File::open(
		matches.value_of("input").unwrap_or("data/day2/input.txt"),
	)?)?;

	println!(
		"Part 1: {}",
		commands
			.iter()
			.fold(Pt(0, 0), |loc, cmd| cmd.apply(&loc))
			.product()
	);

	println!(
		"Part 2: {}",
		commands
			.iter()
			.fold(PtWithAim(0, 0, 0), |loc, cmd| cmd.apply_with_aim(&loc))
			.product()
	);

	Ok(())
}
