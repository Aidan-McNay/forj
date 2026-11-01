// =======================================================================
// spanned_token.rs
// =======================================================================
// A token with an associated span, to be used in parsing

use crate::*;
use forj_syntax::*;
use lexer::Token;
use std::collections::HashMap;
use winnow::ModalResult;
use winnow::Parser;
use winnow::error::ErrMode;
use winnow::stream::{Checkpoint, Offset, Stateful, Stream, TokenSlice};
use winnow::token::literal;

// Memoize primaries to avoid re-parsing in multiple branches
#[derive(Debug)]
pub(crate) struct MemoizedState<'s> {
    pub(crate) beginning_of_stream: Checkpoint<
        &'s [SpannedToken<'s>],
        winnow::stream::TokenSlice<'s, SpannedToken<'s>>,
    >,
    pub(crate) primaries:
        HashMap<usize, (ModalResult<Primary<'s>, VerboseError<'s>>, usize)>,
}

impl<'s> MemoizedState<'s> {
    pub(crate) fn new(stream: &TokenSlice<'s, SpannedToken<'s>>) -> Self {
        Self {
            beginning_of_stream: stream.checkpoint(),
            primaries: HashMap::new(),
        }
    }
    pub(crate) fn offset(
        &self,
        stream: &TokenSlice<'s, SpannedToken<'s>>,
    ) -> usize {
        stream.checkpoint().offset_from(&self.beginning_of_stream)
    }
}

#[derive(Debug)]
pub(crate) struct ParserState<'s> {
    pub last_err: Option<VerboseError<'s>>,
    pub memoized: MemoizedState<'s>,
}

impl<'s> ParserState<'s> {
    pub(crate) fn new(stream: &TokenSlice<'s, SpannedToken<'s>>) -> Self {
        Self {
            last_err: None,
            memoized: MemoizedState::new(stream),
        }
    }
}

// Keep track of the largest error we've seen in repeat/opt branches
pub(crate) type Tokens<'s, 'b> =
    Stateful<TokenSlice<'s, SpannedToken<'s>>, &'b mut ParserState<'s>>;
impl<'s: 'b, 'b>
    Parser<Tokens<'s, 'b>, &'s SpannedToken<'s>, ErrMode<VerboseError<'s>>>
    for Token<'s>
{
    fn parse_next(
        &mut self,
        input: &mut Tokens<'s, 'b>,
    ) -> ModalResult<&'s SpannedToken<'s>, VerboseError<'s>> {
        literal(*self).parse_next(input).map(|t| &t[0])
    }
}
