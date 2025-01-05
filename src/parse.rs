use std::str::FromStr;

use nom::{character::complete::digit1, Parser as _};
use nom_supreme::ParserExt as _;

pub fn digit<T>(input: &str) -> nom::IResult<&str, T> where T: FromStr {
    digit1.map_res(str::parse).parse(input)
}

