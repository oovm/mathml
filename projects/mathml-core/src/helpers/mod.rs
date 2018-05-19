#![allow(non_snake_case)]
#![allow(unused_variables)]
#![doc = include_str!("readme.md")]

mod matrix;

use crate::{blocks::MathStyle, LineThickness, MathFenced, MathFraction, MathML, MathMultiScript, MathRow, MathTable};

pub use self::matrix::*;

/// Build a normal fraction.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn frac<N, D>(numerator: N, denominator: D) -> MathML
where
    N: Into<MathML>,
    D: Into<MathML>,
{
    MathFraction::new(numerator, denominator).into()
}

/// Renders a matrix with vertical bars.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
// noinspection SpellCheckingInspection
pub fn dfrac<N, D>(numerator: N, denominator: D) -> MathML
where
    N: Into<MathML>,
    D: Into<MathML>,
{
    MathStyle::display(frac(numerator, denominator)).into()
}

/// Renders a matrix with vertical bars.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
// noinspection SpellCheckingInspection
pub fn cfrac(numerator: MathML, denominator: MathML) -> MathML {
    todo!()
}

/// Renders a matrix with vertical bars.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn binom(numerator: MathML, denominator: MathML) -> MathML {
    MathFraction::new(numerator, denominator).with_thickness(LineThickness::Length(0)).into()
}

/// Renders a matrix with vertical bars.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
// noinspection SpellCheckingInspection
pub fn cbinom(numerator: MathML, denominator: MathML) -> MathML {
    MathMultiScript::sub_super_script(MathML::identifier("C"), numerator, denominator).into()
}
/// Renders a matrix with vertical bars.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn legendre_symbols(numerator: MathML, denominator: MathML) -> MathML {
    todo!()
}

/// Build a isotopic symbol.
///
/// # Input
///
/// ```
/// # use mathml_core::{helpers::isotope,MathFenced};
/// MathFenced::parentheses(vec![
///     isotope("H", 1, None),
///     isotope("H", 2, None),
///     isotope("H", 3, None),
/// ]);
/// ```
///
/// # Output
#[doc = include_str!("isotope.xml")]
pub fn isotope(symbol: &str, mass_number: usize, atomic_number: Option<usize>) -> MathMultiScript {
    let ld = match atomic_number {
        Some(s) => {
            vec![s.into()]
        }
        None => {
            vec![]
        }
    };
    MathMultiScript::new(MathML::identifier(symbol), vec![mass_number.into()], ld, vec![], vec![])
}

#[inline(always)]
pub fn safe_html_char<W>(writer: &mut W, c: char) -> std::fmt::Result
where
    W: std::fmt::Write,
{
    match c {
        '<' => writer.write_str("&lt;"),
        '>' => writer.write_str("&gt;"),
        '&' => writer.write_str("&amp;"),
        '"' => writer.write_str("&quot;"),
        '\'' => writer.write_str("&apos;"),
        _ => writer.write_char(c),
    }
}

#[inline(always)]
pub fn safe_html_str<W>(writer: &mut W, s: &str) -> std::fmt::Result
where
    W: std::fmt::Write,
{
    for c in s.chars() {
        safe_html_char(writer, c)?;
    }
    Ok(())
}
