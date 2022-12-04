pub struct Range {
	min: i32,
	max: i32,
}

pub fn split_at(s: &str, sep: char) -> (&str, &str) {
	let i = s.find(sep);
	i.map(|i| (&s[0..i], &s[i + 1..])).unwrap()
}

impl Range {
	pub fn new(range: &str) -> Range {
		let (first, second) = split_at(range, '-');

		Range {
			min: first.parse::<i32>().unwrap(),
			max: second.parse::<i32>().unwrap(),
		}
	}

	pub fn includes(&self, other: &Range) -> bool {
		self.min <= other.min && self.max >= other.max
	}

	pub fn overlap(&self, other: &Range) -> bool {
		(self.max >= other.max || self.max >= other.min) && self.min <= other.min
	}
}

pub fn get_ranges() -> Vec<(Range, Range)> {
	let input = include_str!("input.txt");
	input
		.lines()
		.map(|line| {
			let (first, second) = split_at(line, ',');
			(Range::new(first), Range::new(second))
		})
		.collect::<Vec<(Range, Range)>>()
}
