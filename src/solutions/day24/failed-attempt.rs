use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fmt,
    str::from_utf8,
};

use itertools::Itertools as _;
use rand::random;
use winnow::{
    ascii::{newline, space1},
    combinator::{alt, separated, trace},
    error::StrContext,
    seq,
    token::take_while,
    PResult, Parser as _,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Gate {
    And,
    Xor,
    Or,
}

impl Gate {
    fn compute(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And => f.write_str("AND"),
            Self::Or => f.write_str("OR"),
            Self::Xor => f.write_str("XOR"),
        }
    }
}

type Wire<'a> = &'a [u8];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Equation<'a> {
    a: Wire<'a>,
    b: Wire<'a>,
    gate: Gate,
}

impl<'a> Equation<'a> {
    fn operands(&self) -> impl Iterator<Item = Wire<'a>> {
        [self.a, self.b].into_iter()
    }
}

impl fmt::Display for Equation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = from_utf8(self.a).unwrap();
        let b = from_utf8(self.b).unwrap();
        // let result = from_utf8(self.result).unwrap();
        write!(f, "{a} {} {b} = something lol", self.gate)
    }
}

fn find_result<'a, 'b>(
    wire: Wire<'a>,
    values: &'b mut HashMap<Wire<'a>, bool>,
    wire_equations: &'b HashMap<Wire<'a>, Equation<'a>>,
) -> bool {
    if let Some(value) = values.get(wire) {
        return *value;
    }

    let equation = wire_equations[wire];
    let a = find_result(equation.a, values, wire_equations);
    let b = find_result(equation.b, values, wire_equations);

    let result = equation.gate.compute(a, b);
    values.insert(wire, result);

    result
}

#[elvish::solution(day = 24, example = 2024)]
fn part1(mut input: &str) -> u64 {
    let (mut values, equations) = parse(&mut input);

    let target_wires = values
        .keys()
        .chain(
            equations
                .iter()
                .flat_map(|(eq, result)| [&eq.a, &eq.b, &result]),
        )
        .filter(|&wire| wire.starts_with(&[b'z']))
        .copied()
        .collect::<Vec<_>>();

    let wire_equations = {
        let mut wire_equations = HashMap::new();
        for &(equation, result) in &equations {
            wire_equations.insert(result, equation);
            // for wire in equation.operands() {
            //     wire_equations.entry(wire).or_default().insert(equation);
            // }
        }

        wire_equations
    };

    let mut output = 0;
    for target_wire in target_wires {
        let value = find_result(target_wire, &mut values, &wire_equations);
        let index = (target_wire[1] - b'0') as u32 * 10 + (target_wire[2] - b'0') as u32;

        if value == true {
            output += 2u64.pow(index);
        }
    }

    output
}

fn compute_value<'i, 'b>(
    wire: Wire<'i>,
    values: &'b mut HashMap<Wire<'i>, bool>,
    wire_equations: &'b HashMap<Wire<'i>, Equation<'i>>,
    pending: &'b mut HashSet<Wire<'i>>,
) -> Option<bool> {
    // Prevents infinite loop, where a value depends on itself.
    if !pending.insert(wire) {
        println!(
            "Preventin loop with {} getting {}. Available are {}",
            print_set(pending.iter()),
            from_utf8(wire).unwrap(),
            print_set(values.keys())
        );

        return None;
    }
    if let Some(value) = values.get(wire) {
        return Some(*value);
    }

    let equation = wire_equations.get(wire)?;
    let a = compute_value(equation.a, values, wire_equations, pending)?;
    let b = compute_value(equation.b, values, wire_equations, pending)?;

    let result = equation.gate.compute(a, b);

    values.insert(wire, result);
    pending.remove(wire);

    Some(result)
}

const TRUTH_TABLE: [(bool, bool); 4] = [(false, false), (true, false), (false, true), (true, true)];

