#[cfg(test)]
mod tests {
	use crate::part1;
	use crate::part2;

	#[test]
	fn test_part1() {
		assert_eq!(part1::get_plays(), 12276);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2::get_plays(), 9975);
	}
}
