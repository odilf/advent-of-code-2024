use color_eyre::eyre;
use pathfinding::matrix::directions::DIRECTIONS_4;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    thread::sleep,
    time::Duration,
};
use winnow::{
    ascii::{digit1, space0},
    combinator::{alt, preceded},
};

type Vec2 = nalgebra::Vector2<i64>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl Robot {
    fn advance(&mut self, steps: u64, size: Vec2) {
        self.position =
            (self.position + self.velocity * (steps as i64)).zip_map(&size, |p, s| p.rem_euclid(s))
    }
}

fn solve(robots: impl Iterator<Item = Robot>, size: Vec2, steps: u64) -> u64 {
    let center = size / 2;

    let mut quadrants = HashMap::<_, u64>::with_capacity(4);

    for mut robot in robots {
        robot.advance(steps, size);
        let p = robot.position;

        if p.x == center.x || p.y == center.y {
            continue;
        }

        let right = p.x > center.x;
        let bottom = p.y > center.y;

        let entry = quadrants.entry((right, bottom)).or_default();
        *entry += 1;
    }

    quadrants.into_values().product()
}

#[elvish::solution(day = 14)]
fn part1(input: &str) -> u64 {
    solve(parse(input), Vec2::new(101, 103), 100)
}

#[elvish::solution(day = 14, example = 281)]
fn part2(input: &str) -> u64 {
    let mut robots = parse(input).collect::<Vec<_>>();
    let size = Vec2::new(101, 103);

    for i in 0.. {
        if i > 8000 {
            panic!()
        }

        for robot in &mut robots {
            robot.advance(1, size);
        }

        let mut queue = vec![&robots[0]];
        let mut visited = HashSet::new();
        while let Some(robot) = queue.pop() {
            if !visited.insert(robot) {
                continue;
            }
            for (dx, dy) in DIRECTIONS_4 {
                let delta = Vec2::new(dx as i64, dy as i64);
                if let Some(neighbor) = robots.iter().find(|n| robot.position + delta == n.position) {
                    queue.push(neighbor);
                }
            }
        }

        // if visited.len() < 5 {
        //     continue;
        // }

        let mut buffer = String::with_capacity((size.x * size.y * 2) as usize);
        for y in 0..size.y {
            for x in 0..size.x {
                let p = Vec2::new(x, y);
                if robots.iter().find(|robot| robot.position == p).is_some() {
                    buffer.push('\u{2588}');
                } else {
                    buffer.push('.');
                }
            }

            buffer.push('\n');
        }

        print!("\n\n\n\n{i}{buffer}");

        sleep(Duration::from_millis(60));
    }
    
    panic!();
}

elvish::example!(
    "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "
);

#[test]
fn part1_example() {
    let output = solve(parse(EXAMPLE_PART1), Vec2::new(11, 7), 100);

    assert_eq!(output, 12);
}

fn parse(input: &str) -> impl Iterator<Item = Robot> + use<'_> {
    input.lines().map(|line| Robot::from_str(line).unwrap())
}

impl FromStr for Robot {
    type Err = eyre::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use winnow::{prelude::*, seq};

        fn natural(input: &mut &str) -> PResult<i64> {
            digit1.parse_to().parse_next(input)
        }

        fn number(input: &mut &str) -> PResult<i64> {
            alt((preceded('-', natural).map(|x| -x), natural)).parse_next(input)
        }

        fn vec2(input: &mut &str) -> PResult<Vec2> {
            seq!(number, _: ',', number)
                .map(|(x, y)| Vec2::new(x, y))
                .parse_next(input)
        }

        let mut parser = seq!(Robot {
            position: preceded("p=", vec2),
            _: space0,
            velocity: preceded("v=", vec2),
        });

        Ok(parser.parse(s).map_err(|e| eyre::eyre!(e.to_string()))?)
    }
}
