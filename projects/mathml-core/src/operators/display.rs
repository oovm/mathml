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

impl Display for MathSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<mspace")?;
        for (key, value) in &self.attributes {
            write!(f, " {}=\"{}\"", key, value)?;
        }
        write!(f, "/>")
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

impl Display for MathFenced {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let last = self.separators.chars().last().unwrap_or(',');
        let mut separators = self.separators.chars().chain(repeat(last));
        f.write_str("<mrow><mo stretchy=\"true\" form=\"prefix\">")?;
        safe_html_char(f, self.open)?;
        f.write_str("</mo>")?;
        for (i, item) in self.base.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", item)?;
            }
            else {
                // SAFETY: `separators` is infinite
                let split = unsafe { separators.next().unwrap_unchecked() };
                f.write_str("<mo>")?;
                safe_html_char(f, split)?;
                f.write_str("</mo>")?;
                write!(f, "{}", item)?;
            }
        }
        f.write_str("<mo stretchy=\"true\" form=\"postfix\">")?;
        safe_html_char(f, self.close)?;
        f.write_str("</mo></mrow>")
    }
}

impl Display for MathMultiScript {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_sub_super_script() {
            write!(f, "<msubsup>{}{}{}</msubsup>", self.base, self.rd[0], self.ru[0])
        }
        else if self.is_sub_script() {
            write!(f, "<msub>{}{}</msub>", self.base, self.rd[0])
        }
        else if self.is_super_script() {
            write!(f, "<msup>{}{}</msup>", self.base, self.ru[0])
        }
        else {
            f.write_str("<mmultiscripts>")?;
            write!(f, "{}", self.base)?;

            let r_max = self.ru.len().max(self.rd.len());
            for i in 0..r_max {
                match self.rd.get(i) {
                    Some(s) => write!(f, "{}", s)?,
                    None => f.write_str("<mrow/>")?,
                }
                match self.ru.get(i) {
                    Some(s) => write!(f, "{}", s)?,
                    None => f.write_str("<mrow/>")?,
                }
            }
            f.write_str("<mprescripts/>")?;
            let l_max = self.lu.len().max(self.ld.len());
            for i in 0..l_max {
                match self.ld.get(i) {
                    Some(s) => write!(f, "{}", s)?,
                    None => f.write_str("<mrow/>")?,
                }
                match self.lu.get(i) {
                    Some(s) => write!(f, "{}", s)?,
                    None => f.write_str("<mrow/>")?,
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
