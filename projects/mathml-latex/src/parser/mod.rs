use crate::{block::LaTeXCommand, LaTeXBlock};
use mathml_core::{MathIdentifier, MathML, MathNumber};
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};

mod sup_sub;

pub fn parse_latex(s: &str) -> Result<LaTeXNode, StopBecause> {
    let state = ParseState::new(s.trim_end()).skip(whitespace);
    match LaTeXNode::parse(state) {
        ParseResult::Pending(state, compound) if state.is_empty() => Ok(compound),
        ParseResult::Pending(state, ..) => Err(StopBecause::ExpectEof { position: state.start_offset }),
        ParseResult::Stop(e) => Err(e),
    }
}

pub enum LaTeXNode<'i> {
    Root { children: Vec<LaTeXNode<'i>> },
    Row { children: Vec<LaTeXNode<'i>> },
    Block(LaTeXBlock<'i>),
    Command(LaTeXCommand<'i>),
    Text { text: &'i str },
    Number { number: &'i str },
    Operation { operator: &'i str },
    Superscript { lhs: Box<LaTeXNode<'i>>, rhs: Box<LaTeXNode<'i>> },
    Fraction { numerator: Box<LaTeXNode<'i>>, denominator: Box<LaTeXNode<'i>> },
    Letter { identifier: &'i str },
}

impl<'i> LaTeXNode<'i> {
    pub fn parse(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, node) = input
            .begin_choice()
            .or_else(Self::parse_block)
            .or_else(Self::parse_combined)
            .or_else(Self::parse_row)
            .end_choice()?;
        state.finish(node)
    }
    fn parse_block(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, block) = LaTeXBlock::parse(input)?;
        state.finish(LaTeXNode::Block(block))
    }
    fn parse_combined(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, lhs) = input.begin_choice().or_else(Self::parse_super_script).end_choice()?;
        state.finish(lhs)
    }
    /// `group := '{' atomic* '}'`
    fn parse_group(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, _) = input.match_char('{')?;
        let (state, mut children) = state.match_repeats(LaTeXNode::parse_atomic)?;
        let (state, _) = state.skip(whitespace).match_char('}')?;
        state.finish(LaTeXNode::Row { children })
    }
    /// `row := atomic*`
    fn parse_row(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, children) = input.match_repeats(LaTeXNode::parse_atomic)?;
        state.finish(LaTeXNode::Row { children })
    }
    fn parse_command(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, _) = input.match_char('\\')?;
        let (state, cmd) = state
            .begin_choice()
            .or_else(|state| state.match_char(' ').map_inner(|_| " "))
            .or_else(|state| state.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA"))
            .end_choice()?;
        let (state, args) = state.match_repeats(|state| state.skip(whitespace).match_fn(LaTeXNode::parse_group))?;

        state.finish(LaTeXNode::Command(LaTeXCommand { name: "", children: vec![] }))
    }
    fn parse_atomic(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, node) = input
            .skip(whitespace)
            .begin_choice()
            .or_else(Self::parse_group)
            .or_else(Self::parse_command)
            .or_else(Self::parse_letter)
            .or_else(Self::parse_operator)
            .or_else(Self::parse_number)
            .end_choice()?;
        state.finish(node)
    }
    // pub fn parse_maybe_digit(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
    //     let (state, node) = input.begin_choice().or_else(Self::parse_number).or_else(Self::parse_letter).end_choice()?;
    //     state.finish(node)
    // }
    // 1
    // 1.0
    fn parse_number(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, dec) = input.begin_choice().or_else(pex::helpers::dec_str).end_choice()?;
        state.finish(LaTeXNode::Number { number: dec })
    }
    /// a
    /// ax
    pub fn parse_letter(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, x) = input.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA")?;
        state.finish(LaTeXNode::Letter { identifier: x })
    }
    fn parse_operator(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, dec) = input
            .begin_choice()
            .or_else(|state| state.match_str("+", false))
            .or_else(|state| state.match_str("-", false))
            .end_choice()?;
        state.finish(LaTeXNode::Operation { operator: dec })
    }
}

impl<'i> LaTeXBlock<'i> {
    pub fn parse(input: ParseState<'i>) -> ParseResult<LaTeXBlock<'i>> {
        let (state, begin) = input.skip(whitespace).match_fn(Self::parse_begin)?;
        let (state, children) = state.skip(whitespace).match_repeats(LaTeXNode::parse)?;
        let (state, end) = state.skip(whitespace).match_fn(Self::parse_end)?;
        if begin != end {
            tracing::warn!("Mismatched begin/end: {} vs {}", begin, end);
        }
        state.finish(LaTeXBlock { kind: begin, children })
    }

    fn parse_begin(input: ParseState<'i>) -> ParseResult<&'i str> {
        let (state, _) = input.match_str("\\begin", false)?;
        let (state, _) = state.skip(whitespace).match_char('{')?;
        let (state, kind) = state.skip(whitespace).match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA")?;
        let (state, _) = state.skip(whitespace).match_char('}')?;
        state.finish(kind)
    }
    fn parse_end(input: ParseState<'i>) -> ParseResult<&'i str> {
        let (state, _) = input.match_str("\\end", false)?;
        let (state, _) = state.skip(whitespace).match_char('{')?;
        let (state, kind) = state.skip(whitespace).match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA")?;
        let (state, _) = state.skip(whitespace).match_char('}')?;
        state.finish(kind)
    }
}
