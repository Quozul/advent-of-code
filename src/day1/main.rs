use std::cmp::Ordering;

fn get_calories() -> Vec<i32> {
    let input = include_str!("input.txt");
    let mut calories = input
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|y| y.parse::<i32>().unwrap()) // unwrap is unsafe!!
                .sum()
        })
        .collect::<Vec<i32>>();

    calories.sort_by(|a, b| match a > b {
        true => Ordering::Less,
        false => Ordering::Greater,
    });

    calories
}

pub fn part_1() -> i32 {
    let calories = get_calories();

    calories[0]
}

pub fn part_2() -> i32 {
    let calories = get_calories();
    let top3_sum = calories[0..3].iter().sum();

    top3_sum
}
