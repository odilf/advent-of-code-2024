use std::collections::HashMap;

use super::parse::parse;

#[elvish::solution(day = 24)]
pub fn part1(mut input: &str) -> u64 {
    // let (mut values, equations) = parse(&mut input);
    //
    // let target_wires = values
    //     .keys()
    //     .chain(
    //         equations
    //             .iter()
    //             .flat_map(|(eq, result)| [&eq.a, &eq.b, &result]),
    //     )
    //     .filter(|&wire| wire.starts_with("z"))
    //     .copied()
    //     .collect::<Vec<_>>();
    //
    // let wire_equations = {
    //     let mut wire_equations = HashMap::new();
    //     for &(equation, result) in &equations {
    //         wire_equations.insert(result, equation);
    //     }
    //
    //     wire_equations
    // };
    //
    // let mut output = 0;
    // for target_wire in target_wires {
        todo!("Backport part 1");
        // let value = find_result(target_wire, &mut values, &wire_equations);
        // let index = (target_wire[1] - '0') as u32 * 10 + (target_wire[2] - b'0') as u32;
        //
        // if value == true {
        //     output += 2u64.pow(index);
        // }
    // }
    //
    // output
}
