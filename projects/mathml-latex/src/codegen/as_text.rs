use super::*;

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
        }
    }
}
