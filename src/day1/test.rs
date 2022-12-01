#[cfg(test)]
mod tests {
	use crate::day1::{get_calories, part_1, part_2};

	#[test]
	fn test_part1() {
		let calories = get_calories();
		assert_eq!(part_1(&calories), 67016);
	}

	#[test]
	fn test_part2() {
		let calories = get_calories();
		assert_eq!(part_2(&calories), 200116);
	}
}
