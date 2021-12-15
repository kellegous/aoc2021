use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

type Pt = (usize, usize);

mod flood {
	use super::Pt;
	use std::collections::{HashSet, VecDeque};

	fn scan<F>(
		s: &mut VecDeque<(isize, isize)>,
		g: &HashSet<Pt>,
		lx: isize,
		rx: isize,
		y: isize,
		is_inside: F,
	) where
		F: Fn(isize, isize, &HashSet<Pt>) -> bool,
	{
		let mut added = false;
		for x in lx..=rx {
			if !is_inside(x, y, g) {
				added = false;
			} else if !added {
				s.push_back((x, y));
				added = true;
			}
		}
	}

	// Implements a scan filling strategy for flood fill.
	// See https://en.wikipedia.org/wiki/Flood_fill#Span_Filling
	pub fn fill<F>(pt: &Pt, is_inside: F) -> HashSet<Pt>
	where
		F: Fn(isize, isize, &HashSet<Pt>) -> bool,
	{
		let (x, y) = pt;

		let mut s = VecDeque::new();
		s.push_back((*x as isize, *y as isize));

		let mut g = HashSet::new();

		while let Some((mut x, y)) = s.pop_back() {
			let mut lx = x;
			while is_inside(lx - 1, y, &g) {
				g.insert((lx as usize - 1, y as usize));
				lx -= 1;
			}
			while is_inside(x, y, &g) {
				g.insert((x as usize, y as usize));
				x += 1;
			}
			scan(&mut s, &g, lx, x - 1, y + 1, &is_inside);
			scan(&mut s, &g, lx, x - 1, y - 1, &is_inside);
		}

		g
	}
}

#[derive(Debug)]
struct Map {
	heights: Vec<u8>,
	stride: usize,
}

impl Map {
	fn from_reader<R: io::Read>(r: R) -> Result<Map, Box<dyn Error>> {
		let r = io::BufReader::new(r);
		let mut heights = Vec::new();
		let mut stride = 0;
		for line in r.lines() {
			let line = line?;
			stride = line.len();
			heights.reserve(heights.len() + stride);
			for c in line.chars() {
				heights.push(match c.to_digit(10) {
					Some(v) => v as u8,
					None => return Err(format!("invalid height: {}", c).into()),
				});
			}
		}

		Ok(Map { heights, stride })
	}

	fn get(&self, pt: &Pt) -> u8 {
		let (x, y) = pt;
		self.heights[y * self.stride + x]
	}

	fn size(&self) -> (usize, usize) {
		(self.stride, self.heights.len() / self.stride)
	}

	fn collect_neighbors_of(&self, dst: &mut Vec<Pt>, pt: &Pt) {
		let (w, h) = self.size();
		let xlim = w - 1;
		let ylim = h - 1;

		let (x, y) = *pt;
		if y > 0 {
			dst.push((x, y - 1));
		}
		if y < ylim {
			dst.push((x, y + 1));
		}
		if x > 0 {
			dst.push((x - 1, y));
		}
		if x < xlim {
			dst.push((x + 1, y));
		}
	}

	fn find_low_points(&self) -> Vec<(Pt, u8)> {
		let (w, h) = self.size();
		let mut neighbors: Vec<Pt> = Vec::with_capacity(4);
		let mut low_points = Vec::new();
		for x in 0..w {
			for y in 0..h {
				neighbors.clear();
				let pt = (x, y);
				self.collect_neighbors_of(&mut neighbors, &pt);
				let v = self.get(&pt);
				if v < neighbors.iter().map(|pt| self.get(pt)).min().unwrap() {
					low_points.push((pt, v));
				}
			}
		}
		low_points
	}

	fn find_basin_at(&self, pt: &Pt) -> HashSet<Pt> {
		flood::fill(pt, |x, y, g| {
			let (w, h) = self.size();
			let pt = (x as usize, y as usize);
			if x < 0 || y < 0 || x >= w as isize || y >= h as isize || g.contains(&pt) {
				false
			} else {
				self.get(&pt) < 9
			}
		})
	}
}

struct DrainSorted<T: Ord> {
	heap: BinaryHeap<T>,
}

impl<T: Ord> Iterator for DrainSorted<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.heap.pop()
	}
}

impl<T: Ord> DrainSorted<T> {
	fn from(heap: BinaryHeap<T>) -> DrainSorted<T> {
		DrainSorted { heap }
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("day09")
		.arg(
			clap::Arg::with_name("input")
				.takes_value(true)
				.help("the input file"),
		)
		.get_matches();

	let map = Map::from_reader(File::open(
		matches.value_of("input").unwrap_or("data/day09/input.txt"),
	)?)?;

	let low_points = map.find_low_points();
	println!(
		"Part 1: {}",
		low_points
			.iter()
			.map(|(_, v)| 1 + *v as usize)
			.sum::<usize>()
	);

	let mut basins = BinaryHeap::new();
	for (pt, _) in low_points.iter() {
		basins.push(map.find_basin_at(pt).len());
	}
	println!(
		"Part 2: {}",
		DrainSorted::from(basins).take(3).product::<usize>()
	);
	Ok(())
}
