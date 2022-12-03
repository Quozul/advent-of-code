#[cfg(test)]
mod tests {
	use crate::part1;

	#[test]
	fn test_char_score() {
		assert_eq!(part1::get_char_score('p'), 16);
		assert_eq!(part1::get_char_score('L'), 38);
		assert_eq!(part1::get_char_score('P'), 42);
		assert_eq!(part1::get_char_score('v'), 22);
		assert_eq!(part1::get_char_score('t'), 20);
		assert_eq!(part1::get_char_score('s'), 19);
	}

	#[test]
	fn test_intersect() {
		assert_eq!(part1::intersect("vJrwpWtwJgWr", "hcsFMMfFFhFp"), 16);
	}

	#[test]
	fn test_part_1() {
		assert_eq!(part1::get_priorities_sum(), 8394);
	}
}
