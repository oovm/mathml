use crate::{helpers::safe_html_str, MathML};
use std::fmt::{Display, Formatter};

mod display;

/// The [`<mi>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mi) element indicates that the content should be rendered as an identifier such as function names, variables or symbolic constants.
///
/// You can also have arbitrary text in it to mark up terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathIdentifier {
    identifier: String,
    variant: FontVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathText {
    is_string: bool,
    text: String,
}

// noinspection SpellCheckingInspection
/// Font variant for [`<mi>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mi),
/// used as [`mathvariant`](https://developer.mozilla.org/en-US/docs/Web/MathML/Global_attributes/mathvariant) attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontVariant {
    /// <math><mi mathvariant="normal">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Normal,
    /// <math><mi mathvariant="italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Italic,
    /// <math><mi mathvariant="bold">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Bold,
    /// <math><mi mathvariant="bold-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    BoldItalic,
    /// <math><mi mathvariant="double-struck">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    DoubleStruck,
    /// <math><mi mathvariant="bold-fraktur">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    BoldFraktur,
    /// <math><mi mathvariant="script">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Script,
    /// <math><mi mathvariant="bold-script">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    BoldScript,
    /// <math><mi mathvariant="fraktur">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Fraktur,
    /// <math><mi mathvariant="sans-serif">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    SansSerif,
    /// <math><mi mathvariant="bold-sans-serif">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    BoldSansSerif,
    /// <math><mi mathvariant="sans-serif-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    SansSerifItalic,
    /// <math><mi mathvariant="sans-serif-bold-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    SansSerifBoldItalic,
    /// <math><mi mathvariant="monospace">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Monospace,
}

impl MathIdentifier {
    /// Creates a new [`MathIdentifier`] with the given [`FontVariant`].
    pub fn new<S>(text: S, variant: FontVariant) -> Self
    where
        S: ToString,
    {
        Self { identifier: text.to_string(), variant }
    }
    /// Creates a new [`MathIdentifier`] with the [`FontVariant::Normal`] variant.
    pub fn normal<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { identifier: text.to_string(), variant: FontVariant::Normal }
    }
    /// Creates a new [`MathIdentifier`] with the [`FontVariant::Italic`] variant.
    pub fn italic<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { identifier: text.to_string(), variant: FontVariant::Italic }
    }
    /// Gets the font variant of the identifier.
    pub fn get_variant(&self) -> FontVariant {
        self.variant
    }
    /// Gets the identifier of the identifier.
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

impl MathText {
    /// Creates a new [`MathText`] with the given [`FontVariant`].
    pub fn text<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { text: text.to_string(), is_string: false }
    }
    /// Creates a new [`MathText`] with the [`FontVariant::Normal`] variant.
    pub fn string<S>(text: S) -> Self
    where
        S: ToString,
    {
        Self { text: text.to_string(), is_string: true }
    }
}

impl MathML {
    /// Creates a new [`MathIdentifier`] with the [`FontVariant::Italic`] variant.
    pub fn identifier<S>(text: S) -> MathIdentifier
    where
        S: ToString,
    {
        MathIdentifier::italic(text)
    }
    /// Creates a new [`MathIdentifier`] with the [`FontVariant::Normal`] variant.
    pub fn text<S>(text: S) -> MathText
    where
        S: ToString,
    {
        MathText::text(text)
    }
    /// Creates a new [`MathText`] with the [`FontVariant::Normal`] variant.
    pub fn string<S>(text: S) -> MathText
    where
        S: ToString,
    {
        MathText::string(text)
    }
}
