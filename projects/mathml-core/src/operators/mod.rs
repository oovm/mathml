use crate::MathML;
use std::fmt::{Display, Formatter};

mod constructors;
mod display;

#[derive(Clone, Debug, PartialEq)]
pub struct MathOperator {
    operator: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MathFraction {
    numerator: MathML,
    denominator: MathML,
    line_thickness: Option<f32>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msub>
#[derive(Clone, Debug, PartialEq)]
pub struct MathSub {
    base: MathML,
    sub: MathML,
}

/// <https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msup>
#[derive(Clone, Debug, PartialEq)]
pub struct MathSup {
    base: MathML,
    sup: MathML,
}

/// <https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msubsup>
#[derive(Clone, Debug, PartialEq)]
pub struct MathSubSup {
    base: MathML,
    sub: MathML,
    sup: MathML,
}
