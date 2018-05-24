use crate::{
    helpers::{safe_html_char, safe_html_str},
    MathElement, MathML,
};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    iter::repeat,
};
mod constructors;
mod display;

/// The [`<mo>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mo) element represents an operator in a broad sense.
///
/// Besides operators in strict mathematical meaning, this element also includes "operators" like parentheses, separators like comma and semicolon, or "absolute value" bars.
#[derive(Clone, Debug, PartialEq)]
pub struct MathOperator {
    operator: String,
    attributes: BTreeMap<String, String>,
}

/// The [`<mspace>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mspace) element is used to display a blank space, whose size is set by its attributes.
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

/// The [`<mmultiscripts>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mmultiscripts) element is used to attach an arbitrary number of subscripts and superscripts to an expression at once
#[derive(Clone, Debug, PartialEq)]
pub struct MathMultiScript {
    base: MathML,
    ru: Vec<MathML>,
    rd: Vec<MathML>,
    lu: Vec<MathML>,
    ld: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}

/// The [`<munderover>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/munderover) element is used to attach accents or limits both under and over an expression.
#[derive(Clone, Debug, PartialEq)]
pub struct MathUnderOver {
    base: MathML,
    under: Option<MathML>,
    over: Option<MathML>,
    attributes: BTreeMap<String, String>,
}

/// The [`<mfenced>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mfenced) element provides the possibility to add custom opening and closing parentheses and separators to an expression.
///
/// ## Polyfill
///
/// Since it is deprecated by the MathML standard, we provide a polyfill to `<mrow>` for this element.
#[derive(Clone, Debug, PartialEq)]
pub struct MathFenced {
    base: Vec<MathML>,
    open: char,
    close: char,
    separators: String,
}