fn is_adder_bit<'a, 'b>(
    bit: usize,
    wire_equations: &'b HashMap<Wire<'a>, Equation<'a>>,
    inputs: &[(String, String)],
    outputs: &[String],
) -> bool {
    if bit == 0 {
        let (x_wire, y_wire) = &inputs[bit];
        let x_wire = x_wire.as_bytes();
        let y_wire = y_wire.as_bytes();
        let z_wire = outputs[bit].as_bytes();

        for (x0, y0) in TRUTH_TABLE {
            let mut values = HashMap::from([(x_wire, x0), (y_wire, y0)]);
            let mut pending = HashSet::new();
            let Some(z0) = compute_value(z_wire, &mut values, wire_equations, &mut pending) else {
                return false;
            };

            if z0 != (x0 ^ y0) {
                return false;
            }
        }
    } else if bit == inputs.len() {
        let (x_wire_prev, y_wire_prev) = &inputs[bit - 1];
        let x_wire = x_wire_prev.as_bytes();
        let y_wire = y_wire_prev.as_bytes();
        let z_wire = outputs[bit].as_bytes();

        for (x_prev, y_prev) in TRUTH_TABLE {
            let mut values = HashMap::from([(x_wire, x_prev), (y_wire, y_prev)]);
            let mut pending = HashSet::new();
            let Some(z) = compute_value(z_wire, &mut values, wire_equations, &mut pending) else {
                return false;
            };

            if z != (x_prev & y_prev) {
                return false;
            }
        }
    } else {
        let (x_wire, y_wire) = &inputs[bit];
        let (x_wire_prev, y_wire_prev) = &inputs[bit - 1];

        let x_wire = x_wire.as_bytes();
        let y_wire = y_wire.as_bytes();
        let x_wire_prev = x_wire_prev.as_bytes();
        let y_wire_prev = y_wire_prev.as_bytes();

        let z_wire = outputs[bit].as_bytes();

        for (x, y) in TRUTH_TABLE {
            for (x_prev, y_prev) in TRUTH_TABLE {
                let mut values = HashMap::from([
                    (x_wire, x),
                    (y_wire, y),
                    (x_wire_prev, x_prev),
                    (y_wire_prev, y_prev),
                ]);

                let mut pending = HashSet::new();
                let Some(z) = compute_value(z_wire, &mut values, wire_equations, &mut pending)
                else {
                    return false;
                };

                if z != (x ^ y) ^ (x_prev & y_prev) {
                    return false;
                }
            }
        }
    }

    true
}

// fn find_incorrect_bits<'a, 'b>(
//     wire_equations: &'b HashMap<Wire<'a>, Equation<'a>>,
//     inputs: &[(String, String)],
//     outputs: &[String],
// ) -> usize {
//     let mut incorrect_bits = 0;
//     for bit in 0..=inputs.len() {
//         if !is_adder_bit(bit, wire_equations, inputs, outputs) {
//             incorrect_bits += 1;
//         }
//     }
//
//     incorrect_bits
// }

fn swap<'i, 'b>(
    wire_equations: &'b HashMap<Wire<'i>, Equation<'i>>,
    a: Wire<'i>,
    b: Wire<'i>,
) -> HashMap<Wire<'i>, Equation<'i>> {
    let mut output = wire_equations.clone();
    // dbg!(from_utf8(a), from_utf8(b));

    let eq_a = output.remove(a).unwrap();
    let eq_b = output.remove(b).unwrap();

    output.insert(b, eq_a);
    output.insert(a, eq_b);

    output
}

fn is_adder<'i, 'b>(
    wire_equations: &'b HashMap<Wire<'i>, Equation<'i>>,
    inputs: &[(String, String)],
    outputs: &[String],
) -> bool {
    for i in 0..20 {
        let mut values = inputs
            .iter()
            .flat_map(|(x, y)| [(x.as_bytes(), random()), (y.as_bytes(), random())])
            .collect::<HashMap<_, bool>>();

        // for (wire, value) in &values {
        //     println!("Wire {} has value {value}", from_utf8(wire).unwrap());
        // }

        let mut pending = HashSet::new();
        let Some(result) = outputs
            .iter()
            .map(|wire| compute_value(wire.as_bytes(), &mut values, wire_equations, &mut pending))
            .collect::<Option<Vec<bool>>>()
        else {
            return false;
        };

        let mut x = 0;
        let mut y = 0;
        for (i, (x_wire, y_wire)) in inputs.iter().enumerate() {
            if values[x_wire.as_bytes()] {
                x += 2u64.pow(i as u32)
            }

            if values[y_wire.as_bytes()] {
                y += 2u64.pow(i as u32)
            }
        }

        let z = result
            .iter()
            .enumerate()
            .map(|(i, z_on)| if *z_on { 2u64.pow(i as u32) } else { 0 })
            .sum::<u64>();
        if x + y != z {
            return false;
        }
    }

    true
}

