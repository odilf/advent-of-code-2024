use std::collections::{HashMap, HashSet};

use crate::solutions::day24::types::{Equation, Pair};

use super::{correctness::get_first_incorrect_bit, pairs_and_swaps::swapped};

const MAX_DEPTH: usize = 4;

pub fn find_swaps_depth_first_in_order<'i>(
    equations: &HashMap<&'i str, Equation<'i>>,
    all_swappable_wires: &HashSet<&'i str>,
    inputs: &[[String; 2]],
    outputs: &[String],
    depth: usize,
) -> Option<HashSet<Pair<'i>>> {
    if depth >= MAX_DEPTH {
        return None;
    };

    let Some(first_incorrect_bit) = get_first_incorrect_bit(equations, inputs, outputs) else {
        println!("No incorrect bits, this is an adder!");
        return Some(HashSet::new());
    };

    println!(
        "{} > searching at depth {depth}, first incorrect bit is {first_incorrect_bit}",
        " ".repeat(depth * 2),
    );

    let mut visited = HashSet::new();
    for a in all_swappable_wires {
        for b in all_swappable_wires {
            if a == b {
                continue;
            }
            let pair = Pair::new(a, b);
            if !visited.insert(pair) {
                continue;
            }

            let Some(swapped_equations) = swapped(pair, equations) else {
                // Swap is impossible
                println!("Cannot do swap {pair}");
                continue;
            };

            let Some(new_first_incorrect_bit) =
                get_first_incorrect_bit(&swapped_equations, inputs, outputs)
            else {
                return Some(HashSet::from([pair]));
            };

            if new_first_incorrect_bit <= first_incorrect_bit {
                // println!("Swap worsens result, moved first incorrect bit from {first_incorrect_bit} to {new_first_incorrect_bit}");
                continue;
            }

            println!(
                "{} | Swap {pair} is improvement (first incorrect bit {first_incorrect_bit} -> {new_first_incorrect_bit}). Searching further.", 
                " ".repeat(depth * 2)
            );

            let Some(mut swaps) = find_swaps_depth_first_in_order(
                &swapped_equations,
                all_swappable_wires,
                inputs,
                outputs,
                depth + 1,
            ) else {
                continue;
            };

            swaps.insert(pair);
            return Some(swaps);
        }
    }

    None
}
