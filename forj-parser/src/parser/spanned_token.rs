// =======================================================================
// spanned_token.rs
// =======================================================================
// A token with an associated span, to be used in parsing

use crate::*;
use lexer::Token;
use winnow::ModalResult;
use winnow::Parser;
use winnow::error::ErrMode;
use winnow::stream::{Stateful, TokenSlice};
use winnow::token::literal;

// Keep track of the largest error we've seen in repeat/opt branches
pub(crate) type Tokens<'s> =
    Stateful<TokenSlice<'s, SpannedToken<'s>>, Option<VerboseError<'s>>>;
impl<'s> Parser<Tokens<'s>, &'s SpannedToken<'s>, ErrMode<VerboseError<'s>>>
    for Token<'s>
{
    fn parse_next(
        &mut self,
        input: &mut Tokens<'s>,
    ) -> ModalResult<&'s SpannedToken<'s>, VerboseError<'s>> {
        literal(*self).parse_next(input).map(|t| &t[0])
    }
}
