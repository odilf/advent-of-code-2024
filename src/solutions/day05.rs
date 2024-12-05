use std::cmp::Ordering;

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap();
    let prints = iter.next().unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let mut iter = line.split("|");
            let before = iter.next().unwrap().parse::<i32>().unwrap();
            let after = iter.next().unwrap().parse::<i32>().unwrap();

            (before, after)
        })
        .collect::<Vec<_>>();

    let prints = prints
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, prints)
}

#[elvish::solution(day = 5, example = 143)]
fn part1(input: &str) -> i32 {
    let (rules, prints) = parse(input);

    let mut correct_prints = vec![true; prints.len()];
    for (before, after) in rules {
        for (i, print) in prints.iter().enumerate() {
            if let (Some(before), Some(after)) = (
                print.iter().position(|&x| x == before),
                print.iter().position(|&x| x == after),
            ) {
                if before > after {
                    correct_prints[i] = false;
                }
            }
        }
    }

    correct_prints
        .into_iter()
        .zip(prints.into_iter())
        .filter(|&(is_correct, _)| is_correct)
        .map(|(_, print)| print[print.len() / 2])
        .sum()
}

#[elvish::solution(day = 5, example = 123)]
fn part2(input: &str) -> i32 {
    let (rules, prints) = parse(input);

    let mut correct_prints = vec![true; prints.len()];
    for &(before, after) in &rules {
        for (i, print) in prints.iter().enumerate() {
            if let (Some(before), Some(after)) = (
                print.iter().position(|&x| x == before),
                print.iter().position(|&x| x == after),
            ) {
                if before > after {
                    correct_prints[i] = false;
                }
            }
        }
    }

    correct_prints
        .into_iter()
        .zip(prints.into_iter())
        .filter(|&(is_correct, _)| !is_correct)
        .map(|(_, mut print)| {
            print.sort_by(|&a, &b| {
                for &(before, after) in &rules {
                    if a == before && b == after {
                        return Ordering::Less;
                    } else if a == after && b == before {
                        return Ordering::Greater;
                    }
                }

                Ordering::Equal
            });

            print[print.len() / 2]
        })
        .sum()
}

elvish::example!(
    "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "
);
