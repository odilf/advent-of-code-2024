use core::fmt;
use std::{collections::HashMap, fmt::Write, iter::repeat};

type Vec2 = nalgebra::Vector2<i32>;

#[derive(Debug, Clone, Copy)]
enum Key {
    A,
    Digit(u8),
}

impl Key {
    fn position(&self) -> Vec2 {
        match *self {
            Self::A => Vec2::new(1, -2),
            Self::Digit(0) => Vec2::new(0, -2),
            Self::Digit(num) => Vec2::new((num as i32 - 1) % 3 - 1, (num as i32 - 1) / 3 - 1),
        }
    }

    fn from_char(char: u8) -> Self {
        match char {
            b'A' => Self::A,
            b'0'..=b'9' => Self::Digit(char - b'0'),
            _ => panic!("Invalid char5 {}", char::from(char)),
        }
    }
}

fn realizations(origin: Vec2, target: Vec2, keypad: bool) -> Vec<Vec<DirKey>> {
    let illegal = if keypad {
        Vec2::new(-1, -2)
    } else {
        Vec2::new(-1, 1)
    };

    let x = DirKey::map_x;
    let y = DirKey::map_y;

    let delta = target - origin;
    match <[_; 2]>::from(delta) {
        [0, dy] => {
            let mut y = y(dy);
            y.push(DirKey::Apply);
            vec![y]
        }
        [dx, 0] => {
            let mut x = x(dx);
            x.push(DirKey::Apply);
            vec![x]
        }
        [dx, dy] => {
            let xy = || {
                let mut seq = x(dx);
                seq.extend(y(dy));
                seq.push(DirKey::Apply);
                seq
            };

            let yx = || {
                let mut seq = y(dy);
                seq.extend(x(dx));
                seq.push(DirKey::Apply);
                seq
            };

            if origin + Vec2::new(dx, 0) == illegal {
                vec![yx()]
            } else if origin + Vec2::new(0, dy) == illegal {
                vec![xy()]
            } else {
                vec![xy(), yx()]
            }
        }
    }
}

fn optimal_move(
    origin: Vec2,
    target: Vec2,
    depth: u32,
    max_depth: u32,
    cache: &mut HashMap<(Vec2, Vec2, u32), u64>,
) -> u64 {
    let cache_key = (origin, target, depth);
    if let Some(output) = cache.get(&cache_key) {
        return output.clone();
    }
    let realizations = realizations(origin, target, depth == 0);

    if depth == max_depth {
        return realizations[0].len() as u64;
    }

    let output = realizations
        .into_iter()
        .map(|sequence| {
            let mut result = 0;
            let mut position = DirKey::Apply.position();
            for dir_key in sequence {
                let length =
                    optimal_move(position, dir_key.position(), depth + 1, max_depth, cache);
                position = dir_key.position();
                result += length;
            }

            result
        })
        .min()
        .unwrap();

    cache.insert(cache_key, output.clone());
    output
}

#[derive(Debug, Clone, Copy)]
enum DirKey {
    Apply,
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for DirKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Apply => 'A',
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        })
    }
}

impl DirKey {
    fn position(&self) -> Vec2 {
        match *self {
            Self::Apply => Vec2::new(1, 1),
            Self::Up => Vec2::new(0, 1),
            Self::Down => Vec2::new(0, 0),
            Self::Left => Vec2::new(-1, 0),
            Self::Right => Vec2::new(1, 0),
        }
    }

    fn map_x(delta: i32) -> Vec<Self> {
        repeat(if delta > 0 { Self::Right } else { Self::Left })
            .take(delta.abs() as usize)
            .collect::<Vec<_>>()
    }

    fn map_y(delta: i32) -> Vec<Self> {
        repeat(if delta > 0 { Self::Up } else { Self::Down })
            .take(delta.abs() as usize)
            .collect::<Vec<_>>()
    }
}

fn solve(input: &str, robots_in_between: u32) -> u64 {
    let codes = input
        .lines()
        .map(|line| line.bytes().map(Key::from_char).collect::<Vec<_>>());

    let mut complexities = 0;
    for code in codes {
        let numeric: u32 = code[0..code.len() - 1]
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| {
                let Key::Digit(digit) = digit else { panic!() };
                *digit as u32 * 10u32.pow(i as u32)
            })
            .sum();

        let mut result = 0;
        let mut position = Key::A.position();
        let mut cache = HashMap::new();

        for key in code {
            let movement = optimal_move(position, key.position(), 0, robots_in_between, &mut cache);
            position = key.position();
            result += movement;
        }

        let complexity = result * numeric as u64;
        complexities += complexity;
    }

    complexities
}

#[elvish::solution(day = 21, example = 126384)]
fn part1(input: &str) -> u64 {
    solve(input, 2)
}

#[elvish::solution(day = 21)]
fn part2(input: &str) -> u64 {
    solve(input, 25)
}

// Debugging, looks cool so I'm keeping it in.
//
//                  3                       7
//              ^   A          <<      ^^   A
//          <   A > A   v <<   AA >  ^ AA > A
// 379A: <v<A>>^AvA^A <vA<AA>>^AAvA<^A>AAvA^A     <vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//
//                  3                           7
//              ^   A        ^^        <<       A
//          <   A > A    <   AA  v <   AA >>  ^ A
//       v<<A>>^AvA^A v<<A>>^AAv<A<A>>^AAvAA^<A>A v<A>^AA<A>Av<A<A>>^AAAvA^<A>A

elvish::example!(
    "
        029A
        980A
        179A
        456A
        379A
    "
);
