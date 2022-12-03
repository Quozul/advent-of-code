pub fn get_char_score(c: char) -> u32 {
	let digit = c as u32;
	if c.is_uppercase() {
		digit - 65 + 27
	} else {
		digit - 96
	}
}

pub fn intersect(a: &str, b: &str) -> u32 {
	for char in a.chars() {
		if b.contains(char) {
			return get_char_score(char);
		}
	}

	0
}

pub fn get_priorities_sum() -> u32 {
	let input = include_str!("input.txt");
	input
		.lines()
		.map(|line| {
			let (first, second) = line.split_at(line.len() / 2);

			intersect(first, second)
		})
		.sum()
}
