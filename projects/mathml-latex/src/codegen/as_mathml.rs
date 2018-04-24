use super::*;
use mathml_core::{MathBinary, MathIdentifier, MathML, MathNumber, MathOperator, MathRoot};

impl<'i> LaTeXNode<'i> {
    pub fn as_mathml(&self, context: &LaTeXEngine) -> MathML {
        match self {
            LaTeXNode::Root { children } => MathRoot::new(children.iter().map(|node| node.as_mathml(context))).into(),
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
            LaTeXNode::Superscript { lhs, rhs } => {
                MathBinary::super_script(lhs.as_mathml(context), rhs.as_mathml(context)).into()
            }
            LaTeXNode::Fraction { .. } => {
                todo!()
            }
        }
    }
}
