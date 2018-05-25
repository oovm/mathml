mod display;

use crate::{
    MathError, MathFenced, MathFraction, MathFunction, MathIdentifier, MathMultiScript, MathNumber, MathOperator, MathPhantom,
    MathRoot, MathRow, MathSpace, MathSqrt, MathStyle, MathTable, MathText, MathUnderOver,
};
use std::fmt::{Display, Formatter};

// noinspection SpellCheckingInspection
/// Represent the [MathML](https://w3c.github.io/mathml/) AST node, For semantic considerations, not exactly the same as the standard.
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
    /// Used for compatibility of [`\operatorname`]() in LaTeX
    Function(Box<MathFunction>),
    /// [`<msqrt>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/msqrt)
    Sqrt(Box<MathSqrt>),
    /// [`<mfrac>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mfrac)
    Frac(Box<MathFraction>),
    /// [`<mphantom>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mphantom)
    Phantom(Box<MathPhantom>),
    /// [`<mstyle>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mstyle)
    Style(Box<MathStyle>),
    /// [`<mfenced>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mfenced), but polyfill as [`<mrow>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mrow)
    Fenced(Box<MathFenced>),
    /// [`<mtable>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mtable)
    Table(Box<MathTable>),
    /// Used for unknown element
    Undefined(Box<MathError>),
    /// Used for compatibility of `&` in LaTeX
    Ampersand,
    /// Used for compatibility of `\\` in LaTeX
    NewLine,
    /// Used for compatibility of `&nbsp;` in HTML
    Nothing,
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
    MathStyle       => Style,
    MathFenced      => Fenced,
    
    MathError       => Undefined
}