fn find_swaps_depth_first<'i, 'b>(
    wire_equations: &'b HashMap<Wire<'i>, Equation<'i>>,
    all_wires: &[Wire<'i>],
    necessary_wires: &[Wire<'i>],
    inputs: &[(String, String)],
    outputs: &'i [String],
    depth: usize,
    visited_swaps: &mut HashSet<BTreeSet<Wire<'i>>>,
) -> Option<Vec<[Wire<'i>; 2]>> {
    // let incorrect_bits = find_incorrect_bits(wire_equations, inputs, outputs);
    // if incorrect_bits == 0 {
    if is_adder(wire_equations, inputs, outputs) {
        return Some(Vec::new());
    };

    if depth > 4 {
        return None;
    }

    println!(
        "{} > searching at depth {depth}, one of {} needs to be swapped",
        " ".repeat(depth as usize * 2),
        print_set(necessary_wires),
    );

    let wires_a = all_wires;
    let wires_b = all_wires;
    // let wires_a = if depth * 2 < necessary_wires.len() {
    //     necessary_wires
    // } else {
    //     println!(
    //         "Getting all wires for a with len {} and depth {}",
    //         necessary_wires.len(),
    //         depth
    //     );
    //     all_wires
    // };
    // let wires_b = if depth * 2 + 1 < necessary_wires.len() {
    //     necessary_wires
    // } else {
    //     println!(
    //         "Getting all wires for b with len {} and depth {}",
    //         necessary_wires.len(),
    //         depth
    //     );
    //     all_wires
    // };

    for &a in wires_a {
        for &b in wires_b {
            if a == b || !visited_swaps.insert(BTreeSet::from([a, b])) {
                continue;
            }

            let swapped_wire_equations = swap(wire_equations, a, b);
            if is_adder(wire_equations, inputs, outputs) {
                panic!("YES");
            }

            // let new_incorrect_bits = find_incorrect_bits(&swapped_wire_equations, inputs, outputs);
            // if new_incorrect_bits >= incorrect_bits {
            //     continue;
            // }

            println!(
                "{} | We found a better swap ({} <-> {}) at depth {depth}. Searching further.",
                " ".repeat(depth as usize * 2),
                from_utf8(a).unwrap(),
                from_utf8(b).unwrap()
            );

            if let Some(mut swaps) = find_swaps_depth_first(
                &swapped_wire_equations,
                all_wires,
                necessary_wires,
                inputs,
                outputs,
                depth + 1,
                visited_swaps,
            ) {
                swaps.push([a, b]);
                return Some(swaps);
            }

            println!(
                "{} < Search with swap {} <-> {} at depth {depth} Was unsuccsesful",
                " ".repeat(depth as usize * 2),
                from_utf8(a).unwrap(),
                from_utf8(b).unwrap()
            );
        }
    }

    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<'i> {
    swaps: Vec<[Wire<'i>; 2]>,
    wire_equations: HashMap<Wire<'i>, Equation<'i>>,
    incorrect_bits: usize,
}

impl PartialOrd for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.incorrect_bits.cmp(&other.incorrect_bits).reverse())
    }
}

impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.incorrect_bits.cmp(&other.incorrect_bits).reverse()
    }
}

// fn find_swaps_priority_queue<'i, 'b>(
//     wire_equations: &'b HashMap<Wire<'i>, Equation<'i>>,
//     wires: &[Wire<'i>],
//     inputs: &[(String, String)],
//     outputs: &[String],
//     depth: u64,
// ) -> Option<Vec<[Wire<'i>; 2]>> {
//     let initial_node = Node {
//         wire_equations: wire_equations.clone(),
//         incorrect_bits: find_incorrect_bits(&wire_equations, inputs, outputs),
//         swaps: Vec::with_capacity(4),
//     };
//
//     let mut queue = BinaryHeap::from([initial_node]);
//
//     while let Some(node) = queue.pop() {
//         if node.incorrect_bits == 0 {
//             return Some(node.swaps);
//         }
//
//         dbg!(node.incorrect_bits);
//
//         for (i, &a) in wires.iter().enumerate() {
//             for &b in &wires[i + 1..] {
//                 // println!("Trying swap {} with {}", from_utf8(a).unwrap(), from_utf8(b).unwrap());
//                 let swapped_wire_equations = swap(wire_equations, a, b);
//                 let new_incorrect_bits =
//                     find_incorrect_bits(&swapped_wire_equations, inputs, outputs);
//                 let mut swaps = node.swaps.clone();
//                 swaps.push([a, b]);
//
//                 let node = Node {
//                     wire_equations: swapped_wire_equations,
//                     incorrect_bits: new_incorrect_bits,
//                     swaps,
//                 };
//
//                 queue.push(node);
//             }
//         }
//     }
//
//     None
// }

