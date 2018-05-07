use super::*;
use crate::helpers::safe_html_str;

impl Display for MathIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // maybe short form
        if !f.alternate() && self.variant == MathVariant::Italic {
            f.write_str("<mi>")?;
            safe_html_str(f, &self.identifier)?;
            f.write_str("</mi>")
        }
        else {
            f.write_str("<mi mathvariant=\"")?;
            Display::fmt(&self.variant, f)?;
            f.write_str("\">")?;
            safe_html_str(f, &self.identifier)?;
            f.write_str("</mi>")
        }
    }
}

impl Display for MathText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tag = if self.is_string { "ms" } else { "mtext" };
        f.write_str("<")?;
        f.write_str(tag)?;
        f.write_str(">")?;
        safe_html_str(f, &self.text)?;
        f.write_str("</")?;
        f.write_str(tag)?;
        f.write_str(">")
    }
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
