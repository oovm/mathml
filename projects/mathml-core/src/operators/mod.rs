use crate::MathML;
use std::fmt::{Display, Formatter};

mod display;
#[derive(Debug, Clone, PartialEq)]
pub struct MathOperator {
    operator: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathBinary {
    kind: MathBinaryKind,
    lhs: MathML,
    rhs: MathML,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MathBinaryKind {
    SuperScript,
    SubScript,
}

impl From<MathOperator> for MathML {
    fn from(value: MathOperator) -> Self {
        MathML::Operator(Box::new(value))
    }
}

impl MathOperator {
    pub fn new<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { operator: text.to_string() }
    }
}

impl From<MathBinary> for MathML {
    fn from(value: MathBinary) -> Self {
        MathML::Binary(Box::new(value))
    }
}

impl MathBinary {
    pub fn super_script(lhs: MathML, rhs: MathML) -> Self {
        Self { kind: MathBinaryKind::SuperScript, lhs, rhs }
    }
    pub fn sub_script(lhs: MathML, rhs: MathML) -> Self {
        Self { kind: MathBinaryKind::SubScript, lhs, rhs }
    }
}
