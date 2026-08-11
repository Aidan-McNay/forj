// =======================================================================
// primitive_strengths.rs
// =======================================================================
// Parsing for 1800-2023 A.3.2

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn pulldown_strength_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PulldownStrength<'s>, VerboseError<'s>> {
    alt((
        (
            token(Token::Paren),
            strength0_parser,
            token(Token::Comma),
            strength1_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| {
                PulldownStrength::S0S1(Box::new((a, b, c, d, e)))
            }),
        (
            token(Token::Paren),
            strength1_parser,
            token(Token::Comma),
            strength0_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| {
                PulldownStrength::S1S0(Box::new((a, b, c, d, e)))
            }),
        (token(Token::Paren), strength0_parser, token(Token::EParen))
            .map(|(a, b, c)| PulldownStrength::S0(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn pullup_strength_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PullupStrength<'s>, VerboseError<'s>> {
    alt((
        (
            token(Token::Paren),
            strength0_parser,
            token(Token::Comma),
            strength1_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| {
                PullupStrength::S0S1(Box::new((a, b, c, d, e)))
            }),
        (
            token(Token::Paren),
            strength1_parser,
            token(Token::Comma),
            strength0_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| {
                PullupStrength::S1S0(Box::new((a, b, c, d, e)))
            }),
        (token(Token::Paren), strength1_parser, token(Token::EParen))
            .map(|(a, b, c)| PullupStrength::S1(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}
