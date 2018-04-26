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
    pub fn new(numerator: MathML, denominator: MathML) -> Self {
        Self { numerator, denominator, line_thickness: Default::default() }
    }
    pub fn with_thickness<T>(mut self, line_thickness: T) -> Self
    where
        T: Into<LineThickness>,
    {
        self.line_thickness = line_thickness.into();
        self
    }
}

impl MathML {
    pub fn fraction(numerator: MathML, denominator: MathML) -> Self {
        MathFraction::new(numerator, denominator).into()
    }
    //  binomial coefficients and Legendre symbols

    //   <mrow>
    //     <mo>(</mo>
    //     <mfrac linethickness="0">
    //       <mi>n</mi>
    //       <mi>k</mi>
    //     </mfrac>
    //     <mo>)</mo>
    //   </mrow>
    // pub fn binomial(numerator: MathML, denominator: MathML) -> Self {
    //     let mut row = vec![];
    //     row.push(MathML::Op("("));
    //     row.push(MathML::fraction(numerator, denominator).with_thickness(LineThickness::Length(0)));
    //     row.push(MathML::mo(")"));
    //     row
    // }
}
