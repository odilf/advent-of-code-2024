use std::collections::{HashMap, HashSet};

use nalgebra::Vector2;
use ndarray::{Array2, ShapeBuilder as _};
use pathfinding::matrix::directions::DIRECTIONS_4;

type Vec2 = nalgebra::Vector2<isize>;

fn ensure_neighbors<T>(grid: &HashMap<Vec2, T>) {
    for pos in grid.keys() {
        let mut count = 0;
        for neighbor in neighbors(*pos) {
            if grid.get(&neighbor).is_some() {
                count += 1;
            }
        }

        assert!(count <= 2, "Position {pos} has {count} neighbors");
    }
}

fn neighbors(position: Vec2) -> impl Iterator<Item = Vec2> {
    DIRECTIONS_4.into_iter().map(move |(dx, dy)| {
        let x = position.x + dx;
        let y = position.y + dy;

        Vec2::new(x, y)
    })
}

#[elvish::solution(day = 20, example = 44)]
fn part1(input: &str) -> u32 {
    let ((start, end), mut grid) = parse(input);

    if cfg!(debug_assertions) {
        ensure_neighbors(&grid);
    };

    {
        let mut position = start;
        let mut i = 0;

        grid.insert(start, Some(i));

        while position != end {
            i += 1;
            let neighbor = neighbors(position)
                .find(|neighbor| matches!(grid.get(neighbor), Some(None)))
                .unwrap();
            grid.insert(neighbor, Some(i));
            position = neighbor;
        }
    };

    let mut saves = 0;
    for (&position, distance) in grid.iter() {
        let distance = distance.unwrap();
        for neighbor1 in neighbors(position) {
            for neighbor2 in neighbors(neighbor1) {
                if neighbor2 == position {
                    continue;
                }

                let Some(neighbor_distance) = grid.get(&neighbor2) else {
                    continue;
                };

                let neighbor_distance = neighbor_distance.unwrap();
                let saved = neighbor_distance as i32 - distance as i32;

                if saved >= 100 {
                    saves += 1;
                }
            }
        }
    }

    saves
}

#[elvish::solution(day = 20, example = 281)]
fn part2(input: &str) -> u32 {
    let ((start, end), mut grid) = parse(input);

    if cfg!(debug_assertions) {
        ensure_neighbors(&grid);
    };

    {
        let mut position = start;
        let mut i = 0;

        grid.insert(start, Some(i));

        while position != end {
            i += 1;
            let neighbor = neighbors(position)
                .find(|neighbor| matches!(grid.get(neighbor), Some(None)))
                .unwrap();
            grid.insert(neighbor, Some(i));
            position = neighbor;
        }
    };

    let cheat_time = 20;

    let mut saves = 0;
    for (&start, d_start) in grid.iter() {
        let d_start = d_start.unwrap();
        for (&end, d_end) in grid.iter() {
            let delta = end - start;
            let manhattan = delta.x.abs() + delta.y.abs();
            if manhattan > cheat_time {
                continue;
            }

            let d_end = d_end.unwrap();

            let saved = d_end as i32 - d_start as i32 - manhattan as i32;
            if saved >= 100 {
                saves += 1;
            }
        }
    }

    saves
}

elvish::example!(
    "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "
);

fn parse(input: &str) -> ((Vec2, Vec2), HashMap<Vec2, Option<u32>>) {
    let shape = (input.lines().count(), input.lines().next().unwrap().len());

    let mut start = None;
    let mut end = None;
    let map = input
        .bytes()
        .filter(|&char| !char.is_ascii_whitespace())
        .enumerate()
        .filter_map(|(i, char)| {
            let position = Vec2::new((i % shape.1) as isize, (i / shape.1) as isize);
            match char {
                b'S' => start = Some(position),
                b'E' => end = Some(position),
                _ => (),
            };

            if char == b'#' {
                None
            } else {
                Some((position, None))
            }
        })
        .collect::<HashMap<_, _>>();

    ((start.unwrap(), end.unwrap()), map)
}
