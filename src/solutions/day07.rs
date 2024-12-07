use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, Parser as _};
use pathfinding::prelude::dfs;
use rayon::prelude::*;
use std::ops::{Add, Mul};

use crate::parse::digit;

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (rest, (result, operands)) = separated_pair(
        digit::<u64>,
        tag(": "),
        separated_list1(tag(" "), digit::<u64>),
    )
    .parse(line)
    .unwrap();

    assert!(rest.is_empty());

    (result, operands)
}

fn solve(input: &str, operators: Vec<impl Fn(u64, u64) -> u64 + Sync>) -> u64 {
    input
        .par_lines()
        .filter_map(|line| {
            let (result, operands) = parse_line(line);

            let path = dfs(
                (operands[0], 0),
                |&(current, i)| {
                    operands
                        .get(i + 1)
                        .map(|&next| operators.iter().map(move |f| (f(current, next), i + 1)))
                        .into_iter()
                        .flatten()
                },
                |&(val, i)| val == result && i == operands.len() - 1,
            );

            path.map(|_| result)
        })
        .sum()
}

#[elvish::solution(day = 7, example = 3749)]
fn part1(input: &str) -> u64 {
    solve(input, vec![Add::add, Mul::mul])
}

/// Concatenates, base 10: `132 || 456 = 132456`
fn concatenate(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    a * 10u64.pow(b.ilog10() + 1) + b
}

#[elvish::solution(day = 7, example = 11387)]
fn part2(input: &str) -> u64 {
    solve(input, vec![Add::add, Mul::mul, concatenate])
}

elvish::example!(
    "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "
);
