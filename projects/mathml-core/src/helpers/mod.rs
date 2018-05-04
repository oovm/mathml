use crate::{LineThickness, MathFraction, MathML, MathMultiScript};

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
