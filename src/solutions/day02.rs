fn parse_line(line: &str) -> Vec<i32> {
    line.split(" ")
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[elvish::solution(day = 2, example = 2)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let digits = parse_line(line);
            valid_list(&digits, (digits[1] - digits[0]).signum())
        })
        .count()
}

fn valid_pair(a: i32, b: i32, signum: i32) -> bool {
    let diff = b - a;

    diff.abs() >= 1 && diff.abs() <= 3 && diff.signum() == signum
}

fn valid_list(digits: &[i32], signum: i32) -> bool {
    digits
        .windows(2)
        .all(|slice| valid_pair(slice[0], slice[1], signum))
}

#[elvish::solution(day = 2, example = 4)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let digits = parse_line(line);

            (0..=digits.len()).any(|exclude| {
                let digits: Vec<_> = digits
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter(|&(i, _)| i != exclude)
                    .map(|(_, v)| v)
                    .collect();

                valid_list(&digits, (digits[1] - digits[0]).signum())
            })
        })
        .count()
}

elvish::example!(
    "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "
);
