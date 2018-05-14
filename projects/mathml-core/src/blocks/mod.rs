use crate::{MathIdentifier, MathML};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

mod constructors;
mod display;

/// <https://developer.mozilla.org/en-US/docs/Web/MathML/Element/math>
#[derive(Debug, Clone, PartialEq)]
pub struct MathRoot {
    children: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathFunction {
    name: String,
    body: Vec<MathML>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathRow {
    children: Vec<MathML>,
    has_grouping: bool,
}

/// The [`<mphantom>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mphantom) element is rendered invisibly, but dimensions (such as height, width, and baseline position) are still kept.
#[derive(Debug, Clone, PartialEq)]
pub struct MathPhantom {
    inner: MathML,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathTable {
    stream: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}

impl MathTable {
    pub fn matrix<I>(stream: I) -> Self
    where
        I: IntoIterator<Item = MathML>,
    {
        Self { stream: stream.into_iter().collect(), attributes: BTreeMap::new() }
    }
    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    pub fn get_attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
    pub fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
}

impl Display for MathTable {
    // let mut mathml = format!("<mtable{}><mtr><mtd>", columnalign);
    // for (i, node) in content.iter().enumerate() {
    //     match node {
    //         MathML::NewLine => {
    //             mathml.push_str("</mtd></mtr>");
    //             if i < content.len() {
    //                 mathml.push_str("<mtr><mtd>")
    //             }
    //         }
    //         MathML::Ampersand => {
    //             mathml.push_str("</mtd>");
    //             if i < content.len() {
    //                 mathml.push_str("<mtd>")
    //             }
    //         }
    //         node => {
    //             mathml = format!("{}{}", mathml, node);
    //         }
    //     }
    // }
    // mathml.push_str("</mtd></mtr></mtable>");
    //
    // write!(f, "{}", mathml)
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<mtable")?;
        for (key, value) in &self.attributes {
            write!(f, " {}=\"{}\"", key, value)?;
        }
        f.write_str(">")?;
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
        f.write_str("</mtable>")
    }
}
