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
        if self.is_sub_super_script() {
            write!(f, "<msubsup>{}{}{}</msubsup>", self.base, self.ld[0], self.lu[0])
        }
        else if self.is_sub_script() {
            write!(f, "<msub>{}{}</msub>", self.base, self.ld[0])
        }
        else if self.is_super_script() {
            write!(f, "<msup>{}{}</msup>", self.base, self.lu[0])
        }
        else {
            f.write_str("<mmultiscripts>")?;
            write!(f, "{}", self.base)?;

            let r_max = self.ru.len().max(self.rd.len());
            for i in 0..r_max {
                match self.rd.get(i) {
                    Some(s) => write!(f, "{}", s)?,
                    None => write!(f, "<mrow></mrow>")?,
                }
                match self.ru.get(i) {
                    Some(s) => write!(f, "{}", s)?,
                    None => write!(f, "<mrow></mrow>")?,
                }
            }
            f.write_str("<mprescripts/>")?;
            let l_max = self.lu.len().max(self.ld.len());
            for i in 0..l_max {
                match self.ld.get(i) {
                    Some(s) => write!(f, "{}", s)?,
                    None => write!(f, "<mrow></mrow>")?,
                }
                match self.lu.get(i) {
                    Some(s) => write!(f, "{}", s)?,
                    None => write!(f, "<mrow></mrow>")?,
                }
            }
            f.write_str("</mmultiscripts>")
        }
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
