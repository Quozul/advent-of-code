mod part1;
mod shared;
mod test;

fn main() {
	let final_score = part1::get_plays();
	println!("My total score with the given strategy is {}.", final_score);
}
