use color_eyre::eyre;
use itertools::Itertools as _;
use std::{collections::VecDeque, fmt::Write as _, str::FromStr};
use strum::{EnumString, FromRepr};
use winnow::{
    ascii::{digit1, multispace0},
    combinator::{preceded, separated},
    PResult, Parser as _,
};

/// The quote-on-quote "`usize`" for the machine. Only 3 bits.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Usize {
    bits: [bool; 3],
}

impl std::fmt::Debug for Usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            return write!(f, "{}", self.value());
        }

        for bit in self.bits {
            if bit {
                f.write_char('1')?;
            } else {
                f.write_char('0')?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for Usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Usize {
    pub fn value(&self) -> u8 {
        self.bits
            .iter()
            .enumerate()
            .map(|(i, &bit)| (bit as u8) * (1 << i))
            .sum()
    }

    fn iter() -> impl Iterator<Item = Self> + Clone {
        (0..8).map(|v| Usize::try_from(v).unwrap())
    }
}

impl TryFrom<u8> for Usize {
    type Error = eyre::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 1 << 3 {
            eyre::bail!("Number doesn't fit in 3 bits: {value}");
        }

        let bits: [bool; 3] = [0, 1, 2].map(|i| (value & (1 << i)) != 0);

        Ok(Self { bits })
    }
}

impl FromStr for Usize {
    type Err = eyre::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(u8::from_str(s)?.try_into()?)
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, FromRepr)]
enum Instruction {
    #[strum(serialize = "adv")]
    DivisionA = 0,

    #[strum(serialize = "bxl")]
    BitwiseBXorOp,

    #[strum(serialize = "bst")]
    Modulo8,

    #[strum(serialize = "jnz")]
    JumpNotZero,

    #[strum(serialize = "bxc")]
    BitwiseBXorC,

    #[strum(serialize = "out")]
    Output,

    #[strum(serialize = "bdv")]
    DivisionB,

    #[strum(serialize = "cdv")]
    DivisionC,
}

impl Instruction {
    pub fn execute(&self, operand: Usize, machine: &mut Machine) -> Option<Usize> {
        use Instruction as I;

        // println!("reg_a: {:b}", machine.reg_a);
        // println!("reg_b: {:b}", machine.reg_b);
        // println!("reg_c: {:b}", machine.reg_c);
        // println!("Running {self:?}({operand}), combo is {:b}", machine.combo(operand));

        match self {
            I::DivisionA => machine.reg_a >>= machine.combo(operand),
            I::DivisionB => machine.reg_b = machine.reg_a >> machine.combo(operand),
            I::DivisionC => machine.reg_c = machine.reg_a >> machine.combo(operand),

            I::BitwiseBXorOp => machine.reg_b ^= operand.value() as u64,
            I::BitwiseBXorC => machine.reg_b ^= machine.reg_c,

            I::Modulo8 => machine.reg_b = machine.combo(operand) % 8,
            I::JumpNotZero => {
                if machine.reg_a != 0 {
                    machine.instruction_pointer = operand.value() as usize
                }
            }
            I::Output => return Some(Usize::try_from((machine.combo(operand) % 8) as u8).unwrap()),
        }

        None
    }
}

