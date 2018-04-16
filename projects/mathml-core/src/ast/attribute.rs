use super::*;

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
