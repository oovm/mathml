use super::*;

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
