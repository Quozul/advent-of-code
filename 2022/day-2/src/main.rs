mod part1;
mod part2;
mod shared;
mod test;

fn main() {
	let d2p1 = part1::get_plays();
	println!("My total score with the given strategy is {}.", d2p1);

	let d2p2 = part2::get_plays();
	println!(
		"My total score with the given updated strategy is {}.",
		d2p2
	);
}
