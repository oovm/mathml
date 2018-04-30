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

impl Display for MathSqrt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.surd {
            Some(power) => {
                write!(f, "<mroot>{}{}</mroot>", self.base, power)
            }
            None => {
                write!(f, "<msqrt>{}</msqrt>", self.base)
            }
        }
    }
}

impl Display for MathSup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<msup>{}{}</msup>", self.base, self.sup)
    }
}

impl Display for MathSub {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<msub>{}{}</msub>", self.base, self.sub)
    }
}

impl Display for MathSubSup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<msubsup>{}{}{}</msubsup>", self.base, self.sub, self.sup)
    }
}

impl Display for MathUnderOver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tag = match (&self.under, &self.over) {
            (Some(_), Some(_)) => "munderover",
            (Some(_), None) => "munder",
            (None, Some(_)) => "mover",
            (None, None) => unreachable!("MathUnderOver must have at least one of under or over"),
        };
        write!(f, "<{}", tag)?;
        for (key, value) in &self.attributes {
            write!(f, " {}=\"{}\"", key, value)?;
        }
        write!(f, ">")?;
        write!(f, "{}", self.base)?;
        if let Some(under) = &self.under {
            write!(f, "{}", under)?;
        }
        if let Some(over) = &self.over {
            write!(f, "{}", over)?;
        }
        write!(f, "</{}>", tag)
    }
}
