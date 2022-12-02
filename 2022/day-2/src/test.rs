#[cfg(test)]
mod tests {
	use crate::part1::get_plays;

	#[test]
	fn test_part1() {
		assert_eq!(get_plays(), 12276);
	}
}
