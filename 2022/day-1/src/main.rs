mod test;

use std::cmp::Ordering;

/// Returns the amount of calories carried by each elf.
fn get_calories() -> Vec<u32> {
	let input = include_str!("input.txt");
	let mut calories = input
		.split("\n\n")
		.map(|x| {
			x.split('\n')
				.map(|y| y.parse::<u32>().unwrap()) // unwrap is unsafe!!
				.sum()
		})
		.collect::<Vec<u32>>();

	calories.sort_by(|a, b| match a > b {
		true => Ordering::Less,
		false => Ordering::Greater,
	});

	calories
}

fn part_1(calories: &[u32]) -> u32 {
	calories[0]
}

fn part_2(calories: &[u32]) -> u32 {
	let top3_sum = calories[0..3].iter().sum();

	top3_sum
}

fn main() {
	let calories = get_calories();

	let d1p1_result = part_1(&calories);
	println!(
		"The total amount of Calories that Elf is carrying is {}.",
		d1p1_result
	);

	let d1p2_result = part_2(&calories);

	println!(
		"The top three Elves carrying the most Calories is {}.",
		d1p2_result
	);
}
