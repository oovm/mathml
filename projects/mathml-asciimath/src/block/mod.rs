use crate::AsciiNode;

/// `\begin{kind}` ... `\end{kind}`
#[derive(Clone, Debug)]
pub struct AsciiBlock<'i> {
    pub kind: &'i str,
    pub children: Vec<AsciiNode<'i>>,
}

/// `\frac{numerator}{denominator}`
#[derive(Clone, Debug)]
pub struct LaTeXFraction<'i> {
    pub kind: &'static str,
    pub numerator: Box<AsciiNode<'i>>,
    pub denominator: Box<AsciiNode<'i>>,
}

#[derive(Clone, Debug)]
pub struct LaTeXCommand<'i> {
    pub name: &'i str,
    pub children: Vec<AsciiNode<'i>>,
}
