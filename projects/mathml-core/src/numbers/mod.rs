use crate::{MathElement, MathML};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

mod constructors;
mod display;

/// The [`<mn>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mn) element represents a numeric literal which is normally a sequence of digits with a possible separator (a dot or a comma).
///
/// However, it is also allowed to have arbitrary text in it which is actually a numeric quantity, for example "eleven".
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MathNumber {
    number: String,
}

/// The [`<mfrac>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mfrac) element is used to display fractions.
///
/// It can also be used to mark up fraction-like objects such as binomial coefficients and Legendre symbols.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MathFraction {
    numerator: MathML,
    denominator: MathML,
    line_thickness: LineThickness,
}

// noinspection SpellCheckingInspection
/// Line thickness for [`<mfrac>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mfrac),
/// used as [`linethickness`](https://developer.mozilla.org/en-US/docs/Web/MathML/Values#legacy_mathml_lengths) attribute.
///
/// ## Polyfill
///
/// We provide a polyfill for this attribute, which supports deprecated values `thin`, `medium`, `thick` and `length`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LineThickness {
    /// `1rem`
    Thin,
    /// `1rem`
    Medium,
    /// `1rem`
    Thick,
    /// `1rem`
    Length(u8),
}

/// The [`<merror>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/merror) element is used to display contents as error messages.
///
/// The intent of this element is to provide a standard way for programs that generate MathML from other input to report syntax errors.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MathError {
    message: String,
}
