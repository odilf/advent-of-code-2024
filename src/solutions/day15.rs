use std::{
    fmt::{Debug, Display, Write as _},
    ops::{Add, Neg, Sub},
};

use ndarray::{Array2, ShapeBuilder as _};

type Vec2 = nalgebra::Vector2<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile<R> {
    Wall,
    Robot,
    Rock(R),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move(Vec2);

trait Rock: Eq {
    /// Parses the input into the grid corresponding to this rock
    fn parse_grid(input: &str) -> Grid<Self>
    where
        Self: Sized;

    /// The positions that this rock needs clear to be pushed.
    fn pushes_into(&self, position: Vec2, mov: Move) -> impl IntoIterator<Item = Vec2>;

    fn gps_coordinates(&self, position: Vec2) -> Vec2 {
        position
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rock1;

impl Rock for Rock1 {
    fn parse_grid(input: &str) -> Grid<Self> {
        let shape = (input.lines().count(), input.lines().next().unwrap().len());

        let bytes = input
            .bytes()
            .filter(|&char| !char.is_ascii_whitespace())
            .map(|char| match char {
                b'#' => Some(Tile::Wall),
                b'O' => Some(Tile::Rock(Rock1)),
                b'@' => Some(Tile::Robot),
                b'.' => None,
                _ => panic!("Wrong char `{}`", char::from(char)),
            })
            .collect::<Vec<_>>();

        let array = Array2::from_shape_vec(shape.strides((1, shape.1)), bytes).unwrap();

        Grid { array }
    }

    fn pushes_into(&self, position: Vec2, mov: Move) -> impl IntoIterator<Item = Vec2> {
        [position + mov]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rock2 {
    left: bool,
}
impl Rock2 {
    fn partner(&self, position: Vec2) -> Vec2 {
        if self.left {
            position + Vec2::new(1, 0)
        } else {
            position - Vec2::new(1, 0) 
        }
    }
}

impl Rock for Rock2 {
    fn parse_grid(input: &str) -> Grid<Self> {
        let shape = (
            input.lines().next().unwrap().len() * 2,
            input.lines().count(),
        );

        let bytes = input
            .bytes()
            .filter(|&char| !char.is_ascii_whitespace())
            .flat_map(|char| match char {
                b'#' => [Some(Tile::Wall); 2],
                b'O' => [
                    Some(Tile::Rock(Rock2 { left: true })),
                    Some(Tile::Rock(Rock2 { left: false })),
                ],
                b'@' => [Some(Tile::Robot), None],
                b'.' => [None; 2],
                _ => panic!("Wrong char `{}`", char::from(char)),
            })
            .collect::<Vec<_>>();

        let array = Array2::from_shape_vec(shape.strides((1, shape.0)), bytes).unwrap();

        Grid { array }
    }

    fn pushes_into(&self, position: Vec2, mov: Move) -> impl IntoIterator<Item = Vec2> {
        [position + mov, self.partner(position) + mov]
    }
    
    fn gps_coordinates(&self, position: Vec2) -> Vec2 {
        if self.left {
            position
        } else {
            Vec2::zeros()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid<R> {
    array: Array2<Option<Tile<R>>>,
}

impl<R> Grid<R> {
    pub fn get(&self, position: Vec2) -> &Option<Tile<R>> {
        &self.array[(position.x, position.y)]
    }

    pub fn shape(&self) -> Vec2 {
        let &[x, y] = self.array.shape() else {
            unreachable!()
        };

        Vec2::new(x, y)
    }
}

impl<R: Display> Display for Grid<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.shape().y {
            for x in 0..self.shape().x {
                match &self.array[(x, y)] {
                    Some(Tile::Wall) => f.write_char('#')?,
                    Some(Tile::Rock(rock)) => write!(f, "{rock}")?,
                    Some(Tile::Robot) => f.write_char('@')?,
                    None => f.write_char('.')?,
                };
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl<R: Rock> Grid<R> {
    fn robot(&self) -> Vec2 {
        let (x, y) = self
            .array
            .indexed_iter()
            .find_map(|(pos, tile)| {
                if matches!(tile, Some(Tile::Robot)) {
                    Some(pos)
                } else {
                    None
                }
            })
            .expect("Grid should have exaclty one robot.");

        Vec2::new(x, y)
    }

    pub fn apply(&mut self, mov: Move)
    where
        R: Debug,
    {
        // List of position that are pushed-into.
        let mut pushes = vec![self.robot() + mov];

        let mut i = 0;
        while let Some(&pos) = pushes.get(i) {
            i += 1;
            let rock = match self.get(pos) {
                Some(Tile::Wall) => return,
                Some(Tile::Rock(rock)) => rock,
                Some(Tile::Robot) => {
                    unreachable!();
                }
                None => continue,
            };

            for push_into in rock.pushes_into(pos, mov) {
                // Don't make the same push twice.
                // TODO: Can I remove this?
                if !pushes.contains(&push_into) {
                    pushes.push(push_into);
                }
            }
        }

        for to in pushes.into_iter().rev() {
            let from = to - mov;
            assert_eq!(*self.get(to), None);
            self.array.swap((from.x, from.y), (to.x, to.y));
        }
    }

    pub fn sum_of_gps_coordinates(&self) -> usize {
        self.array
            .indexed_iter()
            .filter_map(|((x, y), tile)| {
                if let Some(Tile::Rock(rock)) = tile {
                    let c = rock.gps_coordinates(Vec2::new(x, y));
                    Some(c.x + 100 * c.y)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl Move {
    fn from_char(char: u8) -> Self {
        Move(match char {
            b'>' => Vec2::new(1, 0),
            b'<' => Vec2::new(usize::MAX, 0),
            b'v' => Vec2::new(0, 1),
            b'^' => Vec2::new(0, usize::MAX),
            _ => panic!("wrong char `{}`", char::from(char)),
        })
    }
}

impl Add<Move> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Move) -> Self::Output {
        self.zip_map(&rhs.0, |l, r| l.wrapping_add(r))
    }
}

impl Sub<Move> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Move) -> Self::Output {
        self + -rhs
    }
}

impl Neg for Move {
    type Output = Move;
    fn neg(self) -> Self::Output {
        Move(self.0.map(|x| -(x as isize) as usize))
    }
}


fn solve<R: Rock + Debug + Display>(input: &str) -> usize {
    let (mut grid, moves) = parse::<R>(input);

    for m in moves {
        // println!("{grid}");
        grid.apply(m);
    }

    grid.sum_of_gps_coordinates()
}

#[elvish::solution(day = 15, example = 10092)]
fn part1(input: &str) -> usize {
    solve::<Rock1>(input)
}

#[elvish::solution(day = 15, example = 9021)]
fn part2(input: &str) -> usize {
    solve::<Rock2>(input)
}

// 74478585072604: Too high

elvish::example!(
    "
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "
);

#[cfg(test)]
const SMOL: &str = elvish::indoc! {
    "
        #######
        #...#.#
        #.....#
        #..OO@#
        #..O..#
        #.....#
        #######

        <vv<<^^<<^^
    "
};

#[test]
fn part2_smol() {
    println!("{}", part2(SMOL));
}

impl Display for Rock1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('O')
    }
}

impl Display for Rock2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.left {
            true => f.write_char('['),
            false => f.write_char(']'),
        }
    }
}

fn parse<R: Rock>(input: &str) -> (Grid<R>, impl Iterator<Item = Move> + use<'_, R>) {
    let (grid, instructions) = {
        let mut parts = input.split("\n\n");
        (parts.next().unwrap(), parts.next().unwrap())
    };

    let instructions = instructions
        .bytes()
        .filter(|&char| !char.is_ascii_whitespace())
        .map(Move::from_char);

    (Rock::parse_grid(grid), instructions)
}
