use color_eyre::eyre;
use nalgebra::Matrix2;
use nom::{branch::alt, sequence::{separated_pair, tuple}, IResult, Parser as _};
use nom_supreme::{tag::complete::tag, ParserExt as _};
use std::str::FromStr;

use crate::parse;

type Vec2<T = u64> = nalgebra::Vector2<T>;

struct Machine {
    buttons: [Vec2; 2],
    prize: Vec2,
}


impl Machine {
    pub fn a(&self) -> Vec2 {
        self.buttons[0]
    }

    pub fn b(&self) -> Vec2 {
        self.buttons[1]
    }

    pub fn min_tokens(&self) -> Option<u64> {
        // Idea: we have to solve a system of equations.
        // Ax * x + Bx * y = Px
        // Ay * x + By * y = Py

        let matrix = Matrix2::from_columns(&[self.a(), self.b()]);
        let result = self.prize.cast();

        let clicks = matrix.cast::<f64>().try_inverse().unwrap() * result;
        let clicks = clicks.map(|x| x.round()).try_cast().unwrap();

        // Check integer result
        let hyp_result = matrix.cast::<u64>() * clicks;
        if hyp_result != self.prize.cast() {
            return None;
        }

        Some(clicks.x * 3 + clicks.y)
    }
}

// Too low: 25173

#[elvish::solution(day = 13, example = 480)]
fn part1(input: &str) -> u64 {
    let machines = input.split("\n\n").map(|s| s.parse::<Machine>().unwrap());

    machines.filter_map(|m| m.min_tokens()).sum()
}

#[elvish::solution(day = 13, example = 281)]
fn part2(input: &str) -> u64 {
    let machines = input
        .split("\n\n")
        .map(|s| s.parse::<Machine>().unwrap())
        .map(|mut machine| {
            machine.prize += Vec2::new(1, 1) * 10000000000000;
            machine
        });

    machines.filter_map(|m| m.min_tokens()).sum()
}

elvish::example!(
    part1: "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    ", 

    part2: "
    ",
);


impl FromStr for Machine {
    type Err = eyre::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Ugly
        fn coord(axis: &str) -> impl Fn(&str) -> IResult<&str, u64> + use<'_> {
            move |input| {
                tuple((tag(axis), alt((tag("+"), tag("="))), parse::digit::<u64>))
                    .map(|(_, _, digit)| digit)
                    .parse(input)
            }
        }

        fn vec<'a>(input: &'a str) -> IResult<&'a str, Vec2> {
            separated_pair(coord("X"), tag(", "), coord("Y"))
                .map(|(x, y)| Vec2::new(x, y))
                .parse(input)
        }

        let (rest, a) = vec
            .preceded_by(tag("Button A: "))
            .parse(input.trim())
            .map_err(|e| e.to_owned())?;
        let (rest, b) = vec
            .preceded_by(tag("Button B: "))
            .parse(rest.trim())
            .map_err(|e| e.to_owned())?;
        let (_, prize) = vec
            .preceded_by(tag("Prize: "))
            .parse(rest.trim())
            .map_err(|e| e.to_owned())?;

        Ok(Machine {
            buttons: [a, b],
            prize,
        })
    }
}
