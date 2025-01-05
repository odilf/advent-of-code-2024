use std::collections::HashMap;

use winnow::{
    ascii::{newline, space1},
    combinator::{alt, separated, trace},
    error::StrContext,
    seq,
    token::take_while,
    PResult, Parser as _,
};

use crate::solutions::day24::types::Gate;

use super::types::Equation;

pub fn parse(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Equation<'_>>) {
    fn wire<'a>(input: &mut &'a str) -> PResult<&'a str> {
        let parser = take_while(3, (b'a'..=b'z', b'0'..=b'9'));
        trace("wire", parser).parse_next(input)
    }

    fn initial_value<'a>(input: &mut &'a str) -> PResult<(&'a str, bool)> {
        let parser = seq!((
            wire,
            _: ":",
            _: space1,
            alt(['0', '1']).map(|c| c == '1')
        ));
        trace("initial_value", parser).parse_next(input)
    }

    fn initial_values<'a>(input: &mut &'a str) -> PResult<HashMap<&'a str, bool>> {
        trace(
            "initial_values",
            separated(0.., initial_value, newline).map(|v: HashMap<_, _>| v),
        )
        .parse_next(input)
    }

    fn gate(input: &mut &str) -> PResult<Gate> {
        let parser = alt((
            "AND".map(|_| Gate::And),
            "OR".map(|_| Gate::Or),
            "XOR".map(|_| Gate::Xor),
        ));

        trace("gate", parser)
            .context(StrContext::Label("Couldn't parse gate"))
            .parse_next(input)
    }

    fn equation<'a>(input: &mut &'a str) -> PResult<(&'a str, Equation<'a>)> {
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
        .map(|(a, gate, b, result)| (result, Equation { a, b, gate }));
        trace("operator", parser).parse_next(input)
    }

    fn equations<'a>(input: &mut &'a str) -> PResult<HashMap<&'a str, Equation<'a>>> {
        separated(0.., equation, newline).parse_next(input)
    }

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
}
