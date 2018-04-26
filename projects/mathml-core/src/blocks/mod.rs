use crate::MathML;
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};
mod constructors;
mod display;

/// <https://developer.mozilla.org/en-US/docs/Web/MathML/Element/math>
#[derive(Debug, Clone, PartialEq)]
pub struct MathRoot {
    children: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}

/// The [`<mphantom>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mphantom) element is rendered invisibly, but dimensions (such as height, width, and baseline position) are still kept.
#[derive(Debug, Clone, PartialEq)]
pub struct MathPhantom {
    inner: MathML,
}
