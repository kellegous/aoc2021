use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn count_increases(nums: &[i32], n: usize) -> i32 {
	let mut c = 0;
	for i in n..nums.len() {
		if nums[i - n] < nums[i] {
			c += 1;
		}
	}
	c
}

fn read_input<R: Read>(r: R) -> Result<Vec<i32>, Box<dyn Error>> {
	let r = BufReader::new(r);
	let mut vals = Vec::new();
	for line in r.lines() {
		vals.push(line?.parse::<i32>()?);
	}
	Ok(vals)
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day1")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let nums = read_input(File::open(
		matches.value_of("input").unwrap_or("day1/input.txt"),
	)?)?;
	println!("part1: {}", count_increases(&nums, 1));
	println!("part2: {}", count_increases(&nums, 3));
	Ok(())
}
