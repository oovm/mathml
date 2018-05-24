use crate::{traits::MathElementWriter, MathElement, MathIdentifier, MathML};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
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

/// The [`<mrow>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mrow) element is used to group sub-expressions, which usually contain one or more operators with their respective operands (such as <mi> and <mn>).
///
/// This element renders as a horizontal row containing its arguments.
///
/// When writing a MathML expression, you should group elements within an <mrow> in the same way as they are grouped in the mathematical interpretation of the expression. Proper grouping helps the rendering of the expression in several ways:
///
/// - It can improve the display by possibly affecting spacing and preventing line breaks.
/// - It simplifies the interpretation of the expression by automated systems such as computer algebra systems and audio renderers.
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

/// The [`<mspace>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mspace) element is used to insert space characters into a mathematical formula.
#[derive(Debug, Clone, PartialEq)]
pub struct MathStyle {
    base: MathML,
    attributes: BTreeMap<String, String>,
}

/// The [`<mtable>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mtable) element allows you to create tables or matrices. Its children are <mtr> elements (representing rows), each of them having <mtd> elements as its children (representing cells).
///
/// These elements are similar to `<table>`, `<tr>` and `<td>` elements of HTML.
#[derive(Debug, Clone, PartialEq)]
pub struct MathTable {
    stream: Vec<MathML>,
    attributes: BTreeMap<String, String>,
}
