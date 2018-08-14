use super::*;

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
    pub fn identifier<S>(text: S) -> Self
    where
        S: ToString,
    {
        MathIdentifier::italic(text).into()
    }
    /// Creates a new [`MathIdentifier`] with the [`FontVariant::Normal`] variant.
    pub fn text<S>(text: S) -> Self
    where
        S: ToString,
    {
        MathText::text(text).into()
    }
    /// Creates a new [`MathText`] with the [`FontVariant::Normal`] variant.
    pub fn string<S>(text: S) -> Self
    where
        S: ToString,
    {
        MathText::string(text).into()
    }
}
