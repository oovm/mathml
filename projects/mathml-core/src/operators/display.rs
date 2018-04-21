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

impl Display for MathBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            MathBinaryKind::SuperScript => {
                write!(f, "<msup>{}{}</msup>", self.lhs, self.rhs)
            }
            MathBinaryKind::SubScript => {
                write!(f, "<msub>{}{}</msub>", self.lhs, self.rhs)
            }
        }
    }
}
