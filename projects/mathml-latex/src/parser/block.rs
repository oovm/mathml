use super::*;

impl<'i> LaTeXBlock<'i> {
    pub fn parse(input: ParseState<'i>) -> ParseResult<LaTeXBlock<'i>> {
        let (state, begin) = input.skip(whitespace).match_fn(Self::parse_begin)?;
        let (state, children) = state.skip(whitespace).match_repeat_m_n(1, 255, LaTeXNode::parse_row)?;
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
