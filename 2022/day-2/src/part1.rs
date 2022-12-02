use crate::shared::{Outcome, Shape};

fn parse_input(c: char) -> Option<Shape> {
	match c {
		'A' => Some(Shape::Rock),
		'B' => Some(Shape::Paper),
		'C' => Some(Shape::Scissors),
		'X' => Some(Shape::Rock),
		'Y' => Some(Shape::Paper),
		'Z' => Some(Shape::Scissors),
		_ => None,
	}
}

/// Params:
/// * a : opponent's plays
/// * b : own play
fn get_outcome(opponent: Shape, own: &Shape) -> Outcome {
	match opponent {
		Shape::Rock => match own {
			Shape::Rock => Outcome::Draw,
			Shape::Paper => Outcome::Win,
			Shape::Scissors => Outcome::Loose,
		},
		Shape::Paper => match own {
			Shape::Rock => Outcome::Loose,
			Shape::Paper => Outcome::Draw,
			Shape::Scissors => Outcome::Win,
		},
		Shape::Scissors => match own {
			Shape::Rock => Outcome::Win,
			Shape::Paper => Outcome::Loose,
			Shape::Scissors => Outcome::Draw,
		},
	}
}

pub fn get_plays() -> u32 {
	let input = include_str!("input.txt");
	input
		.split('\n')
		.map(|x| {
			let opponent = parse_input(x.chars().next().unwrap()).unwrap();
			let own = parse_input(x.chars().nth(2).unwrap()).unwrap();
			let outcome = get_outcome(opponent, &own);
			own as u32 + outcome as u32
		})
		.sum()
}
