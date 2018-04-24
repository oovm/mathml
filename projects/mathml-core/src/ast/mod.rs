pub mod attribute;
mod display;

use crate::{
    ast::attribute::{Accent, ColumnAlign, LineThickness},
    operators::MathOperator,
    MathIdentifier, MathNumber, MathRoot, MathSub, MathSubSup, MathSup,
};
use std::fmt::{Display, Formatter};

/// AST node
#[derive(Debug, Clone, PartialEq)]
pub enum MathML {
    Root(Box<MathRoot>),
    Number(Box<MathNumber>),
    /// `<mi>`
    Letter(Box<MathIdentifier>),
    Text(String),
    Operator(Box<MathOperator>),
    SubScript(Box<MathSub>),
    SupScript(Box<MathSup>),
    SubSupScript(Box<MathSubSup>),
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
    Sqrt(Option<Box<MathML>>, Box<MathML>),
    Frac(Box<MathML>, Box<MathML>, LineThickness),
    Row(Vec<MathML>),
    Fenced {
        open: &'static str,
        close: &'static str,
        content: Box<MathML>,
    },
    StrechedOp(bool, String),
    OtherOperator(&'static str),
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
