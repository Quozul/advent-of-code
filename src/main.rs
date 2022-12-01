mod day1;

fn main() {
	let d1p1_result = day1::part_1();
	println!(
		"The total amount of Calories that Elf is carrying is {}.",
		d1p1_result
	);

	let d1p2_result = day1::part_2();

	println!(
		"The top three Elves carrying the most Calories is {}.",
		d1p2_result
	);
}
