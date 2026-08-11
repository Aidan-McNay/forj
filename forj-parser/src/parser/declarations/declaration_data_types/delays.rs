// =======================================================================
// delays.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.3

use crate::*;
use forj_syntax::*;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::error::ModalResult;

pub fn delay2_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Delay2<'s>, VerboseError<'s>> {
    let _value_parser = (token(Token::Pound), delay_value_parser)
        .map(|(a, b)| Delay2::Value(Box::new((a, b))));
    let _mintypmax_parser = (
        token(Token::Pound),
        token(Token::Paren),
        mintypmax_expression_parser,
        opt_note((token(Token::Comma), mintypmax_expression_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| Delay2::Mintypmax(Box::new((a, b, c, d, e))));
    alt((_value_parser, _mintypmax_parser)).parse_next(input)
}

pub fn delay3_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Delay3<'s>, VerboseError<'s>> {
    let _value_parser = (token(Token::Pound), delay_value_parser)
        .map(|(a, b)| Delay3::Value(Box::new((a, b))));
    let _mintypmax_parser = (
        token(Token::Pound),
        token(Token::Paren),
        mintypmax_expression_parser,
        opt_note((
            token(Token::Comma),
            mintypmax_expression_parser,
            opt_note((token(Token::Comma), mintypmax_expression_parser)),
        )),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| Delay3::Mintypmax(Box::new((a, b, c, d, e))));
    alt((_value_parser, _mintypmax_parser)).parse_next(input)
}

pub fn delay_value_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DelayValue<'s>, VerboseError<'s>> {
    alt((
        unsigned_number_parser.map(|a| DelayValue::Unsigned(a)),
        real_number_parser.map(|a| DelayValue::Real(a)),
        ps_identifier_parser.map(|a| DelayValue::Ps(a)),
        time_literal_parser.map(|a| DelayValue::Time(a)),
        token(Token::OneStep).map(|a| DelayValue::OneStep(a)),
    ))
    .parse_next(input)
}
