use super::*;
use crate::MathSpace;

impl Display for MathRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<math")?;
        for (key, value) in &self.attributes {
            write!(f, " {}=\"{}\"", key, value)?;
        }
        write!(f, ">")?;
        for child in &self.children {
            write!(f, "{}", child)?;
        }
        write!(f, "</math>")
    }
}

impl Display for MathPhantom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<mphantom>")?;
        write!(f, "{}", self.inner)?;
        write!(f, "</mphantom>")
    }
}

impl Display for MathRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<mrow>")?;
        for child in &self.children {
            write!(f, "{}", child)?;
        }
        f.write_str("</mrow>")
    }
}

impl Display for MathFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let identifier = MathIdentifier::normal(&self.name);
        if self.body.is_empty() {
            write!(f, "{}", identifier)
        }
        else {
            let mut row = MathRow::new(vec![identifier.into(), MathSpace::new(0.167).into()]);
            row.mut_items().extend(self.body.iter().cloned());
            write!(f, "{}", row)
        }
    }
}
