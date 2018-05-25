use super::*;
use crate::{block::LaTeXCommand, AsciiBlock};
use mathml_core::{
    helpers::{binom, bmatrix, cases, dfrac, frac, matrix, pmatrix, vmatrix, Bmatrix, Pmatrix, Vmatrix},
    MathFunction, MathIdentifier, MathML, MathMultiScript, MathNumber, MathOperator, MathRoot, MathRow, MathSpace,
};

impl<'i> AsciiNode<'i> {
    pub fn as_mathml(&self, context: &LaTeXEngine) -> MathML {
        match self {
            AsciiNode::Root { children } => MathRoot::new(children.iter().map(|node| node.as_mathml(context))).into(),
            AsciiNode::Row { children } => MathRow::new(children.iter().map(|node| node.as_mathml(context))).into(),
            AsciiNode::Block(block) => block.as_mathml(context),
            AsciiNode::Command(cmd) => cmd.as_mathml(context),
            AsciiNode::Text { .. } => {
                todo!()
            }
            AsciiNode::Number { number } => MathML::Number(Box::new(MathNumber::new(number))),

            AsciiNode::Letter { identifier } => MathIdentifier::italic(identifier).into(),
            AsciiNode::Operation { operator } => MathOperator::new(operator).into(),
            AsciiNode::Superscript { lhs, rhs } => {
                MathMultiScript::super_script(lhs.as_mathml(context), rhs.as_mathml(context)).into()
            }
            AsciiNode::NewLine => MathML::NewLine,
            AsciiNode::Ampersand => MathML::Ampersand,
        }
    }
}

impl<'i> AsciiBlock<'i> {
    pub fn as_mathml(&self, context: &LaTeXEngine) -> MathML {
        let stream = self.children.iter().map(|node| node.as_mathml(context));
        match self.kind {
            "matrix" => matrix(stream),
            "Bmatrix" => Bmatrix(stream),
            "bmatrix" => bmatrix(stream),
            "Pmatrix" => Pmatrix(stream),
            "pmatrix" => pmatrix(stream),
            "Vmatrix" => Vmatrix(stream),
            "vmatrix" => vmatrix(stream),
            "cases" => cases(stream),
            name => todo!("unknown block: {}", name),
        }
    }
}

impl<'i> LaTeXCommand<'i> {
    pub fn as_mathml(&self, context: &LaTeXEngine) -> MathML {
        match self.name {
            kind @ ("frac" | "dfrac") => match self.children.as_slice() {
                [] => panic!("frac command must have at least two arguments"),
                [numerator] => panic!("frac command must have at least two arguments"),
                [numerator, denominator, rest @ ..] => {
                    let term = match kind {
                        "frac" => frac(numerator.as_mathml(context), denominator.as_mathml(context)),
                        "dfrac" => dfrac(numerator.as_mathml(context), denominator.as_mathml(context)),
                        _ => unreachable!(),
                    };
                    if rest.len() == 0 {
                        return term;
                    }
                    let mut terms = MathRow::new(vec![term]);
                    terms.mut_items().extend(rest.iter().map(|node| node.as_mathml(context)));
                    return terms.into();
                }
            },
            _ => {}
        }
        if self.name.eq("binom") {
            match self.children.as_slice() {
                [numerator, denominator] => {
                    return binom(numerator.as_mathml(context), denominator.as_mathml(context));
                }
                _ => panic!("binom command must have exactly two arguments"),
            }
        }
        if self.name.eq("operatorname") {
            match self.children.as_slice() {
                [] => panic!("operatorname command must have exactly one argument"),
                [head, rest @ ..] => {
                    return MathFunction::new(head.as_identifier(), rest.iter().map(|node| node.as_mathml(context))).into();
                }
            }
        }
        if let Some(s) = context.get_function(&self.name) {
            return MathFunction::new(s, self.children.iter().map(|node| node.as_mathml(context))).into();
        }
        if let Some(s) = context.get_operator(&self.name) {
            return MathOperator::new(s).into();
        }
        if let Some(s) = context.get_letters(&self.name) {
            return MathIdentifier::normal(s).into();
        }
        if let Some(s) = context.get_space(&self.name) {
            return MathSpace::new(s).into();
        }

        todo!()
    }
}
