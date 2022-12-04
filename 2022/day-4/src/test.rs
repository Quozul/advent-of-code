#[cfg(test)]
mod tests {
	use crate::shared::{get_ranges, Range};
	use crate::{part1, part2};

	#[test]
	fn test_range_include() {
		let first = Range::new("2-4");
		let second = Range::new("6-8");
		assert!(!first.includes(&second));

		let third = Range::new("2-8");
		let fourth = Range::new("3-7");
		assert!(third.includes(&fourth));
	}

	#[test]
	fn test_range_overlap() {
		let first = Range::new("2-4");
		let second = Range::new("6-8");
		assert!(!first.overlap(&second));

		let third = Range::new("5-7");
		let fourth = Range::new("7-9");
		assert!(third.overlap(&fourth));
	}

	#[test]
	fn test_part1() {
		let ranges: Vec<(Range, Range)> = get_ranges();
		assert_eq!(part1::get_fully_overlap_ranges(&ranges), 441);
	}

	#[test]
	fn test_part2() {
		let ranges: Vec<(Range, Range)> = get_ranges();
		assert_eq!(part2::get_overlap_ranges(&ranges), 861);
	}
}
