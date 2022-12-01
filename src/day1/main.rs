use std::cmp::Ordering;

pub fn get_calories() -> Vec<u32> {
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

pub fn part_1(calories: &[u32]) -> u32 {
	calories[0]
}

pub fn part_2(calories: &[u32]) -> u32 {
	let top3_sum = calories[0..3].iter().sum();

	top3_sum
}
