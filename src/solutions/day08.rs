use std::collections::{HashMap, HashSet};

use nalgebra::{vector, Vector2};

fn parse(input: &str) -> HashMap<u8, Vec<Vector2<i32>>> {
    let mut stations = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.bytes().enumerate() {
            if char == b'.' {
                continue;
            }

            let entry = stations.entry(char).or_insert(Vec::new());
            entry.push(vector![x as i32, y as i32])
        }
    }

    stations
}

#[elvish::solution(day = 8, example = 14)]
fn part1(input: &str) -> usize {
    let bounds = vector![input.lines().next().unwrap().len(), input.lines().count(),];

    let stations = parse(input);

    let mut visited = HashSet::new();

    for stations in stations.values() {
        for (i, &a) in stations.iter().enumerate() {
            for &b in stations[i + 1..].iter() {
                visited.insert(2 * a - b);
                visited.insert(2 * b - a);
            }
        }
    }

    visited
        .into_iter()
        .filter(|&antinode| {
            antinode
                .zip_map(&bounds, |a, b| a >= 0 && a < b)
                .iter()
                .all(|&x| x)
        })
        .count()
}

#[elvish::solution(day = 8, example = 34)]
fn part2(input: &str) -> usize {
    let bounds = vector![input.lines().next().unwrap().len(), input.lines().count(),];

    let stations = parse(input);

    let mut visited = HashSet::new();

    for stations in stations.values() {
        for &a in stations {
            for &b in stations {
                dbg!(a, b);
                if a == b {
                    continue;
                }

                for i in 0.. {
                    let antinode = i * a - (i - 1) * b;
                    if antinode.x >= bounds.x || antinode.y >= bounds.y {
                        break;
                    }
                    visited.insert(antinode);
                }
            }
        }
    }

    visited.len()
}

elvish::example!(
    "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "
);
