use super::*;
use crate::LaTeXBlock;

impl<'i> LaTeXNode<'i> {
    pub fn as_identifier(&self) -> &'i str {
        match self {
            LaTeXNode::Letter { identifier } => identifier,
            LaTeXNode::Operation { operator } => operator,
            _ => "",
        }
    }
}

impl<'i> Display for LaTeXNode<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LaTeXNode::ArticleRoot { .. } => {
                todo!()
            }
            LaTeXNode::ArticleText { .. } => {
                todo!()
            }
            LaTeXNode::MathRoot { .. } => {
                todo!()
            }
            LaTeXNode::Row { children } => {
                for child in children {
                    write!(f, "{}", child)?;
                }
                Ok(())
            }
            LaTeXNode::Block(v) => Display::fmt(v, f),
            LaTeXNode::Command { .. } => {
                todo!()
            }
            LaTeXNode::MathText { .. } => {
                todo!()
            }
            LaTeXNode::Number { number } => f.write_str(number),
            LaTeXNode::Operation { .. } => {
                todo!()
            }
            LaTeXNode::Superscript { .. } => {
                todo!()
            }
            LaTeXNode::Letter { .. } => {
                todo!()
            }
            LaTeXNode::NewLine => f.write_str("\\\\"),
            LaTeXNode::Ampersand => f.write_str("&"),
        }
    }
}

impl<'i> Display for LaTeXBlock<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\\begin{{{}}}", self.kind)?;
        for child in &self.children {
            write!(f, "{}", child)?;
        }
        writeln!(f, "\\end{{{}}}", self.kind)
    }
}