/// A set of sets where each set must share at least one element with the set of all wires.
fn get_necessary_subsets<'i, 'b>(
    wire_equations: &'b HashMap<Wire<'i>, Equation<'i>>,
    inputs: &[(String, String)],
    outputs: &'i [String],
) -> Vec<HashSet<Wire<'i>>> {
    let mut all_candidates = Vec::new();
    // let mut all_wires = Vec::<Wire>::new();

    for (i, output) in outputs.iter().enumerate() {
        if is_adder_bit(i, wire_equations, inputs, outputs) {
            continue;
        }

        let mut current_candidate = HashSet::new();
        let out_wire = output.as_bytes();
        let mut queue: Vec<Wire<'i>> = vec![out_wire];
        while let Some(wire) = queue.pop() {
            let Some(eq) = wire_equations.get(wire) else {
                continue;
            };

            current_candidate.insert(wire);
            queue.extend(eq.operands());
        }

        // all_wires.extend(&current_candidate);
        all_candidates.push(current_candidate);
    }

    all_candidates
}

fn print_set<'i, 'b>(set: impl IntoIterator<Item = &'b Wire<'i>>) -> String
where
    'i: 'b,
{
    set.into_iter()
        .map(|&wire| from_utf8(wire).unwrap())
        .join(", ")
}

/// All sets of at most size 8 where each necessary subsets has an intersection with the return.
fn possible_sets_of_wires<'i, 'b>(
    wire_equations: &'b HashMap<Wire<'i>, Equation<'i>>,
    inputs: &'b [(String, String)],
    outputs: &'i [String],
) -> Vec<HashSet<Wire<'i>>> {
    let mut necessary_subsets = get_necessary_subsets(wire_equations, inputs, outputs);
    necessary_subsets.sort_unstable_by_key(|s| s.len());

    // println!("The necessary subsets are: ");
    // for subset in &necessary_subsets {
    //     println!("{}", print_set(subset));
    // }

    fn recurse<'i, 'b>(
        necessary_subsets: &'b [HashSet<Wire<'i>>],
        accumulator: &'b mut HashSet<Wire<'i>>,
    ) -> Vec<HashSet<Wire<'i>>> {
        if necessary_subsets.len() == 0 {
            return vec![accumulator.iter().copied().collect::<HashSet<_>>()];
        }

        let current = &necessary_subsets[0];

        // println!("Current set is {{ {} }}", print_set(current));
        // println!("Accumulator is [{}]", print_set(accumulator.iter()));
        if accumulator.intersection(current).next().is_some() {
            return recurse(&necessary_subsets[1..], accumulator);
        }

        let mut output = Vec::new();
        for &wire in current {
            let inserted = accumulator.insert(wire);
            if !inserted {
                continue;
            }
            output.extend(recurse(&necessary_subsets[1..], accumulator));
            accumulator.remove(wire);
        }

        output
    }

    let mut accumulator = HashSet::new();
    recurse(&necessary_subsets, &mut accumulator)
}

// fn find_candidates<'i, 'b>(
//     wire_equations: &'b HashMap<Wire<'i>, Equation<'i>>,
//     inputs: &[(String, String)],
//     outputs: &'i [String],
// ) -> Vec<Wire<'i>> {
//     let mut best_candidate = None::<HashSet<Wire>>;
//     for (i, output) in outputs.iter().enumerate() {
//         if is_adder_bit(i, wire_equations, inputs, outputs) {
//             continue;
//         }
//
//         let mut current_candidate = HashSet::new();
//         let out_wire = output.as_bytes();
//         let mut queue: Vec<Wire<'i>> = vec![out_wire];
//         while let Some(wire) = queue.pop() {
//             let Some(eq) = wire_equations.get(wire) else {
//                 continue;
//             };
//
//             current_candidate.insert(wire);
//             queue.extend(eq.operands());
//         }
//
//         if current_candidate.len()
//             < best_candidate
//                 .as_ref()
//                 .map(|c| c.len())
//                 .unwrap_or(usize::MAX)
//         {
//             // println!(
//             //     "better candidate: {} at index {i}",
//             //     current_candidate
//             //         .iter()
//             //         .map(|wire| from_utf8(wire).unwrap())
//             //         .join(",")
//             // );
//             best_candidate = Some(current_candidate);
//         }
//     }
//
//     best_candidate.unwrap().into_iter().collect::<Vec<_>>()
// }

