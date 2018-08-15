use crate::LaTeXNode;

/// `\begin{kind}` ... `\end{kind}`
#[derive(Clone, Debug)]
pub struct LaTeXBlock<'i> {
    /// The name of the block, e.g. `matrix`, `cases`, etc.
    pub kind: &'i str,
    /// The children of the block.
    pub children: Vec<LaTeXNode<'i>>,
}

/// `\frac{numerator}{denominator}`
#[derive(Clone, Debug)]
pub struct LaTeXFraction<'i> {
    /// the kind of fraction, e.g. `frac`, `dfrac`, etc.
    pub kind: &'static str,
    /// The numerator of the fraction.
    pub numerator: Box<LaTeXNode<'i>>,
    /// The denominator of the fraction.
    pub denominator: Box<LaTeXNode<'i>>,
}

#[derive(Clone, Debug)]
pub struct LaTeXCommand<'i> {
    pub name: &'i str,
    pub children: Vec<LaTeXNode<'i>>,
}
