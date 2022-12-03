use crate::part1::get_char_score;

pub fn intersect(a: &str, b: &str, c: &str) -> u32 {
	for char in a.chars() {
		if b.contains(char) && c.contains(char) {
			return get_char_score(char);
		}
	}

	0
}

pub fn get_badges() -> u32 {
	let input = include_str!("input.txt");

	let mut lines = input.lines();
	let mut sum: u32 = 0;

	while let Some(line) = lines.next() {
		sum += intersect(line, lines.next().unwrap(), lines.next().unwrap());
	}

	sum
}
