use std::collections::HashMap;


fn solve(input: &str, total_blinks: u32) -> u64 {
    let stones = input
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap());

    fn explore(stone: u64, blink: u32, total_blinks: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
        if let Some(output) = cache.get(&(stone, blink)) {
            return *output
        }

        if blink == total_blinks {
            return 1;
        }

        let split = |num| {
            let digits = (stone as f32).log10().trunc() as u32 + 1;
            if digits % 2 != 0 {
                None
            } else {
                let power = 10u64.pow(digits / 2);
                Some((num / power, num % power))
            }
        };

        let mut explore = |stone| explore(stone, blink + 1, total_blinks, cache);

        let output = if stone == 0 {
            explore(1)
        } else if let Some((a, b)) = split(stone) {
            explore(a) + explore(b)
        } else {
            explore(stone * 2024)
        };

        cache.insert((stone, blink), output);
        output
    }

    let mut cache = HashMap::new();

    stones.map(|stone| explore(stone, 0, total_blinks, &mut cache)).sum()
}

#[elvish::solution(day = 11, example = 55312)]
fn part1(input: &str) -> u64 {
    solve(input, 25)
}

#[elvish::solution(day = 11, example = 65601038650482)]
fn part2(input: &str) -> u64 {
    solve(input, 75)
}

elvish::example!("125 17");
