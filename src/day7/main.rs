use std::error::Error;
use std::fs::read_to_string;

fn total_cost<F>(positions: &[isize], x: isize, f: F) -> isize
where
	F: Fn(isize, isize) -> isize,
{
	positions.iter().map(|&p| f(p, x)).sum::<isize>()
}

fn find_min<F>(positions: &[isize], cost: F) -> isize
where
	F: Fn(isize, isize) -> isize,
{
	let mut min = *positions.iter().min().unwrap();
	let mut max = *positions.iter().max().unwrap();

	while max - min >= 2 {
		let mp = (min + max) / 2;
		let ca = total_cost(positions, mp, &cost);
		let cb = total_cost(positions, mp + 1, &cost);
		if ca > cb {
			min = mp;
		} else {
			max = mp;
		}
	}

	if min == max {
		total_cost(&positions, min, &cost)
	} else {
		total_cost(&positions, min, &cost).min(total_cost(&positions, max, &cost))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day7")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let positions = read_to_string(matches.value_of("input").unwrap_or("data/day7/input.txt"))?
		.split(",")
		.map(|s| s.parse::<isize>())
		.collect::<Result<Vec<_>, _>>()?;
	println!("Part 1: {}", find_min(&positions, |a, b| (a - b).abs()));
	println!(
		"Part 2: {}",
		find_min(&positions, |a, b| {
			let n = (a - b).abs();
			(n * (n + 1)) / 2
		})
	);
	Ok(())
}
