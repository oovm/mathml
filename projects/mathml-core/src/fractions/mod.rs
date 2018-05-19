use crate::MathML;
use std::fmt::{Display, Formatter};

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
    Thin,
    Medium,
    Thick,
    Length(u8),
}

impl Default for LineThickness {
    fn default() -> Self {
        LineThickness::Medium
    }
}

impl MathFraction {
    pub fn new<N, D>(numerator: N, denominator: D) -> Self
    where
        N: Into<MathML>,
        D: Into<MathML>,
    {
        Self { numerator: numerator.into(), denominator: denominator.into(), line_thickness: Default::default() }
    }
    pub fn with_thickness<T>(mut self, line_thickness: T) -> Self
    where
        T: Into<LineThickness>,
    {
        self.line_thickness = line_thickness.into();
        self
    }
}
