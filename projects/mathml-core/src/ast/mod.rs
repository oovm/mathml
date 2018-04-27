pub mod attribute;
mod display;

use crate::{
    ast::attribute::{Accent, ColumnAlign},
    identifiers::MathText,
    operators::{MathOperator, MathSqrt},
    MathFraction, MathIdentifier, MathNumber, MathPhantom, MathRoot, MathSub, MathSubSup, MathSup,
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
    /// [`<msub>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msub)
    Sub(Box<MathSub>),
    /// [`<msup>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msup)
    Sup(Box<MathSup>),
    /// [`<msubsup>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msubsup)
    SubSup(Box<MathSubSup>),
    Function(String, Option<Box<MathML>>),
    Space(f32),
    OverOp(char, Accent, Box<MathML>),
    UnderOp(char, Accent, Box<MathML>),
    Overset {
        over: Box<MathML>,
        target: Box<MathML>,
    },
    Underset {
        under: Box<MathML>,
        target: Box<MathML>,
    },
    Under(Box<MathML>, Box<MathML>),
    UnderOver {
        target: Box<MathML>,
        under: Box<MathML>,
        over: Box<MathML>,
    },
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
    MathSub        => Sub,
    MathSup        => Sup,
    MathSubSup     => SubSup,
    MathSqrt       => Sqrt,
    MathFraction   => Frac,
    MathPhantom    => Phantom
}
