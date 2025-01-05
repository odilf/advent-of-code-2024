mod correctness;
mod pairs_and_swaps;
mod search;

use correctness::get_first_incorrect_bit;

use super::{
    parse::parse,
    types::{Equation, Pair},
};
use std::{cmp::Reverse, collections::{BTreeSet, HashMap, HashSet}, sync::mpsc};

fn get_inputs_and_outputs(num_bits: usize) -> (Vec<[String; 2]>, Vec<String>) {
    let mut inputs = Vec::with_capacity(num_bits as usize);
    let mut outputs = Vec::with_capacity(num_bits as usize + 1);
    for i in 0..num_bits {
        let x = format!("x{i:<02}");
        let y = format!("y{i:<02}");
        let z = format!("z{i:<02}");
        inputs.push([x, y]);
        outputs.push(z);
    }

    let z_last = format!("z{:<02}", num_bits + 1);
    outputs.push(z_last);

    (inputs, outputs)
}

fn get_all_wires<'i>(equations: &HashMap<&'i str, Equation<'i>>) -> HashSet<&'i str> {
    equations.values().flat_map(|eq| eq.operands()).collect()
}

fn get_mandatory_subsets<'i>(
    equations: &HashMap<&'i str, Equation<'i>>,
    outputs: &'i [String],
) -> Vec<HashSet<&'i str>> {
    let mut output = outputs.iter().map(|output| {
        let mut queue = vec![output.as_str()];
        let mut mandatory_subset = HashSet::new();
        while let Some(wire) = queue.pop() {
            let Some(eq) = equations.get(wire) else {
                continue;
            };

            mandatory_subset.insert(wire);
            queue.extend(eq.operands());
        }

        mandatory_subset
    }).collect::<Vec<_>>();

    output.sort_unstable_by_key(|subset| Reverse(subset.len()));

    output
}

fn get_possible_representatives<'i>(
    mandatory_subsets: &[HashSet<&'i str>],
) -> Vec<HashSet<&'i str>> {
    if mandatory_subsets.len() == 0 {
        return vec![HashSet::new()];
    }


    let current_subset = &mandatory_subsets[0];

    if current_subset.len() == 0 {
        return get_possible_representatives(&mandatory_subsets[1..]);
    }

    let next_representatives = get_possible_representatives(&mandatory_subsets[1..]);

    let mut output = Vec::new();
    for wire in current_subset {
        let mut representatives = next_representatives.clone();
        for subset in &mut representatives {
            subset.insert(wire);
        }

        output.extend(representatives);
    } 

    output
}

const NUM_BITS: usize = 44;

#[elvish::solution(day = 24)]
pub fn part2(input: &str) -> u64 {
    let (_, equations) = parse(input);
    let (inputs, outputs) = get_inputs_and_outputs(NUM_BITS);
    let all_wires = get_all_wires(&equations);
    let swappable_wires = all_wires
        .iter()
        .copied()
        .filter(|wire| !wire.starts_with("x") && !wire.starts_with("y"))
        .chain(outputs.iter().map(|s| s.as_str()))
        .collect::<HashSet<_>>();

    let first_incorrect_bit_at_the_start = get_first_incorrect_bit(&equations, &inputs, &outputs);
    println!("The first incorrect bit at the start is {first_incorrect_bit_at_the_start:?}");

    println!("Swappable wires are: {swappable_wires:?}");


    dbg!(&swappable_wires.len());
    let result =
        search::find_swaps_depth_first_in_order(&equations, &swappable_wires, &inputs, &outputs, 0)
            .unwrap();


    // let mut mandatory_subsets = get_mandatory_subsets(&equations, &outputs);
    //
    // mandatory_subsets.sort_unstable_by_key(|subset| subset.len());
    // println!("We have {} possible mandatory subsets", mandatory_subsets.len());
    //
    // let possible_representatives = get_possible_representatives(&mandatory_subsets);
    // println!("We got {} possible representatives", possible_representatives.len());

    // let set_of_pairs = get_n_pairs(2, &HashSet::new(), &all_wires);
    // dbg!(set_of_pairs.count());

    todo!();
}
