use crate::MathML;
use std::fmt::{Display, Formatter};

mod constructors;
mod display;

/// The [`<mn>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mn) element represents a numeric literal which is normally a sequence of digits with a possible separator (a dot or a comma).
///
/// However, it is also allowed to have arbitrary text in it which is actually a numeric quantity, for example "eleven".
#[derive(Debug, Clone, PartialEq)]
pub struct MathNumber {
    number: String,
}
/// The [`<merror>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/merror) element is used to display fractions.
#[derive(Clone, Debug, PartialEq)]
pub struct MathError {
    message: String,
}
