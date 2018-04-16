use std::fmt::{Display, Formatter};
mod display;

/// math identifier, `<mi>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MathIdentifier {
    pub letter: char,
    pub variant: MathVariant,
}

/// mi mathvariant attribute
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathVariant {
    Normal,
    Italic,
    Bold,
    BoldItalic,
    DoubleStruck,
    BoldFraktur,
    Script,
    BoldScript,
    Fraktur,
    SansSerif,
    BoldSansSerif,
    SansSerifItalic,
    SansSerifBoldItalic,
    Monospace,
}

impl MathIdentifier {
    pub fn normal(letter: char) -> Self {
        Self { letter, variant: MathVariant::Normal }
    }
    pub fn italic(letter: char) -> Self {
        Self { letter, variant: MathVariant::Italic }
    }
}
