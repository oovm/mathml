#![allow(non_snake_case)]
#![allow(unused_variables)]

mod matrix;

use crate::{LineThickness, MathFraction, MathML, MathMultiScript, MathRow, MathTable};

pub use self::matrix::*;

pub fn frac(numerator: MathML, denominator: MathML) -> MathML {
    MathFraction::new(numerator, denominator).into()
}

// noinspection SpellCheckingInspection
pub fn dfrac(numerator: MathML, denominator: MathML) -> MathML {
    todo!()
}

// noinspection SpellCheckingInspection
pub fn cfrac(numerator: MathML, denominator: MathML) -> MathML {
    todo!()
}

pub fn binom(numerator: MathML, denominator: MathML) -> MathML {
    MathFraction::new(numerator, denominator).with_thickness(LineThickness::Length(0)).into()
}

// noinspection SpellCheckingInspection
pub fn cbinom(numerator: MathML, denominator: MathML) -> MathML {
    MathMultiScript::sub_super_script(MathML::identifier("C"), numerator, denominator).into()
}

pub fn legendre_symbols(numerator: MathML, denominator: MathML) -> MathML {
    todo!()
}

pub fn isotope(nucleus: usize, atomic_number: usize, mass_number: usize) -> MathML {
    todo!()
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
