use std::collections::{BTreeSet, HashSet, VecDeque};

use ndarray::Array2;
use pathfinding::matrix::directions::DIRECTIONS_4;

fn parse(input: &str) -> Array2<u8> {
    let shape = (input.lines().count(), input.lines().next().unwrap().len());
    let bytes = input.bytes().filter(|&char| char != b'\n').collect();

    Array2::from_shape_vec(shape, bytes).unwrap()
}

#[elvish::solution(day = 12, example = 1930)]
fn part1(input: &str) -> u32 {
    let grid = parse(input);

    let mut visited = HashSet::new();

    let explore_region = |pos: (usize, usize), char, visited: &mut HashSet<(usize, usize)>| {
        let mut perimeter = 0;
        let mut area = 0;

        let mut queue = vec![pos];
        while let Some(pos) = queue.pop() {
            if !visited.insert(pos) {
                continue;
            }

            area += 1;

            for (neighbor, _) in neighbors(pos) {
                if grid.get(neighbor) == Some(&char) {
                    queue.push(neighbor)
                } else {
                    perimeter += 1;
                }
            }
        }

        (perimeter, area)
    };

    grid.indexed_iter()
        .map(|(pos, &char)| {
            if visited.contains(&pos) {
                0
            } else {
                let (perimeter, area) = explore_region(pos, char, &mut visited);
                perimeter * area
            }
        })
        .sum()
}

fn neighbors((x, y): (usize, usize)) -> impl Iterator<Item = ((usize, usize), (isize, isize))> {
    DIRECTIONS_4.into_iter().map(move |(dx, dy)| {
        let neighbor = (x.wrapping_add(dx as usize), y.wrapping_add(dy as usize));
        (neighbor, (dx, dy))
    })
}

#[elvish::solution(day = 12, example = 1206)]
fn part2(input: &str) -> u32 {
    let grid = parse(input);

    let mut visited = HashSet::new();

    let explore_region = |pos: (usize, usize), char, visited: &mut HashSet<(usize, usize)>| {
        let mut queue = VecDeque::from([pos]);

        let mut perimeter = BTreeSet::new();
        let mut area = 0;

        while let Some(pos) = queue.pop_front() {
            if !visited.insert(pos) {
                continue;
            }

            area += 1;

            for (neighbor, delta) in neighbors(pos) {
                if grid.get(neighbor) == Some(&char) {
                    queue.push_back(neighbor)
                } else {
                    perimeter.insert((neighbor, delta));
                }
            }
        }

        let mut sides = 0;
        while let Some(&(pos, delta)) = perimeter.first() {
            sides += 1;
            let mut queue = vec![pos];

            // Delete all contiguous edge from perimeter (to not double count)
            while let Some(pos) = queue.pop() {
                if !perimeter.remove(&(pos, delta)) {
                    continue;
                }

                for (nn, _) in neighbors(pos) {
                    queue.push(nn);
                }
            }
        }

        (sides, area)
    };

    grid.indexed_iter()
        .map(|(pos, &char)| {
            if visited.contains(&pos) {
                0
            } else {
                let (sides, area) = explore_region(pos, char, &mut visited);
                sides * area
            }
        })
        .sum()
}

elvish::example!(
    "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "
);
