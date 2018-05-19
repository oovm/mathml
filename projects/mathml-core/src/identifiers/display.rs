use super::*;
use crate::helpers::safe_html_str;

impl Display for MathIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // maybe short form
        if !f.alternate() && self.variant == FontVariant::Italic {
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
impl Display for FontVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FontVariant::Normal              => write!(f, "normal"),
            FontVariant::Italic              => write!(f, "italic"),
            FontVariant::Bold                => write!(f, "bold"),
            FontVariant::BoldItalic          => write!(f, "bold-italic"),
            FontVariant::DoubleStruck        => write!(f, "double-struck"),
            FontVariant::BoldFraktur         => write!(f, "bold-fraktur"),
            FontVariant::Script              => write!(f, "script"),
            FontVariant::BoldScript          => write!(f, "bold-script"),
            FontVariant::Fraktur             => write!(f, "fraktur"),
            FontVariant::SansSerif           => write!(f, "sans-serif"),
            FontVariant::BoldSansSerif       => write!(f, "bold-sans-serif"),
            FontVariant::SansSerifItalic     => write!(f, "sans-serif-italic"),
            FontVariant::SansSerifBoldItalic => write!(f, "sans-serif-bold-italic"),
            FontVariant::Monospace           => write!(f, "monospace"),
        }
    }
}
