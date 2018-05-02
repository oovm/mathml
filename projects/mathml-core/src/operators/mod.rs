use crate::MathML;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Display, Formatter, Write},
};

mod constructors;
mod display;

#[derive(Clone, Debug, PartialEq)]
pub struct MathOperator {
    operator: String,
}

/// The [`<mroot>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mroot) or [`<msqrt>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msqrt) element is used to display roots with an explicit index.
#[derive(Clone, Debug, PartialEq)]
pub struct MathSqrt {
    base: MathML,
    surd: Option<MathML>,
}

/// <https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msubsup>
#[derive(Clone, Debug, PartialEq)]
pub struct MathMultiScript {
    base: MathML,
    ru: Vec<MathML>,
    rd: Vec<MathML>,
    lu: Vec<MathML>,
    ld: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}

/// The [`<munderover>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/munderover) element is used to attach an accent or a limit under and over an expression.
#[derive(Clone, Debug, PartialEq)]
pub struct MathUnderOver {
    base: MathML,
    under: Option<MathML>,
    over: Option<MathML>,
    attributes: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MathFenced {
    base: MathML,
    lhs: String,
    rhs: String,
}

impl MathFenced {
    pub fn new<S>(base: MathML, lhs: S, rhs: S) -> Self
    where
        S: Into<String>,
    {
        Self { base, lhs: lhs.into(), rhs: rhs.into() }
    }
}

impl Display for MathFenced {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<mrow><mo stretchy="true" form="prefix">{}</mo>{}<mo stretchy="true" form="postfix">{}</mo></mrow>"#,
            self.lhs, self.base, self.rhs
        )
    }
}
