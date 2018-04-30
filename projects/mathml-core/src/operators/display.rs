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

impl Display for MathMultiScript {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tag = match (&self.sub, &self.sup) {
            (Some(_), Some(_)) => "msubsup",
            (Some(_), None) => "msub",
            (None, Some(_)) => "msup",
            (None, None) => unreachable!("MathSubSuper must have at least one of sub or sup"),
        };
        write!(f, "<{}", tag)?;
        for (key, value) in &self.attributes {
            write!(f, " {}=\"{}\"", key, value)?;
        }
        write!(f, ">")?;
        write!(f, "{}", self.base)?;
        if let Some(sub) = &self.sub {
            write!(f, "{}", sub)?;
        }
        if let Some(sup) = &self.sup {
            write!(f, "{}", sup)?;
        }
        write!(f, "</{}>", tag)
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
