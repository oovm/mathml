use super::*;

impl Display for MathNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<mn>{}</mn>", self.number)
    }
}

// noinspection SpellCheckingInspection
impl Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<merror>{}</merror>", self.message)
    }
}

// noinspection SpellCheckingInspection
impl Display for MathFraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.line_thickness {
            LineThickness::Thin => {
                write!(f, r#"<mfrac linethickness="thin">"#)?;
            }
            LineThickness::Medium => {
                write!(f, r#"<mfrac>"#)?;
            }
            LineThickness::Thick => {
                write!(f, r#"<mfrac linethickness="medium">"#)?;
            }
            LineThickness::Length(value) => {
                write!(f, r#"<mfrac linethickness="{}">"#, value)?;
            }
        }
        write!(f, "{}", self.numerator)?;
        write!(f, "{}", self.denominator)?;
        write!(f, "</mfrac>")
    }
}
