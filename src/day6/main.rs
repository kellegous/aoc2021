use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct School {
	// circular buffer with counts per population
	generations: [usize; 9],
	// current time
	t: usize,
}

impl School {
	fn pop(&mut self) -> usize {
		let n = self.generations[self.t % 9];
		self.t += 1;
		n
	}

	fn entry(&mut self, ix: usize) -> &mut usize {
		let idx = (ix + self.t) % 9;
		&mut self.generations[idx]
	}

	fn simulate_until(&mut self, t: usize) -> usize {
		while self.t < t {
			let n = self.pop();
			*self.entry(6) += n;
		}
		self.generations.iter().sum()
	}
}

impl FromStr for School {
	type Err = Box<dyn Error>;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut generations = [0; 9];
		for v in s.split(",") {
			generations[v.parse::<usize>()?] += 1;
		}
		Ok(School { generations, t: 0 })
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day6")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let mut school = read_to_string(matches.value_of("input").unwrap_or("data/day6/input.txt"))?
		.parse::<School>()?;
	println!("Part 1: {}", school.simulate_until(80));
	println!("Part 2: {}", school.simulate_until(256));
	Ok(())
}
