pub mod attribute;
mod display;

use crate::{
    ast::attribute::Accent,
    blocks::{MathRow, MathStyle},
    identifiers::MathText,
    operators::{MathOperator, MathSqrt, MathUnderOver},
    MathFenced, MathFraction, MathFunction, MathIdentifier, MathMultiScript, MathNumber, MathPhantom, MathRoot, MathSpace,
    MathTable,
};
use std::fmt::{Display, Formatter};

/// AST node
#[derive(Debug, Clone, PartialEq)]
pub enum MathML {
    /// [`<math>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/math)
    Root(Box<MathRoot>),
    /// [`<mrow>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mrow)
    Row(Box<MathRow>),
    /// [`<mspace>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mspace)
    Space(Box<MathSpace>),
    /// [`<mn>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mn)
    Number(Box<MathNumber>),
    /// [`<mi>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mi)
    Identifier(Box<MathIdentifier>),
    /// [`<ms>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/ms) / [`<mtext>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mtext)
    Text(Box<MathText>),
    /// [`<mo>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mo)
    Operator(Box<MathOperator>),
    /// [`<msub>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msub)
    /// / [`<msup>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msup)
    /// /  [`<msubsup>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msubsup)
    /// / [`<mmultiscripts>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mmultiscripts)
    MultiScripts(Box<MathMultiScript>),
    /// [`<munder>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/munder) / [`<mover>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mover) / [`<munderover>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/munderover)
    UnderOver(Box<MathUnderOver>),
    Function(Box<MathFunction>),
    OverOp(char, Accent, Box<MathML>),
    UnderOp(char, Accent, Box<MathML>),
    Sqrt(Box<MathSqrt>),
    Frac(Box<MathFraction>),
    Phantom(Box<MathPhantom>),
    Style(Box<MathStyle>),
    Fenced(Box<MathFenced>),
    StrechedOp(bool, String),
    SizedParen {
        size: &'static str,
        paren: &'static str,
    },
    Table(Box<MathTable>),
    Slashed(Box<MathML>),
    Undefined(String),
    Ampersand,
    NewLine,
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

macro_rules! make_number {
    ($($name:ident),*) => {
        $(
            impl From<$name> for MathML {
                fn from(value: $name) -> Self {
                    MathML::Number(Box::new(value.into()))
                }
            }
        )*
    };
}

impl From<char> for MathML {
    fn from(value: char) -> Self {
        MathML::identifier(value)
    }
}

make_number![i8, i16, i32, i64, i128, isize];
make_number![u8, u16, u32, u64, u128, usize];
make_number![f32, f64];

#[rustfmt::skip]
make_mathml! {
    MathRoot        => Root,
    MathRow         => Row,
    MathTable       => Table,
    
    MathSpace       => Space,
    MathText        => Text,
    MathNumber      => Number,
    
    MathFunction    => Function,
    MathUnderOver   => UnderOver,

    MathIdentifier  => Identifier,
    MathOperator    => Operator,
    MathMultiScript => MultiScripts,
    MathSqrt        => Sqrt,
    MathFraction    => Frac,
    MathPhantom     => Phantom,
    MathFenced      => Fenced
}
