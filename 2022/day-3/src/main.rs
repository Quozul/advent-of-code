mod part1;
mod part2;
mod test;

fn main() {
	let d3p1 = part1::get_priorities_sum();
	println!("The sum of the priorities of those item types is {}.", d3p1);

	let d3p2 = part2::get_badges();
	println!("The sum of the badge priorities is {}.", d3p2);
}
