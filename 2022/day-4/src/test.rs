#[cfg(test)]
mod tests {
	use crate::part1;

	#[test]
	fn test_part1() {
		assert_eq!(part1::get_ranges(), 441);
	}
}
