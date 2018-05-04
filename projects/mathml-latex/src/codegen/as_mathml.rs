use super::*;
use crate::block::LaTeXCommand;
use mathml_core::{
    helpers::{binom, frac},
    MathIdentifier, MathML, MathMultiScript, MathNumber, MathOperator, MathRoot,
};
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
                [] => panic!("frac command must have at least two arguments"),
                [numerator] => panic!("frac command must have at least two arguments"),
                [numerator, denominator, rest @ ..] => {
                    let term = frac(numerator.as_mathml(context), denominator.as_mathml(context));
                    if rest.len() == 0 {
                        return term;
                    }
                    let mut terms = Vec::with_capacity(rest.len() + 1);
                    terms.push(term);
                    terms.extend(rest.iter().map(|node| node.as_mathml(context)));
                    return MathML::Row(terms);
                }
            }
        }
        if self.name.eq("binom") {
            match self.children.as_slice() {
                [numerator, denominator] => {
                    return binom(numerator.as_mathml(context), denominator.as_mathml(context));
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
