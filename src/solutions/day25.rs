use std::hint;

use itertools::Itertools;
use ndarray::Array2;
use rayon::slice::ParallelSliceMut;

#[elvish::solution(day = 25, example = 3)]
fn part1(input: &str) -> u32 {
    let patterns = parse(input);
    let heights = patterns.map(|pattern| {
        let tile_type = pattern[(0, 0)];

        let heights = pattern.columns().into_iter().map(move |column| {
            for (i, tile) in column.iter().enumerate() {
                if *tile != tile_type {
                    return i as i64 - 1;
                }
            }

            panic!()
        }).collect::<Vec<_>>();

        (tile_type, heights)
    });

    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for (tile_type, heights) in heights {
        if tile_type == true {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    };

    dbg!(&keys, &locks);

    let mut output = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(key, lock)| lock - key <= 0) {
                output += 1;
            }
        }
    }

    output
}

elvish::example!("
    #####
    .####
    .####
    .####
    .#.#.
    .#...
    .....

    #####
    ##.##
    .#.##
    ...##
    ...#.
    ...#.
    .....

    .....
    #....
    #....
    #...#
    #.#.#
    #.###
    #####

    .....
    .....
    #.#..
    ###..
    ###.#
    ###.#
    #####

    .....
    .....
    .....
    #....
    #.#..
    #.#.#
    #####
");

fn parse(input: &str) -> impl Iterator<Item = Array2<bool>> + use<'_> {
    input.split("\n\n").map(|input| {
        let shape = (input.lines().count(), input.lines().next().unwrap().len());
        let bytes = input
            .bytes()
            .filter(|&char| char != b'\n')
            .map(|char| char == b'#')
            .collect();

        Array2::from_shape_vec(shape, bytes).unwrap()
    })
}
