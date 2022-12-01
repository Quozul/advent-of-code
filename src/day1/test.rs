#[cfg(test)]
mod tests {
	use crate::day1::{part_1, part_2};

	#[test]
	fn test_part1() {
		assert_eq!(part_1(), 67016);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part_2(), 200116);
	}
}
