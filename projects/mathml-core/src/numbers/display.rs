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
