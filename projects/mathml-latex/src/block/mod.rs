use crate::LaTeXNode;
use pex::{helpers::whitespace, ParseResult, ParseState};

/// `\begin{kind}` ... `\end{kind}`
pub struct LaTeXBlock<'i> {
    pub kind: &'i str,
    pub children: Vec<LaTeXNode<'i>>,
}