impl From<Usize> for Instruction {
    fn from(opcode: Usize) -> Self {
        Self::from_repr(opcode.value() as usize).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Machine {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    instruction_pointer: usize,
}

impl Machine {
    #[must_use]
    pub fn run<'a, 'b>(&'a mut self, program: &'b [Usize]) -> MachineIter<'a, 'b> {
        MachineIter {
            machine: self,
            program,
        }
    }

    pub fn combo(&self, operand: Usize) -> u64 {
        match operand.value() {
            val @ 0..=3 => val as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Invalid combo operand"),
            _ => unreachable!(),
        }
    }
}

struct MachineIter<'a, 'b> {
    machine: &'a mut Machine,
    program: &'b [Usize],
}

impl MachineIter<'_, '_> {
    fn try_next(&mut self) -> Option<Option<Usize>> {
        let opcode = *self.program.get(self.machine.instruction_pointer)?;
        self.machine.instruction_pointer += 1;

        let instruction = Instruction::from(opcode);

        let operand = *self.program.get(self.machine.instruction_pointer)?;
        self.machine.instruction_pointer += 1;

        // println!(
        //     "Running {instruction:?} with {operand} and reg_a={}",
        //     self.machine.reg_a
        // );

        let output = instruction.execute(operand, &mut self.machine);
        // dbg!(self.machine.instruction_pointer);

        Some(output)
    }
}

impl Iterator for MachineIter<'_, '_> {
    type Item = Usize;
    fn next(&mut self) -> Option<Self::Item> {
        // println!("{:#?}", &self.machine);

        // println!("reg_a: {:b}", self.machine.reg_a);
        // println!("reg_b: {:b}", self.machine.reg_b);
        // println!("reg_c: {:b}", self.machine.reg_c);

        loop {
            if let Some(output) = self.try_next()? {
                return Some(output);
            }
        }
    }
}

#[elvish::solution(day = 17)]
fn part1(mut input: &str) -> String {
    let (mut machine, program) = parse(&mut input).unwrap();

    let mut output = machine.run(&program);

    output.join(",")
}

// 601201576113503: Too high

#[elvish::solution(day = 17, example = 117440)]
fn part2(mut input: &str) -> u64 {
    let (initial_machine, program) = parse(&mut input).unwrap();
    // println!("\n\n\n\n--- START ---\n\n\n\n");
    //
    // let initial_machine = Machine {
    //     reg_b: 0,
    //     ..initial_machine
    // };
    //
    // // for reg_a in 0..(1 << 3 * 4) {
    // for reg_a in [dbg!(0b101_010 + 0b000_000_000 + 0b101_000_000_000)] {
    //     let mut machine = Machine {
    //         reg_a,
    //         ..initial_machine
    //     };
    //     let output = machine.run(&program).collect::<Vec<_>>();
    //
    //     dbg!(&program, &output);
    //
    //     if output.as_slice() == &program[0..3] {
    //         return reg_a;
    //     }
    // }
    //
    // panic!();

    // let mut queue = Usize::iter()
    //     .flat_map(|bits| {
    //         Usize::iter()
    //             .flat_map(move |bits2| {
    //                 Usize::iter().map(move |bits3| {
    //                     let a = (bits.value() as u64) << 6;
    //                     let b = (bits2.value() as u64) << 3;
    //                     let c = (bits3.value() as u64) << 0;
    //
    //                     a + b + c
    //                 })
    //             })
    //             .map(|reg_a| (reg_a, 0))
    //     })
    //     .collect::<VecDeque<_>>();
    let mut queue = (0..0b111_111_111_111_111)
        .map(|reg_a| (reg_a, 2))
        .collect::<VecDeque<_>>();

    let mut reg_a_output = Vec::new();
    while let Some((reg_a, up_to)) = queue.pop_front() {
        // dbg!(reg_a, up_to);
        // if reg_a.checked_ilog2().is_some_and(|log| log / 3 - 1 != up_to as u32) {
        //     println!("WARN");
        //     dbg!(reg_a, up_to);
        // };

        let mut machine = Machine {
            reg_a: reg_a
                + 1u64
                    .checked_shl(3 * (up_to + 3) as u32 + 2)
                    .expect("not overflow"),
            // reg_a,
            ..initial_machine
        };

        let output = machine.run(&program).take(up_to).collect::<Vec<_>>();

        if output.len() < up_to {
            panic!();
        }
        if output == program {
            println!("CANDIDATE! {reg_a}");
            reg_a_output.push(reg_a);
            continue;
        }

        if output.as_slice() != &program[0..up_to] {
            continue;
        }

        println!("{reg_a} produces input up to {up_to}. {output:?}, {program:?}");

        if up_to > 14 {
            for next_bits in 0u64..0b111 {
                let shift = 3 * (up_to + 4);
                let reg_a_next = reg_a + (next_bits.checked_shl(shift as u32).unwrap());
                // dbg!(reg_a_next);
                queue.push_back((reg_a_next, up_to + 1));
            }
            continue;
        }

        for next_bits in 0u64..0b111_111 {
            let shift = 3 * (up_to + 4);
            let reg_a_next = reg_a + (next_bits.checked_shl(shift as u32).unwrap());
            // dbg!(reg_a_next);
            queue.push_back((reg_a_next, up_to + 2));
        }
    }

    reg_a_output.into_iter().filter(|&reg_a| {
        dbg!(reg_a);

        let mut machine = Machine {
            reg_a,
            ..initial_machine
        };

        let output = machine.run(&program).collect::<Vec<_>>();
        output == program
    }).min().unwrap()
}

elvish::example!(
    part1: "
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    ",

    part2: "
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    ",
);

#[test]
fn example_part1() {
    let solution = part1(EXAMPLE_PART1);
    assert_eq!(solution.as_str(), "4,6,3,5,6,3,5,2,1,0")
}

#[test]
fn example_instructions() {
    // If register C contains 9, the program 2,6 would set register B to 1.
    let program = [2, 6].map(|i| i.try_into().unwrap());
    let mut machine = Machine {
        reg_c: 9,
        ..Default::default()
    };

    machine.run(&program).count();

    assert_eq!(machine.reg_b, 1);

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    let output = Machine {
        reg_a: 10,
        ..Default::default()
    }
    .run(&[5, 0, 5, 1, 5, 4].map(|i| i.try_into().unwrap()))
    .join(",");

    assert_eq!(output.as_str(), "0,1,2");

    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    let output = Machine {
        reg_a: 2024,
        ..Default::default()
    }
    .run(&[0, 1, 5, 4, 3, 0].map(|i| i.try_into().unwrap()))
    .join(",");

    assert_eq!(output.as_str(), "4,2,5,6,7,7,7,7,3,1,0");

    // If register B contains 29, the program 1,7 would set register B to 26.
    let program = [1, 7].map(|i| i.try_into().unwrap());
    let mut machine = Machine {
        reg_b: 29,
        ..Default::default()
    };
    machine.run(&program).count();

    assert_eq!(machine.reg_b, 26);

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354
    let program = [4, 0].map(|i| i.try_into().unwrap());
    let mut machine = Machine {
        reg_b: 2024,
        reg_c: 43690,
        ..Default::default()
    };

    machine.run(&program).count();

    assert_eq!(machine.reg_b, 44354);
}

fn parse(input: &mut &str) -> PResult<(Machine, Vec<Usize>)> {
    let mut parser = winnow::seq!(Machine {
        reg_a:  preceded("Register A: ", digit1.parse_to()),
        _: multispace0,
        reg_b:  preceded("Register B: ", digit1.parse_to()),
        _: multispace0,
        reg_c:  preceded("Register C: ", digit1.parse_to()),
        _: multispace0,
        ..Default::default()
    });

    let machine = parser.parse_next(input)?;
    let program = preceded(
        "Program: ",
        separated(0.., digit1.parse_to::<Usize>(), ',').map(|l: Vec<_>| l),
    )
    .parse_next(input)?;

    Ok((machine, program))
}
