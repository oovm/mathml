use super::*;

impl<'i> AsciiNode<'i> {
    pub fn as_identifier(&self) -> &'i str {
        match self {
            AsciiNode::Letter { identifier } => identifier,
            AsciiNode::Operation { operator } => operator,
            _ => "",
        }
    }
}

impl<'i> Display for AsciiNode<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiNode::Root { .. } => {
                todo!()
            }
            AsciiNode::Row { children } => {
                for child in children {
                    write!(f, "{}", child)?;
                }
                Ok(())
            }
            AsciiNode::Block(_) => {
                todo!()
            }
            AsciiNode::Command { .. } => {
                todo!()
            }
            AsciiNode::Text { .. } => {
                todo!()
            }
            AsciiNode::Number { number } => f.write_str(number),
            AsciiNode::Operation { .. } => {
                todo!()
            }
            AsciiNode::Superscript { .. } => {
                todo!()
            }
            AsciiNode::Letter { .. } => {
                todo!()
            }
            AsciiNode::NewLine => f.write_str("\\\\"),
            AsciiNode::Ampersand => f.write_str("&"),
        }
    }
}
