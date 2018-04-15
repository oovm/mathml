use super::*;

impl Display for MathLetter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.variant {
            MathVariant::Italic => write!(f, "<mi>{}</mi>", self.letter),
            var => write!(f, r#"<mi mathvariant="{}">{}</mi>"#, var, self.letter),
        }
    }
}