fn solve2(input: &str, num_bits: u8) -> String {
    let (_values, equations) = parse(input);

    let all_wires = equations
        .iter()
        .map(|(_, result)| *result)
        .collect::<Vec<_>>();

    let wire_equations = {
        let mut wire_equations = HashMap::new();
        for &(equation, result) in &equations {
            wire_equations.insert(result, equation);
        }

        wire_equations
    };

    let mut inputs = Vec::with_capacity(num_bits as usize);
    let mut outputs = Vec::with_capacity(num_bits as usize + 1);
    for i in 0..=num_bits {
        let x = format!("x{i:<02}");
        let y = format!("y{i:<02}");
        let z = format!("z{i:<02}");
        inputs.push((x, y));
        outputs.push(z);
    }

    let z_last = format!("z{:<02}", num_bits + 1);
    outputs.push(z_last);

    let possible_sets = possible_sets_of_wires(&wire_equations, &inputs, &outputs);
    for possible_set in possible_sets
        .into_iter()
        .filter(|set| set.len() <= 8)
        .sorted_by_key(|set| Reverse(set.len()))
    {
        let mut visited_swaps = HashSet::new();
        let necessary_wires = possible_set.into_iter().collect::<Vec<_>>();
        println!("Searching with set: {}", print_set(&necessary_wires));
        let Some(result) = find_swaps_depth_first(
            &wire_equations,
            &all_wires,
            &necessary_wires,
            &inputs,
            &outputs,
            0,
            &mut visited_swaps,
        ) else {
            println!();
            continue;
        };

        println!("RESULT!! {result:?}");
    }

    todo!()
    // let swaps = find_swaps_depth_first(&wire_equations, &wires, &inputs, &outputs, 0).unwrap();
    //
    // let mut swaps = swaps.into_flattened();
    // swaps.sort_unstable();
    //
    // swaps
    //     .into_iter()
    //     .map(|wire| from_utf8(wire).unwrap())
    //     .join(",")

    // dbg!(&outputs);
    //
    // for z in &outputs {
    //     let mut dependencies = BTreeSet::new();
    //     let mut dependency_eqs = BTreeSet::new();
    //     let mut queue = vec![z.as_bytes()];
    //
    //     while let Some(wire) = queue.pop() {
    //         dependencies.insert(wire);
    //         if wire != z.as_bytes() && wire.starts_with(&[b'z']) {
    //             continue;
    //         }
    //
    //         let Some(eq) = wire_equations.get(&wire) else {
    //             continue;
    //         };
    //
    //         dependency_eqs.insert(eq);
    //         queue.extend(eq.operands());
    //     }
    //
    //     // println!("{z} has {} dependencies", dependencies.len());
    //     // println!(
    //     //     "{z} has dependencies {}",
    //     //     dependencies
    //     //         .into_iter()
    //     //         .map(|wire| from_utf8(wire).unwrap())
    //     //         .join(", ")
    //     // );
    //
    //     println!(
    //         "{z} has dependency equations \n{}",
    //         dependency_eqs.into_iter().join("\n")
    //     );
    //
    //     println!();
    // }

    // let get_zs = |i: usize, x, y| -> Vec<bool> {
    //     let (x_wire, y_wire) = &inputs[i];
    //     let mut new_values = values.clone();
    //     new_values.insert(x_wire.as_bytes(), x);
    //     new_values.insert(y_wire.as_bytes(), y);
    //
    //     let mut zs = Vec::new();
    //     for i in 0..=num_bits {
    //         let z_wire = &outputs[i as usize];
    //         zs.push(find_result(
    //             z_wire.as_bytes(),
    //             &mut new_values,
    //             &wire_equations,
    //         ));
    //     }
    //
    //     zs
    // };
    //
    // fn exonorate<'input, 'a>(
    //     wire_equations: &'a HashMap<&'input [u8], Equation<'input>>,
    //     num_bits: u8,
    //     get_zs: impl Fn(usize, bool, bool) -> Vec<bool>,
    //     outputs: &[String],
    // ) -> HashSet<Equation<'input>>
    // where
    //     'input: 'a,
    // {
    //     let mut exonorated = HashSet::new();
    //     for i in 0usize..=(num_bits as usize) {
    //         let zs = get_zs(i as usize, false, false);
    //         let mut affected = BTreeSet::new();
    //         let mut wrong_results = Vec::new();
    //         for (x, y) in [(true, false), (false, true), (true, true)] {
    //             let other_zs = get_zs(i, x, y);
    //             for (i, (z, other_z)) in zs.iter().zip(&other_zs).enumerate() {
    //                 if z != other_z {
    //                     affected.insert(i);
    //                 }
    //             }
    //
    //             if x | y != zs[i] {
    //                 wrong_results.push((x, y, zs[i]));
    //             }
    //         }
    //
    //         // println!("{i} affects {affected:?}");
    //
    //         if affected != BTreeSet::from([i, i + 1]) || !wrong_results.is_empty() {
    //             continue;
    //         }
    //
    //         for (x, y, z) in wrong_results {
    //             // println!("{i} has wrong result {x} | {y} = {z}");
    //         }
    //
    //         // println!("{i} exonorated");
    //
    //         let mut queue = vec![outputs[i].as_bytes()];
    //         while let Some(wire) = queue.pop() {
    //             let Some(&eq) = wire_equations.get(wire) else {
    //                 continue;
    //             };
    //
    //             queue.extend(eq.operands());
    //             exonorated.insert(eq);
    //             queue.extend(eq.operands());
    //         }
    //     }
    //
    //     exonorated
    // }
    //
    // let exonorated = exonorate(&wire_equations, num_bits, get_zs, &outputs);
    //
    // dbg!(equations.len());
    // dbg!(exonorated.len());
    //
    // let candidates = equations
    //     .iter()
    //     .filter(|eq| !exonorated.contains(eq))
    //     .collect::<Vec<_>>();
    //
    // for (i, a) in candidates.iter().enumerate() {
    //     for b in &candidates[i + 1..] {
    //         let mut new_wire_equations = wire_equations.clone();
    //         let mut a = (*a).clone();
    //         let mut b = (*b).clone();
    //         mem::swap(&mut a.result, &mut b.result);
    //         new_wire_equations.insert(a.result, a);
    //         new_wire_equations.insert(b.result, b);
    //
    //         let new_exonorated = exonorate(&new_wire_equations, num_bits, get_zs, &outputs);
    //         if new_exonorated.len() > exonorated.len() {
    //             dbg!(new_exonorated.len());
    //             println!("WE MIGHT BE DOING SOMETHING!!!! {a:?}, {b:?}");
    //         }
    //     }
    // }

    // Certainly wrong, I feel
    // let mut wrong_wires = Vec::new();
    // for (x_wire, y_wire, z_wire) in variables {
    //     for (x, y) in [(false, false), (false, true), (true, false), (true, true)] {
    //         let mut new_values = values.clone();
    //         new_values.insert(x_wire.as_bytes(), x);
    //         new_values.insert(y_wire.as_bytes(), y);
    //
    //         let result = find_result(z_wire.as_bytes(), &mut new_values, &wire_equations);
    //         if result != (x | y) {
    //             mistakes += 1;
    //             println!("PROBLEM WITH {x_wire} and {y_wire}!!");
    //             println!("{x} | {y} = {result}");
    //         }
    //     }
    // }
}

