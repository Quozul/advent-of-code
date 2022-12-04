use crate::shared::{get_ranges, Range};

mod part1;
mod part2;
mod shared;
mod test;

fn main() {
	let ranges: Vec<(Range, Range)> = get_ranges();

	let d4p1 = part1::get_fully_overlap_ranges(&ranges);
	println!(
		"Amount of assignment pairs that fully contain the other: {}",
		d4p1
	);

	let d4p2 = part2::get_overlap_ranges(&ranges);
	println!(
		"Amount of assignment pairs that overlap the other: {}",
		d4p2
	);
}
