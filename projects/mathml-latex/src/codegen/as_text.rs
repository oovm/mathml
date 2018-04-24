use super::*;

impl<'i> Display for LaTeXNode<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LaTeXNode::Root { .. } => {
                todo!()
            }
            LaTeXNode::Row { children } => {
                for child in children {
                    write!(f, "{}", child)?;
                }
                Ok(())
            }
            LaTeXNode::Block(_) => {
                todo!()
            }
            LaTeXNode::Command { .. } => {
                todo!()
            }
            LaTeXNode::Text { .. } => {
                todo!()
            }
            LaTeXNode::Number { .. } => {
                todo!()
            }
            LaTeXNode::Operation { .. } => {
                todo!()
            }
            LaTeXNode::Superscript { .. } => {
                todo!()
            }
            LaTeXNode::Letter { .. } => {
                todo!()
            }
            LaTeXNode::Fraction { numerator, denominator } => {
                f.write_str("\\frac{")?;
                numerator.fmt(f)?;
                f.write_str("}{")?;
                denominator.fmt(f)?;
                f.write_str("}")
            }
        }
    }
}
