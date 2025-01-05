use std::collections::{HashMap, HashSet};
use crate::solutions::day24::types::{Equation, Pair};

pub fn get_pairs<'i>(
    options_a: impl Iterator<Item = &'i str> + ExactSizeIterator + Copy,
    options_b: impl Iterator<Item = &'i str> + ExactSizeIterator + Copy,
) -> HashSet<Pair<'i>> {
    let mut output = HashSet::with_capacity(options_a.len() * options_b.len());
    for a in options_a {
        for b in options_b {
            let pair = Pair::new(a, b);
            output.insert(pair);
        }
    }

    output
}

pub fn swapped<'i>(pair: Pair<'i>, equations: &HashMap<&'i str, Equation<'i>>) -> Option<HashMap<&'i str, Equation<'i>>> {
    let mut output = equations.clone();
    let eq_a = output.remove(pair.a)?;
    let eq_b = output.remove(pair.b)?;

    output.insert(pair.a, eq_b);
    output.insert(pair.b, eq_a);

    Some(output)
}




// fn get_n_pairs<'i, 'b, 'c>(
//     n: usize,
//     necessary_options: HashSet<&'i str>,
//     all_options: HashSet<&'i str>,
// ) -> impl Iterator<Item = HashSet<Pair<'i>>> + use<'i> {
//     if n == 0 {
//         todo!()
//         // return None.into_iter();
//     }
//
//     let all_options = &all_options;
//     let necessary_options = &necessary_options;
//
//     let a_wires = if necessary_options.is_empty() {
//         &all_options
//     } else {
//         &necessary_options
//     };
//     let b_wires = if necessary_options.len() <= 1 {
//         &all_options
//     } else {
//         &necessary_options
//     };
//
//     // let mut output = Vec::new();
//     let mut visited_pairs = HashSet::new();
//
//     let raw_pairs_iter = a_wires
//         .iter()
//         .flat_map(|&a| b_wires.iter().map(|&b| Pair::new(a, b)));
//
//     let output = raw_pairs_iter.flat_map(move |pair| {
//         if pair.a == pair.b || !visited_pairs.insert(pair.clone()) {
//             todo!();
//             // return None;
//         }
//
//         let mut next_necessary_options = necessary_options.clone();
//         let mut next_all_options = all_options.clone();
//         for set in [&mut next_necessary_options, &mut next_all_options] {
//             set.remove(pair.a);
//             set.remove(pair.b);
//         }
//
//         let mut set_of_pairs = get_n_pairs(n - 1, next_necessary_options, next_all_options);
//
//         let pair = pair.clone();
//         set_of_pairs.map(move |mut pairs| {
//             pairs.insert(pair);
//             pairs
//         })
//     });
//
//     output
// }
//
// #[test]
// fn set_of_pairs_works() {
//     let set_of_pairs = || {
//         get_n_pairs(
//             2,
//             &HashSet::from(["a", "b", "c"]),
//             &HashSet::from(["a", "b", "c", "d", "e"]),
//         )
//     };
//
//     let expected = Vec::from([
//         HashSet::from([Pair::new("a", "b"), Pair::new("c", "d")]),
//         HashSet::from([Pair::new("a", "b"), Pair::new("c", "e")]),
//         HashSet::from([Pair::new("a", "c"), Pair::new("b", "d")]),
//         HashSet::from([Pair::new("a", "c"), Pair::new("b", "e")]),
//         HashSet::from([Pair::new("b", "c"), Pair::new("a", "d")]),
//         HashSet::from([Pair::new("b", "c"), Pair::new("a", "e")]),
//     ]);
//
//     assert_eq!(expected.len(), set_of_pairs().count());
//     for pairs in set_of_pairs() {
//         assert!(expected.contains(&pairs));
//     }
// }
