use regex::{Regex, RegexSet};

#[elvish::solution(day = 3, example = 161)]
fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let a: i32 = cap["a"].parse().unwrap();
            let b: i32 = cap["b"].parse().unwrap();
            a * b
        })
        .sum()
}

#[elvish::solution(day = 3, example = 48)]
fn part2(input: &str) -> i32 {
    let set = Regex::new(r"(?<dont>don't)|(?<do>do)|mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let mut enabled = true;
    set.captures_iter(input)
        .filter_map(|cap| {
            cap.name("dont").map(|_| enabled = false);
            cap.name("do").map(|_| enabled = true);

            if !enabled {
                return None;
            }

            let a: i32 = cap.name("a")?.as_str().parse().unwrap();
            let b: i32 = cap.name("b")?.as_str().parse().unwrap();
            Some(a * b)
        })
        .sum()
}

elvish::example!(
    part1: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    part2: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
);
