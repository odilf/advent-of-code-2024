use core::fmt;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gate {
    And,
    Xor,
    Or,
}

impl Gate {
    pub fn compute(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Equation<'a> {
    pub a: &'a str,
    pub b: &'a str,
    pub gate: Gate,
}

impl<'a> Equation<'a> {
    pub fn operands(&self) -> impl Iterator<Item = &'a str> {
        [self.a, self.b].into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pair<'i> {
    pub a: &'i str,
    pub b: &'i str,
}

impl<'i> Pair<'i> {
    pub fn new(mut a: &'i str, mut b: &'i str) -> Self {
        if a > b {
            mem::swap(&mut a, &mut b);
        }

        Self { a, b }
    }
}

impl fmt::Display for Pair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}, {} }}", self.a, self.b)
    }
}
