use crate::{MathElement, MathML};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};
mod constructors;
mod display;

/// The [`<mfrac>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mfrac) element is used to display fractions.
///
/// It can also be used to mark up fraction-like objects such as binomial coefficients and Legendre symbols.
#[derive(Clone, Debug, PartialEq)]
pub struct MathFraction {
    numerator: MathML,
    denominator: MathML,
    line_thickness: LineThickness,
}

// noinspection SpellCheckingInspection
/// Line thickness for [`<mmfraci>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mfrac),
/// used as [`linethickness`](https://developer.mozilla.org/en-US/docs/Web/MathML/Global_attributes/mathvariant) attribute.
///
/// ## Polyfill
///
/// We provide a polyfill for this attribute, which supports deprecated values `thin`, `medium`, `thick` and `length`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineThickness {
    /// <math><mfrac linethickness="thin">1</mfrac></math>
    Thin,
    /// <math><mfrac linethickness="medium">1</mfrac></math>
    Medium,
    /// <math><mfrac linethickness="thick">1</mfrac></math>
    Thick,
    /// <math><mfrac linethickness="length">1</mfrac></math>
    Length(u8),
}
