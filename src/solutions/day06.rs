use rayon::prelude::*;
use indexmap::IndexSet;
use nalgebra::{vector, Vector2};

#[elvish::solution(day = 6, example = 41)]
fn part1(input: &str) -> usize {
    let mut position = Vector2::zeros();
    let mut obstacles = IndexSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let coords = vector![x as i32, y as i32];

            match char {
                '#' => {
                    obstacles.insert(coords);
                }
                '^' => position = coords,
                _ => (),
            }
        }
    }

    let bounds = vector![
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32
    ];

    let mut velocity = vector![0, -1];
    let mut hit = IndexSet::new();
    while (position + velocity)
        .zip_map(&bounds, |p, b| p < b && p >= 0)
        .iter()
        .all(|&x| x)
    {
        // while position.x < bounds.x && position.y < bounds.y {
        if obstacles.contains(&(position + velocity)) {
            velocity = vector![-velocity.y, velocity.x];
        } else {
            position += velocity;
            hit.insert(position);
        }
    }

    hit.len()
}

#[elvish::solution(day = 6, example = 6)]
fn part2(input: &str) -> usize {
    let mut initial_position = Vector2::zeros();
    let mut obstacles = IndexSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let coords = vector![x as i32, y as i32];

            match char {
                '#' => {
                    obstacles.insert(coords);
                }
                '^' => initial_position = coords,
                _ => (),
            }
        }
    }

    let bounds = vector![
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32
    ];

    let mut count = 0;

    for x in 0..bounds.x {
        count += (0..bounds.y).into_par_iter().filter(|&y| {
            let coords = vector![x, y];
            let mut new_obstacles = obstacles.clone();
            new_obstacles.insert(coords);

            let mut velocity = vector![0, -1];
            let mut visited = IndexSet::new();
            let mut position = initial_position.clone();

            while (position + velocity)
                .zip_map(&bounds, |p, b| p < b && p >= 0)
                .iter()
                .all(|&x| x)
            {
                if visited.contains(&(position, velocity)) {
                    return true;
                }

                if new_obstacles.contains(&(position + velocity)) {
                    velocity = vector![-velocity.y, velocity.x];
                } else {
                    visited.insert((position, velocity));
                    position += velocity;
                }
            }

            false
        }).count()
    }

    count
}

elvish::example!(
    "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "
);
