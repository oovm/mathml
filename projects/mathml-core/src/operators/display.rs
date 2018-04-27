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
        match (&self.under, &self.over) {
            (Some(under), Some(over)) => {
                f.write_str("<munderover")?;
                if under.accent {
                    f.write_str(" accentunder=\"true\"")?;
                }
                if over.accent {
                    f.write_str(" accent=\"true\"")?;
                }
                write!(f, ">{}</munderover>", under.base)
            }
            (Some(under), None) => {
                f.write_str("<munder")?;
                if under.accent {
                    f.write_str(" accentunder=\"true\"")?;
                }
                write!(f, ">{}</munder>", under.base)
            }
            (None, Some(over)) => {
                f.write_str("<mover")?;
                if over.accent {
                    f.write_str(" accent=\"true\"")?;
                }
                write!(f, ">{}</mover>", over.base)
            }
            (None, None) => unreachable!("MathUnderOver must have at least one of under or over"),
        }
    }
}
