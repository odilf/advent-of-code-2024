use nom::{character::complete::space1, sequence::separated_pair};

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    use crate::parse::digit;

    input
        .lines()
        .map(|line| separated_pair(digit::<i32>, space1, digit::<i32>)(line).unwrap().1)
        .unzip()
}

#[elvish::solution(day = 1, example = 11)]
fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(&a, &b)| (a - b).abs())
        .sum()
}

#[elvish::solution(day = 1, example = 31)]
fn part2(input: &str) -> i32 {
    let (left, right) = parse(input);

    left.iter()
        .map(|&a| a * right.iter().filter(|&&b| b == a).count() as i32)
        .sum()
}

elvish::example!(
    "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "
);
