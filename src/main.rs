mod day1;

fn main() {
	let calories = day1::get_calories();

	let d1p1_result = day1::part_1(&calories);
	println!(
		"The total amount of Calories that Elf is carrying is {}.",
		d1p1_result
	);

	let d1p2_result = day1::part_2(&calories);

	println!(
		"The top three Elves carrying the most Calories is {}.",
		d1p2_result
	);
}
