use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(nums: &[i32]) -> i32 {
	nums.windows(2)
		.fold(0, |c, p| if p[0] < p[1] { c + 1 } else { c })
}

fn part2(nums: &[i32]) -> i32 {
	let agg: Vec<i32> = nums.windows(3).map(|vals| vals.iter().sum()).collect();
	part1(&agg)
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day1")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let src = matches.value_of("input").unwrap_or("day1/input.txt");
	let mut nums = Vec::new();
	for line in BufReader::new(File::open(&src)?).lines() {
		nums.push(line?.parse::<i32>()?);
	}
	println!("part1: {}", part1(&nums));
	println!("part2: {}", part2(&nums));
	Ok(())
}
