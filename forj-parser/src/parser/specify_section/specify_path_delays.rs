// =======================================================================
// specify_path_delays.rs
// =======================================================================
// Parsing for 1800-2023 A.7.4

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn path_delay_value_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PathDelayValue<'s>, VerboseError<'s>> {
    alt((
        list_of_path_delay_expressions_parser
            .map(|a| PathDelayValue::NoParen(Box::new(a))),
        (
            token(Token::Paren),
            list_of_path_delay_expressions_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c)| PathDelayValue::Paren(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn list_of_path_delay_expressions_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfPathDelayExpressions<'s>, VerboseError<'s>> {
    let _path_parser = t_path_delay_expression_parser
        .map(|a| ListOfPathDelayExpressions::Path(Box::new(a)));
    let _rise_fall_parser = (
        trise_path_delay_expression_parser,
        token(Token::Comma),
        tfall_path_delay_expression_parser,
    )
        .map(|(a, b, c)| {
            ListOfPathDelayExpressions::RiseFall(Box::new((a, b, c)))
        });
    let _rise_fall_z_parser = (
        trise_path_delay_expression_parser,
        token(Token::Comma),
        tfall_path_delay_expression_parser,
        token(Token::Comma),
        tz_path_delay_expression_parser,
    )
        .map(|(a, b, c, d, e)| {
            ListOfPathDelayExpressions::RiseFallZ(Box::new((a, b, c, d, e)))
        });
    let _edge_z_parser = (
        t01_path_delay_expression_parser,
        token(Token::Comma),
        t10_path_delay_expression_parser,
        token(Token::Comma),
        t0z_path_delay_expression_parser,
        token(Token::Comma),
        tz1_path_delay_expression_parser,
        token(Token::Comma),
        t1z_path_delay_expression_parser,
        token(Token::Comma),
        tz0_path_delay_expression_parser,
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k)| {
            ListOfPathDelayExpressions::EdgeZ(Box::new((
                a, b, c, d, e, f, g, h, i, j, k,
            )))
        });
    let _edge_z_x_parser = (
        (
            t01_path_delay_expression_parser,
            token(Token::Comma),
            t10_path_delay_expression_parser,
            token(Token::Comma),
            t0z_path_delay_expression_parser,
            token(Token::Comma),
            tz1_path_delay_expression_parser,
            token(Token::Comma),
            t1z_path_delay_expression_parser,
            token(Token::Comma),
            tz0_path_delay_expression_parser,
            token(Token::Comma),
        ),
        (
            t0x_path_delay_expression_parser,
            token(Token::Comma),
            tx1_path_delay_expression_parser,
            token(Token::Comma),
            t1x_path_delay_expression_parser,
            token(Token::Comma),
            tx0_path_delay_expression_parser,
            token(Token::Comma),
            txz_path_delay_expression_parser,
            token(Token::Comma),
            tzx_path_delay_expression_parser,
        ),
    )
        .map(|(a, b)| ListOfPathDelayExpressions::EdgeZX(Box::new((a, b))));
    alt((
        _path_parser,
        _rise_fall_parser,
        _rise_fall_z_parser,
        _edge_z_parser,
        _edge_z_x_parser,
    ))
    .parse_next(input)
}

pub fn t_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| TPathDelayExpression(a))
        .parse_next(input)
}

pub fn trise_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TrisePathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| TrisePathDelayExpression(a))
        .parse_next(input)
}

pub fn tfall_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TfallPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| TfallPathDelayExpression(a))
        .parse_next(input)
}

pub fn tz_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TzPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| TzPathDelayExpression(a))
        .parse_next(input)
}

pub fn t01_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<T01PathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| T01PathDelayExpression(a))
        .parse_next(input)
}

pub fn t10_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<T10PathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| T10PathDelayExpression(a))
        .parse_next(input)
}

pub fn t0z_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<T0zPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| T0zPathDelayExpression(a))
        .parse_next(input)
}

pub fn tz1_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Tz1PathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| Tz1PathDelayExpression(a))
        .parse_next(input)
}

pub fn t1z_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<T1zPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| T1zPathDelayExpression(a))
        .parse_next(input)
}

pub fn tz0_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Tz0PathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| Tz0PathDelayExpression(a))
        .parse_next(input)
}

pub fn t0x_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<T0xPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| T0xPathDelayExpression(a))
        .parse_next(input)
}

pub fn tx1_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Tx1PathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| Tx1PathDelayExpression(a))
        .parse_next(input)
}

pub fn t1x_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<T1xPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| T1xPathDelayExpression(a))
        .parse_next(input)
}

pub fn tx0_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Tx0PathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| Tx0PathDelayExpression(a))
        .parse_next(input)
}

pub fn txz_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TxzPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| TxzPathDelayExpression(a))
        .parse_next(input)
}

pub fn tzx_path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TzxPathDelayExpression<'s>, VerboseError<'s>> {
    path_delay_expression_parser
        .map(|a| TzxPathDelayExpression(a))
        .parse_next(input)
}

pub fn path_delay_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PathDelayExpression<'s>, VerboseError<'s>> {
    constant_mintypmax_expression_parser
        .map(|a| PathDelayExpression(a))
        .parse_next(input)
}
