mod part1;
mod test;

fn main() {
	let d3p1 = part1::get_priorities_sum();

	print!("The sum of the priorities of those item types is {} ", d3p1);
}
