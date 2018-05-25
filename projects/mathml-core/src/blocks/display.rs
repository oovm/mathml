use super::*;
use crate::{
    traits::{write_tag_close, write_tag_start},
    MathSpace,
};

impl Display for MathRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write_tag_start(f, self)?;
        for child in &self.children {
            write!(f, "{}", child)?;
        }
        write_tag_close(f, self)
    }
}

impl Display for MathPhantom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<mphantom>")?;
        write!(f, "{}", self.inner)?;
        write!(f, "</mphantom>")
    }
}
impl Display for MathStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write_tag_start(f, self)?;
        write!(f, "{}", self.base)?;
        write_tag_close(f, self)
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

impl Display for MathTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write_tag_start(f, self)?;
        f.write_str("<mtr><mtd>")?;
        for (i, node) in self.stream.iter().enumerate() {
            match node {
                MathML::NewLine => {
                    f.write_str("</mtd></mtr>")?;
                    if i < self.stream.len() {
                        f.write_str("<mtr><mtd>")?;
                    }
                }
                MathML::Ampersand => {
                    f.write_str("</mtd>")?;
                    if i < self.stream.len() {
                        f.write_str("<mtd>")?;
                    }
                }
                _ => {
                    write!(f, "{}", node)?;
                }
            }
        }
        f.write_str("</mtd></mtr>")?;
        write_tag_close(f, self)
    }
}
