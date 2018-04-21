use crate::MathML;
use std::fmt::{Display, Formatter};

mod display;
#[derive(Debug, Clone, PartialEq)]
pub struct MathOperator {
    operator: String,
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
