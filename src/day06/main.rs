use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct School {
	// circular buffer with counts per generation
	generations: [usize; 9],
	// current time
	t: usize,
}

impl School {
	fn simulate_until(&mut self, t: usize) -> usize {
		while self.t < t {
			let spawning_gen = self.t % 9;
			let spawned_into_gen = (self.t + 7) % 9;
			self.generations[spawned_into_gen] += self.generations[spawning_gen];
			self.t += 1;
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
	let matches = clap::App::new("day06")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let mut school = read_to_string(matches.value_of("input").unwrap_or("data/day06/input.txt"))?
		.parse::<School>()?;
	println!("Part 1: {}", school.simulate_until(80));
	println!("Part 2: {}", school.simulate_until(256));
	Ok(())
}
