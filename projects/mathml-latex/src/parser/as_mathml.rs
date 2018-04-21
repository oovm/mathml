use super::*;
use crate::LaTeXDefinition;
use mathml_core::MathOperator;

impl<'i> LaTeXNode<'i> {
    pub fn as_mathml(&self, context: &LaTeXDefinition) -> MathML {
        match self {
            LaTeXNode::Root { children } => MathML::Root(Vec::from_iter(children.iter().map(|node| node.as_mathml(context)))),
            LaTeXNode::Row { children } => match children.as_slice() {
                [one] => one.as_mathml(context),
                many => MathML::Row(Vec::from_iter(many.iter().map(|node| node.as_mathml(context)))),
            },
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

            LaTeXNode::Letter { identifier } => MathIdentifier::italic(identifier).into(),
            LaTeXNode::Operation { operator } => MathOperator::new(operator).into(),
        }
    }
}
