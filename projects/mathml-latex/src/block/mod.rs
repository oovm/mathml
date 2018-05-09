use crate::LaTeXNode;

/// `\begin{kind}` ... `\end{kind}`
#[derive(Clone, Debug)]
pub struct LaTeXBlock<'i> {
    pub kind: &'i str,
    pub children: Vec<LaTeXNode<'i>>,
}

/// `\frac{numerator}{denominator}`
#[derive(Clone, Debug)]
pub struct LaTeXFraction<'i> {
    pub kind: &'static str,
    pub numerator: Box<LaTeXNode<'i>>,
    pub denominator: Box<LaTeXNode<'i>>,
}

#[derive(Clone, Debug)]
pub struct LaTeXCommand<'i> {
    pub name: &'i str,
    pub children: Vec<LaTeXNode<'i>>,
}
