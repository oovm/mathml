use crate::LaTeXBlock;
use mathml_core::{MathIdentifier, MathML, MathNumber};
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use std::{collections::BTreeSet, str::FromStr};

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
    Block(LaTeXBlock<'i>),
    Command { name: &'i str, children: Vec<LaTeXNode<'i>> },
    Text { text: &'i str },
    Number { number: &'i str },
    Identifier { identifier: &'i str },
}

impl<'i> LaTeXNode<'i> {
    pub fn parse(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, node) = input
            .begin_choice()
            // .or_else(parse_command)
            // .or_else(parse_text)
            .or_else(Self::parse_number)
            .end_choice()?;
        state.finish(node)
    }
    // 1
    // 1.0
    fn parse_number(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, dec) = input.begin_choice().or_else(parse_decimal).end_choice()?;
        state.finish(LaTeXNode::Number { number: dec })
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

fn parse_decimal<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let mut offset = 0;
    let mut first_dot = true;
    for char in input.rest_text.chars() {
        match char {
            '.' if first_dot => {
                first_dot = false;
                offset += 1;
            }
            '0'..='9' => {
                offset += 1;
            }
            _ => {
                break;
            }
        }
    }
    input.advance_view(offset)
}

/// a
/// ax
pub fn parse_letter(input: ParseState) -> ParseResult<MathIdentifier> {
    let (state, x) = input.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA")?;
    state.finish(MathIdentifier::italic(x))
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
