use crate::shared::Range;

pub fn get_overlap_ranges(range: &[(Range, Range)]) -> usize {
	range
		.iter()
		.filter(|(first, second)| first.overlap(second) || second.overlap(first))
		.count()
}
