use super::*;
use crate::LaTeXDefinition;

impl<'i> LaTeXNode<'i> {
    pub fn as_mathml(&self, context: &LaTeXDefinition) -> MathML {
        match self {
            LaTeXNode::Root { children } => MathML::Root(Vec::from_iter(children.iter().map(|node| node.as_mathml(context)))),
            LaTeXNode::Block(v) => {
                todo!()
            }
            LaTeXNode::Command { .. } => {
                todo!()
            }
            LaTeXNode::Text { .. } => {
                todo!()
            }
            LaTeXNode::Number { number } => MathML::Number(Box::new(MathNumber::new(number))),

            LaTeXNode::Identifier { .. } => {
                todo!()
            }
        }
    }
}
