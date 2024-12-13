use std::collections::HashSet;

use ndarray::Array2;
use pathfinding::matrix::directions::DIRECTIONS_4;

fn parse(input: &str) -> Array2<u8> {
    let shape = (input.lines().count(), input.lines().next().unwrap().len());
    let bytes = input
        .bytes()
        .filter(|&char| char != b'\n')
        .map(|char| char - b'0')
        .collect();
    Array2::from_shape_vec(shape, bytes).unwrap()
}

#[elvish::solution(day = 10, example = 36)]
fn part1(input: &str) -> u32 {
    let grid = parse(input);
    let trailheads = grid
        .indexed_iter()
        .filter(|(_, &h)| h == 0)
        .map(|(pos, _)| pos);

    trailheads
        .map(|trailhead| {
            let mut visited = HashSet::with_capacity(grid.len());
            let mut queue = vec![trailhead];
            let mut score = 0;

            while let Some((x, y)) = queue.pop() {
                if !visited.insert((x, y)) {
                    continue;
                }

                let h = grid[(x, y)];
                if h == 9 {
                    score += 1;
                }

                for (dx, dy) in DIRECTIONS_4 {
                    let next_pos = (x.wrapping_add(dx as usize), y.wrapping_add(dy as usize));
                    if let Some(&new_height) = grid.get(next_pos) {
                        if new_height == h + 1 {
                            queue.push(next_pos)
                        }
                    }
                }
            }

            score
        })
        .sum()
}

#[elvish::solution(day = 10, example = 81)]
fn part2(input: &str) -> u32 {
    let grid = parse(input);
    let trailheads = grid
        .indexed_iter()
        .filter(|(_, &h)| h == 0)
        .map(|(pos, _)| pos);

    trailheads
        .map(|trailhead| {
            fn explore((x, y): (usize, usize), grid: &Array2<u8>, visited: &mut HashSet<(usize, usize)>) -> u32 {
                let mut rating = 0;
                let h = grid[(x, y)];
                if h == 9 {
                    rating += 1;
                }

                for (dx, dy) in DIRECTIONS_4 {
                    let next_pos = (x.wrapping_add(dx as usize), y.wrapping_add(dy as usize));
                    if let Some(&new_height) = grid.get(next_pos) {
                        if new_height == h + 1 {
                            rating += explore(next_pos, grid, visited);
                        }
                    }
                }

                return rating
            }

            let mut visited = HashSet::new();
            explore(trailhead, &grid, &mut visited)
        })
        .sum()
}

elvish::example!(
    "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "
);
