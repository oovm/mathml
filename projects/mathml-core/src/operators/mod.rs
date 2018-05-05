use crate::MathML;
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

mod constructors;
mod display;

#[derive(Clone, Debug, PartialEq)]
pub struct MathOperator {
    operator: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MathSpace {
    attributes: BTreeMap<String, String>,
}

/// The [`<mroot>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mroot) or [`<msqrt>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msqrt) element is used to display roots with an explicit index.
#[derive(Clone, Debug, PartialEq)]
pub struct MathSqrt {
    base: MathML,
    surd: Option<MathML>,
}

/// The [`<mmultiscripts>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mmultiscripts) element is used to attach multiple subscripts and superscripts to a base.
#[derive(Clone, Debug, PartialEq)]
pub struct MathMultiScript {
    base: MathML,
    ru: Vec<MathML>,
    rd: Vec<MathML>,
    lu: Vec<MathML>,
    ld: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}

/// The [`<munderover>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/munderover) element is used to attach an accent or a limit under and over an expression.
#[derive(Clone, Debug, PartialEq)]
pub struct MathUnderOver {
    base: MathML,
    under: Option<MathML>,
    over: Option<MathML>,
    attributes: BTreeMap<String, String>,
}

/// The [`<mfenced>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mfenced) element provides the possibility to add custom opening and closing parentheses (such as brackets) and separators (such as commas or semicolons) to an expression.
///
/// ## Syntax
///
/// Since it is deprecated by the MathML standard, the syntax is not well defined.
#[derive(Clone, Debug, PartialEq)]
pub struct MathFenced {
    base: Vec<MathML>,
    open: char,
    close: char,
    separators: String,
}
