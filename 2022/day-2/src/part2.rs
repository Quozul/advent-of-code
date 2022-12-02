use crate::shared::{Outcome, Shape};

fn parse_shape(c: char) -> Option<Shape> {
	match c {
		'A' => Some(Shape::Rock),
		'B' => Some(Shape::Paper),
		'C' => Some(Shape::Scissors),
		_ => None,
	}
}

fn parse_outcome(c: char) -> Option<Outcome> {
	match c {
		'X' => Some(Outcome::Loose),
		'Y' => Some(Outcome::Draw),
		'Z' => Some(Outcome::Win),
		_ => None,
	}
}

/// Params:
/// * opponent : opponent's plays
/// * outcome : the desired outcome
fn get_what_should_play(opponent: Shape, outcome: &Outcome) -> Shape {
	match opponent {
		Shape::Rock => match outcome {
			Outcome::Loose => Shape::Scissors,
			Outcome::Draw => Shape::Rock,
			Outcome::Win => Shape::Paper,
		},
		Shape::Paper => match outcome {
			Outcome::Loose => Shape::Rock,
			Outcome::Draw => Shape::Paper,
			Outcome::Win => Shape::Scissors,
		},
		Shape::Scissors => match outcome {
			Outcome::Loose => Shape::Paper,
			Outcome::Draw => Shape::Scissors,
			Outcome::Win => Shape::Rock,
		},
	}
}

pub fn get_plays() -> u32 {
	let input = include_str!("input.txt");
	input
		.split('\n')
		.map(|x| {
			let opponent = parse_shape(x.chars().next().unwrap()).unwrap();
			let outcome = parse_outcome(x.chars().nth(2).unwrap()).unwrap();
			let play = get_what_should_play(opponent, &outcome);
			play as u32 + outcome as u32
		})
		.sum()
}
