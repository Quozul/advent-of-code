mod part1;
mod test;

fn main() {
	let d4p1 = part1::get_ranges();
	println!(
		"Amount of assignment pairs that fully contain the other: {}",
		d4p1
	);
}
