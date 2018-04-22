use super::*;

impl<'i> LaTeXNode<'i> {
    pub fn is_super_script(&self) -> bool {
        matches!(self, LaTeXNode::Superscript { .. })
    }

    pub(super) fn parse_super_script(input: ParseState<'i>) -> ParseResult<LaTeXNode<'i>> {
        let (state, lhs) = input.match_fn(Self::parse_atomic)?;
        if lhs.is_super_script() {
            // StopBecause::ShouldNotBe("^", state.start_offset)
            todo!()
        }
        let (state, _) = state.skip(whitespace).match_str("^", false)?;
        let (state, rhs) = state.skip(whitespace).match_fn(Self::parse_atomic)?;
        state.finish(LaTeXNode::Superscript { lhs: Box::new(lhs), rhs: Box::new(rhs) })
    }
}
