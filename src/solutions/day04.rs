use ndarray::Array2;

fn parse(input: &str) -> Array2<u8> {
    // TODO: Maybe make a PR to ndarray to add a `from_iter` or even `from_str` and `from_lines` method. 
    let shape = (input.lines().count(), input.lines().next().unwrap().len());
    let bytes = input.bytes().filter(|&char| char != b'\n').collect();
    Array2::from_shape_vec(shape, bytes).unwrap()
}

#[elvish::solution(day = 4, example = 18)]
fn part1(input: &str) -> usize {
    let grid = parse(input);
    let directions = || (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)));

    let grid = &grid;
    grid.indexed_iter()
        .flat_map(move |((x, y), _)| {
            directions().filter(move |&(dx, dy)| {
                b"XMAS".iter().enumerate().all(|(i, char)| {
                    grid.get((x + i * dx as usize, y + i * dy as usize)) == Some(&char)
                })
            })
        })
        .count()
}

#[elvish::solution(day = 4, example = 9)]
fn part2(input: &str) -> usize {
    let grid = parse(input);

    let grid = &grid;
    grid.indexed_iter()
        .filter(|&((x, y), char)| {
            if *char != b'A' {
                return false;
            }

            let letters = [(-1i32, -1i32), (-1, 1), (1, 1), (1, -1)]
                .into_iter()
                .map(|(dx, dy)| grid.get((x + dx as usize, y + dy as usize)))
                .collect::<Vec<_>>();

            let is_cross = (0..2).any(|i| letters[i] == letters[i + 1]);
            let two_of = |x| letters.iter().filter(|&&letter| letter == Some(&x)).count() == 2;

            is_cross && two_of(b'M') && two_of(b'S')
        })
        .count()
}

elvish::example!(
    "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "
);
