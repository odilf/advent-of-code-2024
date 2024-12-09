#[elvish::solution(day = 9, example = 1928)]
fn part1(input: &str) -> u64 {
    let mut memory = Vec::new();
    let mut iter = input.trim().as_bytes().into_iter().map(|&char| char - b'0');

    let mut id = 0;
    loop {
        let Some(size) = iter.next() else {
            break;
        };

        for _ in 0..size {
            memory.push(Some(id));
        }

        if let Some(free_space) = iter.next() {
            for _ in 0..free_space {
                memory.push(None);
            }
        }

        id += 1;
    }

    let mut previous_candidate = 0;
    for removing in (0..memory.len()).rev() {
        let Some(_id) = memory[removing] else {
            continue;
        };

        for candidate in previous_candidate..removing {
            if memory[candidate].is_some() {
                continue;
            }

            previous_candidate = candidate;
            memory.swap(removing, candidate);
            break;
        }
    }

    memory
        .into_iter()
        .enumerate()
        .filter_map(|(i, id)| Some(i as u64 * id?))
        .sum()
}

#[elvish::solution(day = 9, example = 2858)]
fn part2(input: &str) -> u64 {
    let mut memory = Vec::new();
    let mut iter = input.trim().as_bytes().into_iter().map(|&char| char - b'0');

    let mut id = 0;
    loop {
        let Some(size) = iter.next() else {
            break;
        };

        memory.push((Some(id), size));

        if let Some(free_space) = iter.next() {
            memory.push((None, free_space));
        }

        id += 1;
    }

    for removing in (0..memory.len()).rev() {
        let (Some(id), size) = memory[removing] else {
            continue;
        };

        for space_candidate in 0..removing {
            let (None, free_space) = memory[space_candidate] else {
                continue;
            };

            if free_space < size {
                continue;
            }

            memory[removing].0 = None; // This makes an amount of spaces equal to `removing.size`
            memory[space_candidate].1 -= size; // This removes `removing.size` spaces. Net 0.
            memory.insert(space_candidate, (Some(id), size));
            break;
        }
    }

    let mut i = 0;
    let mut checksum = 0;

    for &(id, size) in &memory {
        let Some(id) = id else {
            i += size as u64;
            continue;
        };

        for _ in 0..size {
            checksum += id * i;
            i += 1;
        }
    }

    checksum
}

elvish::example!("2333133121414131402");
