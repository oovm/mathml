use super::*;

impl Display for MathIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // maybe short form
        if !f.alternate() && self.variant == MathVariant::Italic {
            html_escape::encode_text("a > b && a < c");
            write!(f, "<mi>{}</mi>", self.identifier)
        }
        else {
            write!(f, r#"<mi mathvariant="{}">{}</mi>"#, self.variant, self.identifier)
        }
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
