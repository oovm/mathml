use mathml_core::MathIdentifier;
use pex::{ParseResult, ParseState};

pub fn parse_number(input: ParseState) -> ParseResult<String> {
    todo!()
}

/// a
/// ax
pub fn parse_letter(input: ParseState) -> ParseResult<MathIdentifier> {
    let (state, x) = input.match_str_if(|c| c.is_ascii_alphabetic(), "ASCII_ALPHA")?;
    state.finish(MathIdentifier::italic(x))
}
