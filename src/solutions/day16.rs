use std::collections::HashSet;

use nalgebra::vector;
use ndarray::{Array2, ShapeBuilder as _};
use pathfinding::prelude::{astar_bag, dijkstra};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: IVec2,
    direction: IVec2,
}

type IVec2 = nalgebra::Vector2<i64>;

fn successors(grid: &Array2<Option<()>>, node: &Node) -> impl Iterator<Item = (Node, u64)> {
    let Node {
        position,
        direction,
    } = *node;

    let straight = position.cast() + direction;
    let straight = grid[(straight.x as usize, straight.y as usize)].map(|_| {
        (
            Node {
                position: straight,
                direction,
            },
            1,
        )
    });

    let turn_a = IVec2::new(direction.y, -direction.x);
    let turn_b = -turn_a;

    [
        (
            Node {
                position,
                direction: turn_a,
            },
            1000u64,
        ),
        (
            Node {
                position,
                direction: turn_b,
            },
            1000u64,
        ),
    ]
    .into_iter()
    .chain(straight.into_iter())
}

#[elvish::solution(day = 16, example = [7036, 11048])]
fn part1(input: &str) -> u64 {
    let (grid, start, end) = parse(&input);

    let (_path, total_cost) = dijkstra(
        &start,
        |node| successors(&grid, node),
        |node| node.position == end,
    )
    .unwrap();

    total_cost
}

#[elvish::solution(day = 16, example = [45, 64])]
fn part2(input: &str) -> usize {
    let (grid, start, end) = parse(&input);

    let (paths, _total_cost) = astar_bag(
        &start,
        |node| successors(&grid, node),
        |_| 0, // Just to get bag of paths more conviniently
        |node| node.position == end,
    )
    .unwrap();

    let mut potentially_optimal = HashSet::new();
    for path in paths {
        for node in path {
            potentially_optimal.insert(node.position);
        }
    }

    potentially_optimal.len()
}

elvish::example!(
    part1: "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    ", 

    part1: "
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    ",

    // Yuck, repeating the same two inputs twice...
    part2: "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    ", 

    part2: "
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    ",
);

fn parse(input: &str) -> (Array2<Option<()>>, Node, IVec2) {
    let shape = (input.lines().count(), input.lines().next().unwrap().len());

    let mut start = None;
    let mut end = None;
    let bytes = input
        .bytes()
        .filter(|&char| !char.is_ascii_whitespace())
        .enumerate()
        .map(|(i, char)| match char {
            b'#' => None,
            b'.' => Some(()),
            b'S' => {
                start = Some(vector![i % shape.1, i / shape.1].cast());
                Some(())
            }
            b'E' => {
                end = Some(vector![i % shape.1, i / shape.1].cast());
                Some(())
            }
            _ => panic!("Wrong char `{}`", char::from(char)),
        })
        .collect::<Vec<_>>();

    let array = Array2::from_shape_vec(shape.strides((1, shape.1)), bytes).unwrap();

    (
        array,
        Node {
            position: start.unwrap(),
            direction: IVec2::new(1, 0),
        },
        end.unwrap(),
    )
}
