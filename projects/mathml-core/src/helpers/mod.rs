#![allow(non_snake_case)]
#![allow(unused_variables)]
#![doc = include_str!("readme.md")]

mod matrix;

use crate::{blocks::MathStyle, LineThickness, MathFraction, MathML, MathMultiScript, MathRow, MathTable};

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

// noinspection SpellCheckingInspection
/// Renders a display style fraction.
///
/// # Input
///
/// ```tex
/// \begin{vmatrix} a & b \\ c & d \end{vmatrix}
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn dfrac<N, D>(numerator: N, denominator: D) -> MathML
where
    N: Into<MathML>,
    D: Into<MathML>,
{
    MathStyle::display(frac(numerator, denominator)).into()
}
// noinspection SpellCheckingInspection
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
pub fn cfrac(numerator: MathML, denominator: MathML) -> MathML {
    todo!()
}
// noinspection SpellCheckingInspection
/// Renders binomial coefficient as `\binom{n}{k}`.
///
/// # Input
///
/// ```
/// # use mathml_core::helpers::binom;
/// binom('n', 'k');
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn binom<N, D>(numerator: N, denominator: D) -> MathML
where
    N: Into<MathML>,
    D: Into<MathML>,
{
    MathFraction::new(numerator.into(), denominator.into()).with_thickness(LineThickness::Length(0)).into()
}

// noinspection SpellCheckingInspection
/// Renders binomial coefficient as `C_{n}^{k}`.
///
/// # Input
///
/// ```
/// # use mathml_core::helpers::cbinom;
/// cbinom('n', 'k');
/// ```
///
/// # Output
#[doc = include_str!("vmatrix.xml")]
pub fn cbinom<N, D>(numerator: N, denominator: D) -> MathML
where
    N: Into<MathML>,
    D: Into<MathML>,
{
    MathMultiScript::sub_super_script(MathML::identifier("C"), numerator.into(), denominator.into()).into()
}

// noinspection SpellCheckingInspection
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
/// # use mathml_core::{helpers::isotope, MathFenced};
/// MathFenced::parentheses(vec![
///     isotope("H", 1, None).into(),
///     isotope("H", 2, Some(2)).into(),
///     isotope("H", 3, None).into(),
/// ]);
/// ```
///
/// # Output
#[doc = include_str!("isotope.xml")]
pub fn isotope(symbol: &str, mass_number: usize, atomic_number: Option<usize>) -> MathMultiScript {
    let ld = match atomic_number {
        Some(s) => vec![s.into()],
        None => vec![],
    };
    MathMultiScript::new(MathML::identifier(symbol), vec![mass_number.into()], ld, vec![], vec![])
}

/// Build a isotopic symbol.
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

/// Build a isotopic symbol.
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
