use std::collections::{HashMap, VecDeque};

use winnow::{
    ascii::{alpha1, multispace1},
    combinator::separated,
    seq, PResult, Parser,
};

type Pattern = Vec<Color>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn from_char(char: char) -> Self {
        match char {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!("Invalid char {char}"),
        }
    }
}

fn ways_to_make<'p>(
    pattern: &'p [Color],
    extensions: &[Pattern],
    cache: &mut HashMap<&'p [Color], usize>,
) -> usize {
    if let Some(&output) = cache.get(pattern) {
        return output;
    }

    if pattern.is_empty() {
        return 1;
    }

    let output = extensions
        .into_iter()
        .filter_map(|extension| {
            if extension.len() > pattern.len() {
                return None;
            }
            let slice = &pattern[pattern.len() - extension.len()..pattern.len()];
            if extension.as_slice() == slice {
                Some(ways_to_make(
                    &pattern[0..pattern.len() - extension.len()],
                    extensions,
                    cache,
                ))
            } else {
                None
            }
        })
        .sum();

    cache.insert(pattern, output);
    output
}

#[elvish::solution(day = 19, example = 6)]
fn part1(input: &str) -> usize {
    let (available_extensions, target_patterns) = parse(input);

    let mut cache = HashMap::new();
    target_patterns
        .iter()
        .filter(|&pattern| ways_to_make(pattern, &available_extensions, &mut cache) > 0)
        .count()
}

#[elvish::solution(day = 19, example = 16)]
fn part2(input: &str) -> usize {
    let (available_extensions, target_patterns) = parse(input);

    let mut cache = HashMap::new();
    target_patterns
        .iter()
        .map(|pattern| ways_to_make(pattern, &available_extensions, &mut cache))
        .sum()
}

fn parse(input: &str) -> (Vec<Pattern>, Vec<Pattern>) {
    let pat = |input: &mut &str| -> PResult<Pattern> {
        alpha1
            .map(|pat: &str| pat.chars().map(Color::from_char).collect::<Vec<_>>())
            .parse_next(input)
    };

    let mut parser = seq!((
        separated(0.., pat, ", "),
        _: multispace1,
        separated(0.., pat, '\n'),
        _: '\n'
    ));

    parser.parse(input).unwrap()
}

elvish::example!(
    "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "
);
