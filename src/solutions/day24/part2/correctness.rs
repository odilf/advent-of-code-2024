use std::collections::{HashMap, HashSet};

use rand::random;

use crate::solutions::day24::types::Equation;

use super::NUM_BITS;

pub fn compute_wire_value<'i>(
    wire: &'i str,
    equations: &HashMap<&'i str, Equation<'i>>,
    values: &mut HashMap<&'i str, bool>,
    pending: &mut HashSet<&'i str>,
) -> Option<bool> {
    if let Some(value) = values.get(wire) {
        return Some(*value);
    }

    if !pending.insert(wire) {
        return None;
    }

    let eq = equations.get(wire)?;
    let a = compute_wire_value(eq.a, equations, values, pending)?;
    let b = compute_wire_value(eq.b, equations, values, pending)?;

    let result = eq.gate.compute(a, b);
    values.insert(wire, result);
    pending.remove(wire);

    Some(result)
}

pub fn is_adder_up_to<'i>(
    bit: usize,
    equations: &HashMap<&'i str, Equation<'i>>,
    inputs: &[[String; 2]],
    outputs: &[String],
) -> bool {
    if bit == 0 {
        return true;
    }

    let is_adder_up_to = |bit| is_adder_up_to(bit, equations, inputs, outputs);

    if !is_adder_up_to(bit - 1) {
        println!("Bit {bit} fails because previous bits are not adders");
        return false;
    };

    const TRUTH_TABLE: [(bool, bool); 4] =
        [(false, false), (true, false), (false, true), (true, true)];

    for (x, y) in TRUTH_TABLE {
        for carry in [false, true] {
            let mut values = (0..=bit)
                .flat_map(|i| {
                    let (x_n, y_n) = if i == bit - 1 {
                        (carry, carry)
                    } else if i == bit {
                        (x, y)
                    } else {
                        (false, false)
                    };

                    [(inputs[i][0].as_str(), x_n), (inputs[i][1].as_str(), y_n)]
                })
                .collect::<HashMap<&str, bool>>();

            let mut pending = HashSet::new();
            let Some(z) = compute_wire_value(&outputs[bit], equations, &mut values, &mut pending)
            else {
                println!("Bit {bit} fails z depends on other bits");
                return false;
            };

            if ((x ^ y) ^ carry) != z {
                println!("Bit {bit} fails because (({x} ^ {y}) ^ {carry}) != {z}");
                return false;
            }
        }
    }

    true
}

fn calculate_sum<'i>(
    equations: &HashMap<&'i str, Equation<'i>>,
    input_wires: &[[String; 2]],
    input_values: &[[bool; 2]],
    outputs: &[String],
) -> Vec<Option<bool>> {
    let mut values = input_wires
        .iter()
        .zip(input_values.iter())
        .flat_map(|([x_wire, y_wire], [x, y])| {
            let x = (x_wire.as_str(), *x);
            let y = (y_wire.as_str(), *y);

            [x, y]
        })
        .collect();

    outputs
        .iter()
        .map(|output_wire| {
            let mut pending = HashSet::new();
            compute_wire_value(output_wire.as_str(), equations, &mut values, &mut pending)
        })
        .collect()
}

pub fn get_first_incorrect_bit<'i>(
    equations: &HashMap<&'i str, Equation<'i>>,
    inputs: &[[String; 2]],
    outputs: &[String],
) -> Option<usize> {
    const SAMPLE_SIZE: usize = 100;

    let mut output = None;
    for _ in 0..SAMPLE_SIZE {
        let input_values = inputs
            .iter()
            .map(|_| [random::<bool>(), random::<bool>()])
            .collect::<Vec<_>>();

        let (x, y) = input_values
            .iter()
            .enumerate()
            .map(|(i, [x, y])| ((*x as usize) << i, (*y as usize) << i))
            .fold((0, 0), |(x0, y0), (x, y)| (x0 + x, y0 + y));
        let z = x + y;

        let zs = calculate_sum(equations, inputs, &input_values, outputs);

        {
            let obtained_z = zs
                .iter()
                .enumerate()
                .map(|(i, &z_bit)| match z_bit {
                    Some(z_bit) => (z_bit as u64) << i,
                    None => 0,
                })
                .sum::<u64>();

            // dbg!(&zs.len());
            // dbg!(64 - z.leading_zeros());

            // println!("z:        {z:#048b}");
            // println!("obtained: {obtained_z:#048b}");
        }
        for (i, z_bit) in zs.iter().enumerate() {
            let expected_z_bit = (z >> i) & 1 == 1;
            let is_correct = z_bit.map(|z_bit| expected_z_bit == z_bit).unwrap_or(false);

            // if !is_correct {
            //     println!("Bit {i} is not correct. Expected {expected_z_bit}, got {z_bit:?}");
            // }

            if !is_correct {
                let update = match output {
                    None => true,
                    Some(first_incorrect) if i < first_incorrect => true,
                    _ => false,
                };

                if update {
                    output = Some(i);
                    // println!("Updating worst case to {i}");
                    break;
                }
            }
        }
    }

    output
}

pub fn is_adder<'i>(
    equations: &HashMap<&'i str, Equation<'i>>,
    inputs: &[[String; 2]],
    outputs: &[String],
) -> bool {
    is_adder_up_to(NUM_BITS, equations, inputs, outputs)
}
