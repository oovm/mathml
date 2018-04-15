use super::*;

/// mi mathvariant attribute
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[rustfmt::skip]
impl Display for MathVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MathVariant::Normal              => write!(f, "normal"),
            MathVariant::Italic              => write!(f, "italic"),
            MathVariant::Bold                => write!(f, "bold"),
            MathVariant::BoldItalic          => write!(f, "bold-italic"),
            MathVariant::DoubleStruck        => write!(f, "double-struck"),
            MathVariant::BoldFraktur         => write!(f, "bold-fraktur"),
            MathVariant::Script              => write!(f, "script"),
            MathVariant::BoldScript          => write!(f, "bold-script"),
            MathVariant::Fraktur             => write!(f, "fraktur"),
            MathVariant::SansSerif           => write!(f, "sans-serif"),
            MathVariant::BoldSansSerif       => write!(f, "bold-sans-serif"),
            MathVariant::SansSerifItalic     => write!(f, "sans-serif-italic"),
            MathVariant::SansSerifBoldItalic => write!(f, "sans-serif-bold-italic"),
            MathVariant::Monospace           => write!(f, "monospace"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Accent {
    True,
    False,
}

impl Display for Accent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Accent::True => write!(f, "true"),
            Accent::False => write!(f, "false"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineThickness {
    Thin,
    Medium,
    Thick,
    Length(u8),
}
impl Display for LineThickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LineThickness::Thin => write!(f, r#" linethickness="thin""#),
            LineThickness::Medium => write!(f, r#""#),
            LineThickness::Thick => write!(f, r#" linethickness="medium""#),
            LineThickness::Length(l) => write!(f, r#" linethickness="{}""#, l),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAlign {
    Center,
    Left,
    Right,
}

impl Display for ColumnAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnAlign::Center => write!(f, r#""#),
            ColumnAlign::Left => write!(f, r#" columnalign=left"#),
            ColumnAlign::Right => write!(f, r#" columnalign=right"#),
        }
    }
}
