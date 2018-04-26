use crate::MathML;
use std::fmt::{Display, Formatter};

mod constructors;
mod display;

#[derive(Clone, Debug, PartialEq)]
pub struct MathOperator {
    operator: String,
}

/// The [`<mroot>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mroot) or [`<msqrt>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msqrt) element is used to display roots with an explicit index.
#[derive(Clone, Debug, PartialEq)]
pub struct MathSqrt {
    base: MathML,
    surd: Option<MathML>,
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
