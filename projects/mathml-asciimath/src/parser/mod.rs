use crate::{block::LaTeXCommand, AsciiBlock};

use pex::{helpers::whitespace, ParseResult, ParseState, StopBecause};

mod block;
mod sup_sub;

pub fn parse_latex(s: &str) -> Result<AsciiNode, StopBecause> {
    let state = ParseState::new(s.trim_end()).skip(whitespace);
    match AsciiNode::parse(state) {
        ParseResult::Pending(state, compound) if state.is_empty() => Ok(compound),
        ParseResult::Pending(state, ..) => Err(StopBecause::ExpectEof { position: state.start_offset }),
        ParseResult::Stop(e) => Err(e),
    }
}

#[derive(Clone, Debug)]
pub enum AsciiNode<'i> {
    Root { children: Vec<AsciiNode<'i>> },
    Row { children: Vec<AsciiNode<'i>> },
    Block(AsciiBlock<'i>),
    Command(LaTeXCommand<'i>),
    Text { text: &'i str },
    Number { number: &'i str },
    Operation { operator: &'i str },
    Superscript { lhs: Box<AsciiNode<'i>>, rhs: Box<AsciiNode<'i>> },
    Letter { identifier: &'i str },
    // `\\`
    NewLine,
    // `&`
    Ampersand,
}

impl<'i> AsciiNode<'i> {
    pub fn refine(self) -> Self {
        match self {
            AsciiNode::Row { mut children } => {
                if children.len() == 1 {
                    children.remove(0)
                }
                else {
                    AsciiNode::Row { children }
                }
            }
            _ => self,
        }
    }
}

impl<'i> AsciiNode<'i> {
    pub fn parse(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, node) = input.begin_choice().or_else(Self::parse_combined).or_else(Self::parse_row).end_choice()?;
        state.finish(node)
    }
    fn parse_block(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, block) = AsciiBlock::parse(input)?;
        state.finish(AsciiNode::Block(block))
    }
    fn parse_combined(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, lhs) = input.begin_choice().or_else(Self::parse_super_script).end_choice()?;
        state.finish(lhs)
    }
    /// `group := '{' atomic* '}'`
    fn parse_group(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, _) = input.match_char('{')?;
        let (state, children) = state.match_repeats(AsciiNode::parse_atomic)?;
        let (state, _) = state.skip(whitespace).match_char('}')?;
        state.finish(AsciiNode::Row { children }.refine())
    }
    /// `row := atomic*`
    fn parse_row(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, children) = input.match_repeats(AsciiNode::parse_atomic)?;
        state.finish(AsciiNode::Row { children }.refine())
    }
    fn parse_command(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, _) = input.match_char('\\')?;
        let (state, cmd) = state
            .begin_choice()
            .or_else(|state| state.match_char(' ').map_inner(|_| " "))
            .or_else(|state| state.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA"))
            .end_choice()?;
        if cmd.eq("begin") {
            Err(StopBecause::ShouldNotBe { message: "\\begin", position: state.start_offset })?;
        }
        if cmd.eq("end") {
            Err(StopBecause::ShouldNotBe { message: "\\end", position: state.start_offset })?;
        }
        let (state, args) = state.match_repeats(|state| state.skip(whitespace).match_fn(AsciiNode::parse_group))?;
        state.finish(AsciiNode::Command(LaTeXCommand { name: cmd, children: args }))
    }
    fn parse_atomic(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, node) = input
            .skip(whitespace)
            .begin_choice()
            .or_else(Self::parse_block)
            .or_else(Self::parse_group)
            .or_else(Self::parse_command)
            .or_else(Self::parse_letter)
            .or_else(Self::parse_operator)
            .or_else(Self::parse_number)
            .or_else(Self::parse_special)
            .end_choice()?;
        state.finish(node)
    }
    // pub fn parse_maybe_digit(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
    //     let (state, node) = input.begin_choice().or_else(Self::parse_number).or_else(Self::parse_letter).end_choice()?;
    //     state.finish(node)
    // }
    // 1
    // 1.0
    fn parse_number(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, dec) = input.begin_choice().or_else(pex::helpers::dec_str).end_choice()?;
        state.finish(AsciiNode::Number { number: dec })
    }
    /// a
    /// ax
    pub fn parse_letter(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, x) = input.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA")?;
        state.finish(AsciiNode::Letter { identifier: x })
    }
    fn parse_operator(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, dec) = input
            .begin_choice()
            .or_else(|state| state.match_str("+", false))
            .or_else(|state| state.match_str("-", false).map_inner(|_| "−"))
            .end_choice()?;
        state.finish(AsciiNode::Operation { operator: dec })
    }
    fn parse_special(input: ParseState<'i>) -> ParseResult<AsciiNode<'i>> {
        let (state, item) = input
            .begin_choice()
            .or_else(|state| state.match_str("\\\\", false).map_inner(|_| AsciiNode::NewLine))
            .or_else(|state| state.match_str("&", false).map_inner(|_| AsciiNode::Ampersand))
            .end_choice()?;
        state.finish(item)
    }
}
