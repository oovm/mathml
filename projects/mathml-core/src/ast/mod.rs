pub mod attribute;
mod display;

use crate::{
    ast::attribute::{Accent, ColumnAlign},
    identifiers::MathText,
    operators::{MathOperator, MathSqrt, MathUnderOver},
    MathFraction, MathIdentifier, MathMultiScript, MathNumber, MathPhantom, MathRoot,
};
use std::fmt::{Display, Formatter};

/// AST node
#[derive(Debug, Clone, PartialEq)]
pub enum MathML {
    /// [`<math>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/math)
    Root(Box<MathRoot>),
    /// [`<mn>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mn)
    Number(Box<MathNumber>),
    /// [`<mi>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mi)
    Identifier(Box<MathIdentifier>),
    /// [`<ms>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/ms) / [`<mtext>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mtext)
    Text(Box<MathText>),
    /// [`<mo>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mo)
    Operator(Box<MathOperator>),
    /// [`<msub>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msub) / [`<msup>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msup) /  [`<msubsup>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msubsup)
    SubSup(Box<MathMultiScript>),
    /// [`<munder>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/munder) / [`<mover>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mover) / [`<munderover>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/munderover)
    UnderOver(Box<MathUnderOver>),
    Function(String, Option<Box<MathML>>),
    Space(f32),
    OverOp(char, Accent, Box<MathML>),
    UnderOp(char, Accent, Box<MathML>),
    Sqrt(Box<MathSqrt>),
    Frac(Box<MathFraction>),
    Phantom(Box<MathPhantom>),
    Row(Vec<MathML>),
    Fenced {
        open: &'static str,
        close: &'static str,
        content: Box<MathML>,
    },
    StrechedOp(bool, String),
    SizedParen {
        size: &'static str,
        paren: &'static str,
    },
    Matrix(Vec<MathML>, ColumnAlign),
    Ampersand,
    NewLine,
    Slashed(Box<MathML>),
    Style(Option<DisplayStyle>, Box<MathML>),
    Undefined(String),
}

/// display style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayStyle {
    Block,
    Inline,
}

macro_rules! make_mathml {
    ($($name:ident => $variant:ident),*) => {
        $(
            impl From<$name> for MathML {
                fn from(value: $name) -> Self {
                    MathML::$variant(Box::new(value))
                }
            }
        )*
    };
}

#[rustfmt::skip]
make_mathml! {
    MathRoot       => Root,
    MathNumber     => Number,
    MathIdentifier => Identifier,
    MathOperator   => Operator,
    MathMultiScript   => SubSup,
    MathSqrt       => Sqrt,
    MathFraction   => Frac,
    MathPhantom    => Phantom
}