#[elvish::solution(day = 24)]
fn part2(input: &str) -> String {
    solve2(input, 44)
}

#[test]
fn example_part2() {
    assert_eq!(solve2(EXAMPLE_PART2, 4).as_str(), "z00,z01,z02,z05");
}

fn parse<'a>(input: &'a str) -> (HashMap<Wire<'a>, bool>, Vec<(Equation<'a>, Wire<'a>)>) {
    let input = input.as_bytes();

    fn wire<'a>(input: &mut &'a [u8]) -> PResult<Wire<'a>> {
        let parser = take_while(3, (b'a'..=b'z', b'0'..=b'9'));
        trace("wire", parser).parse_next(input)
    }

    fn initial_value<'a>(input: &mut &'a [u8]) -> PResult<(Wire<'a>, bool)> {
        let parser = seq!((
            wire,
            _: ":",
            _: space1,
            alt(['0', '1']).map(|c| c == '1')
        ));
        trace("initial_value", parser).parse_next(input)
    }

    fn initial_values<'a>(input: &mut &'a [u8]) -> PResult<HashMap<Wire<'a>, bool>> {
        trace(
            "initial_values",
            separated(0.., initial_value, newline).map(|v: HashMap<_, _>| v),
        )
        .parse_next(input)
    }

    fn gate(input: &mut &[u8]) -> PResult<Gate> {
        let parser = alt((
            "AND".map(|_| Gate::And),
            "OR".map(|_| Gate::Or),
            "XOR".map(|_| Gate::Xor),
        ));

        trace("gate", parser)
            .context(StrContext::Label("Couldn't parse gate"))
            .parse_next(input)
    }

    fn equation<'a>(input: &mut &'a [u8]) -> PResult<(Equation<'a>, Wire<'a>)> {
        let parser = seq!((
            wire,
            _: space1,
            gate,
            _: space1,
            wire,
            _: space1,
            _: "->",
            _: space1,
            wire,
        ))
        .map(|(a, gate, b, result)| (Equation { a, b, gate }, result));
        trace("operator", parser).parse_next(input)
    }

    fn equations<'a>(
        input: &mut &'a [u8],
        // ) -> PResult<impl Iterator<Item = (Wire<'a>, Gate, Wire<'a>, Wire<'a>)> + use<'a>> {
    ) -> PResult<Vec<(Equation<'a>, Wire<'a>)>> {
        separated(0.., equation, newline).parse_next(input)
        // Ok(input
        //     .split(|&c| c == b'\n')
        //     .map(|line| operator.parse(dbg!(line)).unwrap()))
    }

    // let initial_values = initial_values(&mut input);
    // dbg!(initial_values);
    // dbg!(input);
    trace(
        "whole",
        seq!((
            initial_values,
            _: "\n\n",
            equations
        )),
    )
    .parse(input.trim_ascii())
    .unwrap()
    // (todo!(), [todo!()].into_iter())
}

