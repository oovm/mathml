use super::*;

impl Display for MathOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.operator.eq(&"∂") {
            write!(f, r#"<mo mathvariant="italic">∂</mo>"#)
        }
        else {
            write!(f, r#"<mo>{}</mo>"#, self.operator)
        }
    }
}
