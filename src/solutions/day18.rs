use pathfinding::{
    directed::{bfs::bfs, dijkstra::dijkstra},
    matrix::directions::DIRECTIONS_4,
};
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};

pub type Vec2 = nalgebra::Vector2<i32>;

fn solve(input: &str, take: usize, bounds: Vec2) -> usize {
    let obstacles = parse(input.trim()).take(take).collect::<Vec<_>>();
    let obstacles = &obstacles;

    let (_path, cost) = dijkstra(
        &Vec2::new(0, 0),
        |position| {
            DIRECTIONS_4
                .into_iter()
                .map(|(dx, dy)| *position + Vec2::new(dx as i32, dy as i32))
                .filter(move |p| {
                    p.partial_cmp(&bounds).is_some_and(|cmp| cmp.is_le())
                        && p.partial_cmp(&Vec2::zeros()).is_some_and(|cmp| cmp.is_ge())
                        && !obstacles.contains(&p)
                })
                .map(|p| (p, 1))
                .collect::<Vec<_>>()
        },
        |position| *position == bounds,
    )
    .unwrap();

    cost
}

#[elvish::solution(day = 18)]
fn part1(input: &str) -> usize {
    solve(input, 1024, Vec2::new(70, 70))
}

#[test]
fn example_part1() {
    let output = solve(EXAMPLE_PART1, 12, Vec2::new(6, 6));
    assert_eq!(output, 22);
}

fn solve2(input: &str, bounds: Vec2) -> String {
    // let mut obstacles_iter = parse(input.trim());
    // let mut obstacles = Vec::new();

    let obstacles = parse(input.trim()).collect::<Vec<_>>();

    (0..obstacles.len()).into_par_iter().find_map_first(|i| {
        let obstacles_ref = &obstacles[0..i];

        let result = bfs(
            &Vec2::new(0, 0),
            |position| {
                DIRECTIONS_4
                    .into_iter()
                    .map(|(dx, dy)| *position + Vec2::new(dx as i32, dy as i32))
                    .filter(move |p| {
                        p.partial_cmp(&bounds).is_some_and(|cmp| cmp.is_le())
                            && p.partial_cmp(&Vec2::zeros()).is_some_and(|cmp| cmp.is_ge())
                            && !obstacles_ref.contains(&p)
                    })
                    .collect::<Vec<_>>()
            },
            |position| *position == bounds,
        );

        if result.is_none() {
            let [x, y] = obstacles[i - 1].into();
            Some(format!("{x},{y}"))
        } else {
            None
        }
    }).unwrap()
}

#[elvish::solution(day = 18)]
fn part2(input: &str) -> String {
    solve2(input, Vec2::new(70, 70))
}

#[test]
fn example_part2() {
    let output = solve2(EXAMPLE_PART2, Vec2::new(6, 6));
    assert_eq!(output.as_str(), "6,1");
}

elvish::example!(
    "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "
);

fn parse(input: &str) -> impl Iterator<Item = Vec2> + use<'_> {
    input.lines().map(|line| {
        let mut parts = line.split(',').map(|x| x.parse().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        Vec2::new(x, y)
    })
}
