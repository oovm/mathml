use std::fmt::{Display, Formatter};
mod display;
use crate::MathML;
use html_escape::encode_text;

/// math identifier, `<mi>`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathIdentifier {
    identifier: String,
    variant: MathVariant,
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
    pub fn new<S>(text: S, variant: MathVariant) -> Self
    where
        S: ToString,
    {
        Self { identifier: text.to_string(), variant }
    }
    pub fn normal<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { identifier: text.to_string(), variant: MathVariant::Normal }
    }
    pub fn italic<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { identifier: text.to_string(), variant: MathVariant::Italic }
    }
    pub fn get_variant(&self) -> MathVariant {
        self.variant
    }
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
