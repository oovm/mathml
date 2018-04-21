use crate::LaTeXBlock;
use mathml_core::{MathIdentifier, MathML, MathNumber};
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};

mod as_mathml;

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
    Command { name: &'i str, children: Vec<LaTeXNode<'i>> },
    Text { text: &'i str },
    Number { number: &'i str },
    Operation { operator: &'i str },
    Letter { identifier: &'i str },
}

impl<'i> LaTeXNode<'i> {
    pub fn parse(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, node) = input
            .begin_choice()
            .or_else(Self::parse_block)
            // .or_else(parse_text)
            .or_else(Self::parse_row)
            .end_choice()?;
        state.finish(node)
    }
    fn parse_block(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, block) = LaTeXBlock::parse(input)?;
        state.finish(LaTeXNode::Block(block))
    }
    fn parse_row(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, mut children) = input.skip(whitespace).match_repeats(LaTeXNode::parse_atomic)?;
        state.finish(LaTeXNode::Row { children })
    }
    fn parse_atomic(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, node) = input
            .skip(whitespace)
            .begin_choice()
            .or_else(Self::parse_letter)
            .or_else(Self::parse_operator)
            .or_else(Self::parse_number)
            .end_choice()?;
        state.finish(node)
    }
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

fn parse_command_head<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let (state, _) = input.match_char('\\')?;
    let (state, cmd) = state
        .begin_choice()
        .or_else(|state| state.match_char(' ').map_inner(|_| " "))
        .or_else(|state| state.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA"))
        .end_choice()?;
    state.finish(cmd)
}

fn parse_command_brace<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let (state, _) = input.match_char('{')?;

    let (state, _) = state.skip(whitespace).match_char('}')?;
    todo!()
}
