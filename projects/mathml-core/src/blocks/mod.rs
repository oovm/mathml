use crate::{MathElement, MathIdentifier, MathML};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
};
mod constructors;
mod display;

/// <https://developer.mozilla.org/en-US/docs/Web/MathML/Element/math>
#[derive(Debug, Clone, PartialEq)]
pub struct MathRoot {
    children: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathFunction {
    name: String,
    body: Vec<MathML>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathRow {
    children: Vec<MathML>,
    has_grouping: bool,
}

/// The [`<mphantom>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mphantom) element is rendered invisibly, but dimensions (such as height, width, and baseline position) are still kept.
#[derive(Debug, Clone, PartialEq)]
pub struct MathPhantom {
    inner: MathML,
}

/// The [`<mspace>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mspace) element is used to insert space characters into a mathematical formula.
#[derive(Debug, Clone, PartialEq)]
pub struct MathStyle {
    base: MathML,
    attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathTable {
    stream: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}
