use crate::LaTeXNode;
use pex::{helpers::whitespace, ParseResult, ParseState};

/// `\begin{kind}` ... `\end{kind}`
pub struct LaTeXBlock<'i> {
    pub kind: &'i str,
    pub children: Vec<LaTeXNode<'i>>,
}

/// `\frac{numerator}{denominator}`
pub struct LaTeXFraction<'i> {
    pub kind: &'static str,
    pub numerator: Box<LaTeXNode<'i>>,
    pub denominator: Box<LaTeXNode<'i>>,
}
