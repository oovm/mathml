use super::*;
use crate::block::LaTeXCommand;
use mathml_core::{MathIdentifier, MathML, MathMultiScript, MathNumber, MathOperator, MathRoot};
use std::process::Command;

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
            LaTeXNode::Command(cmd) => cmd.as_mathml(context),
            LaTeXNode::Text { .. } => {
                todo!()
            }
            LaTeXNode::Number { number } => MathML::Number(Box::new(MathNumber::new(number))),

            LaTeXNode::Letter { identifier } => MathIdentifier::italic(identifier).into(),
            LaTeXNode::Operation { operator } => MathOperator::new(operator).into(),
            LaTeXNode::Superscript { lhs, rhs } => {
                MathMultiScript::super_script(lhs.as_mathml(context), rhs.as_mathml(context)).into()
            }
            LaTeXNode::Fraction { .. } => {
                todo!()
            }
        }
    }
}

impl<'i> LaTeXCommand<'i> {
    pub fn as_mathml(&self, context: &LaTeXEngine) -> MathML {
        if self.name.eq("frac") {
            match self.children.as_slice() {
                [numerator, denominator] => {
                    return MathML::fraction(numerator.as_mathml(context), denominator.as_mathml(context));
                }
                _ => panic!("frac command must have exactly two arguments"),
            }
        }
        if self.name.eq("binom") {
            match self.children.as_slice() {
                [numerator, denominator] => {
                    return MathML::fraction(numerator.as_mathml(context), denominator.as_mathml(context));
                }
                _ => panic!("binom command must have exactly two arguments"),
            }
        }
        if let Some(s) = context.get_function(&self.name) {
            todo!()
        }
        println!("unknown command: {}", self.name);
        todo!()
    }
}
