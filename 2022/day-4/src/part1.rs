use crate::shared::Range;

pub fn get_fully_overlap_ranges(range: &[(Range, Range)]) -> usize {
	range
		.iter()
		.filter(|(first, second)| first.includes(second) || second.includes(first))
		.count()
}
