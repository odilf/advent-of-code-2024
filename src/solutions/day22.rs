use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use itertools::Itertools as _;

fn mix(secret: &mut i64, salt: i64) {
    *secret = *secret ^ salt;
}

fn prune(secret: &mut i64) {
    *secret = *secret % 16777216;
}

#[test]
fn mix_and_prune() {
    let mut secret = 42;
    mix(&mut secret, 15);
    assert_eq!(secret, 37);

    let mut secret = 100000000;
    prune(&mut secret);
    assert_eq!(secret, 16113920);
}

fn next_secret(mut secret: i64) -> i64 {
    let result = secret * 64;
    mix(&mut secret, result);
    prune(&mut secret);

    let result = secret / 32;
    mix(&mut secret, result);
    prune(&mut secret);

    let result = secret * 2048;
    mix(&mut secret, result);
    prune(&mut secret);

    secret
}

fn secret_numbers(initial: i64) -> impl Iterator<Item = i64> {
    let mut current = initial;
    (0..).map(move |_| {
        current = next_secret(current);
        current
    })
}

#[test]
fn secret_123() {
    let expected = [
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];

    for (actual, expected) in secret_numbers(123).zip(expected) {
        assert_eq!(actual, expected);
    }
}

#[elvish::solution(day = 22, example = 37327623)]
fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .map(|num| secret_numbers(num).take(2000).last().unwrap())
        .sum()
}

#[elvish::solution(day = 22, example = 23)]
fn part2(input: &str) -> i64 {
    let nums = input
        .lines()
        .map(|line| line.parse().unwrap())
        .map(|initial| {
            let nums = secret_numbers(initial)
                .map(|num| num % 10)
                .take(2000)
                .collect::<Vec<_>>();

            let diffs = once(&(initial % 10))
                .chain(&nums)
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>();

            (nums, diffs)
        })
        .collect::<Vec<_>>();

    let diff_map = {
        let mut diff_map = HashMap::new();
        for (nums, diffs) in &nums {
            let mut visited = HashSet::new();
            for (i, window) in diffs.windows(4).enumerate() {
                if !visited.insert(window) {
                    continue;
                }
                let entry = diff_map.entry(window).or_insert(0);
                *entry += nums[i + 3];
            }
        }

        diff_map
    };

    *diff_map.values().max().unwrap()
}

elvish::example!(
    part1: "
        1
        10
        100
        2024
    ",

    part2: "
        1
        2
        3
        2024
    ",
);
