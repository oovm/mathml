use mathml_core::{MathIdentifier, MathML};
use pex::{ParseResult, ParseState};
use std::collections::BTreeSet;

pub fn parse_number(input: ParseState) -> ParseResult<String> {
    todo!()
}

/// a
/// ax
pub fn parse_letter(input: ParseState) -> ParseResult<MathIdentifier> {
    let (state, x) = input.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA")?;
    state.finish(MathIdentifier::italic(x))
}

pub fn parse_command<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    let (state, _) = input.match_char('\\')?;
    let (state, cmd) = state
        .begin_choice()
        .or_else(|state| state.match_char(' ').map_inner(|_| " "))
        .or_else(|state| state.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA"))
        .end_choice()?;
    state.finish(cmd)
}
