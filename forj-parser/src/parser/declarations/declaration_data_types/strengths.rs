// =======================================================================
// strengths.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.2

use crate::*;
use forj_syntax::*;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::error::ModalResult;

pub fn drive_strength_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DriveStrength<'s>, VerboseError<'s>> {
    alt((
        (
            token(Token::Paren),
            strength0_parser,
            token(Token::Comma),
            strength1_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| DriveStrength::S0S1(a, b, c, d, e)),
        (
            token(Token::Paren),
            strength1_parser,
            token(Token::Comma),
            strength0_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| DriveStrength::S1S0(a, b, c, d, e)),
        (
            token(Token::Paren),
            strength0_parser,
            token(Token::Comma),
            token(Token::Highz1),
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| DriveStrength::S0Z1(a, b, c, d, e)),
        (
            token(Token::Paren),
            strength1_parser,
            token(Token::Comma),
            token(Token::Highz0),
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| DriveStrength::S1Z0(a, b, c, d, e)),
        (
            token(Token::Paren),
            token(Token::Highz0),
            token(Token::Comma),
            strength1_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| DriveStrength::Z0S1(a, b, c, d, e)),
        (
            token(Token::Paren),
            token(Token::Highz1),
            token(Token::Comma),
            strength0_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| DriveStrength::Z1S0(a, b, c, d, e)),
    ))
    .parse_next(input)
}

pub fn strength0_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Strength0<'s>, VerboseError<'s>> {
    alt((
        token(Token::Supply0).map(|a| Strength0::Supply0(a)),
        token(Token::Strong0).map(|a| Strength0::Strong0(a)),
        token(Token::Pull0).map(|a| Strength0::Pull0(a)),
        token(Token::Weak0).map(|a| Strength0::Weak0(a)),
    ))
    .parse_next(input)
}

pub fn strength1_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Strength1<'s>, VerboseError<'s>> {
    alt((
        token(Token::Supply1).map(|a| Strength1::Supply1(a)),
        token(Token::Strong1).map(|a| Strength1::Strong1(a)),
        token(Token::Pull1).map(|a| Strength1::Pull1(a)),
        token(Token::Weak1).map(|a| Strength1::Weak1(a)),
    ))
    .parse_next(input)
}

pub fn charge_strength_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ChargeStrength<'s>, VerboseError<'s>> {
    let charge_strength_size_parser = alt((
        token(Token::Small).map(|a| ChargeStrengthSize::Small(a)),
        token(Token::Medium).map(|a| ChargeStrengthSize::Medium(a)),
        token(Token::Large).map(|a| ChargeStrengthSize::Large(a)),
    ));
    (
        token(Token::Paren),
        charge_strength_size_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| ChargeStrength(a, b, c))
        .parse_next(input)
}