const SMOL: &str = elvish::indoc! {
    "
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    "
};

#[test]
fn parse_smol() {
    parse(SMOL);
}

elvish::example!(
    part1: "
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    ",

    part2: "
        x00: 0
        x01: 1
        x02: 0
        x03: 1
        x04: 0
        x05: 1
        y00: 0
        y01: 0
        y02: 1
        y03: 1
        y04: 0
        y05: 1

        x00 AND y00 -> z05
        x01 AND y01 -> z02
        x02 AND y02 -> z01
        x03 AND y03 -> z03
        x04 AND y04 -> z04
        x05 AND y05 -> z00
    "
);

// x00: 1\nx01: 0\nx02: 1\nx03: 1\nx04: 0\ny00: 1\ny01: 1\ny02: 1\ny03: 1\ny04: 1\n\nntg XOR fgs -> mjb\ny02 OR x01 -> tnw\nkwq OR kpj -> z05\nx00 OR x03 -> fst\ntgd XOR rvg -> z01\nvdt OR tnw -> bfw\nbfw AND frj -> z10\nffh OR nrd -> bqk\ny00 AND y03 -> djm\ny03 OR y00 -> psh\nbqk OR frj -> z08\ntnw OR fst -> frj\ngnj AND tgd -> z11\nbfw XOR mjb -> z00\nx03 OR x00 -> vdt\ngnj AND wpb -> z02\nx04 AND y00 -> kjc\ndjm OR pbm -> qhw\nnrd AND vdt -> hwm\nkjc AND fst -> rvg\ny04 OR y02 -> fgs\ny01 AND x02 -> pbm\nntg OR kjc -> kwq\npsh XOR fgs -> tgd\nqhw XOR tgd -> z09\npbm OR djm -> kpj\nx03 XOR y03 -> ffh\nx00 XOR y04 -> ntg\nbfw OR bqk -> z06\nnrd XOR fgs -> wpb\nfrj XOR qhw -> z04\nbqk OR frj -> z07\ny03 OR x01 -> nrd\nhwm AND bqk -> z03\ntgd XOR rvg -> z12\ntnw OR pbm -> gnj\n
