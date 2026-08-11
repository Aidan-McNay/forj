// =======================================================================
// patterns.rs
// =======================================================================
// Parsing for 1800-2023 A.6.6

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn conditional_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConditionalStatement<'s>, VerboseError<'s>> {
    (
        opt_note(unique_priority_parser),
        token(Token::If),
        token(Token::Paren),
        cond_predicate_parser,
        token(Token::EParen),
        statement_or_null_parser,
        repeat_note((
            token(Token::Else),
            token(Token::If),
            token(Token::Paren),
            cond_predicate_parser,
            token(Token::EParen),
            statement_or_null_parser,
        )),
        opt_note((token(Token::Else), statement_or_null_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            ConditionalStatement(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn unique_priority_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UniquePriority<'s>, VerboseError<'s>> {
    alt((
        token(Token::Unique).map(|a| UniquePriority::Unique(a)),
        token(Token::Unique0).map(|a| UniquePriority::Unique0(a)),
        token(Token::Priority).map(|a| UniquePriority::Priority(a)),
    ))
    .parse_next(input)
}

pub fn cond_predicate_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CondPredicate<'s>, VerboseError<'s>> {
    (
        expression_or_cond_pattern_parser,
        repeat_note((
            token(Token::AmpAmpAmp),
            expression_or_cond_pattern_parser,
        )),
    )
        .map(|(a, b)| CondPredicate(a, b))
        .parse_next(input)
}

pub fn expression_or_cond_pattern_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ExpressionOrCondPattern<'s>, VerboseError<'s>> {
    (
        gen_expression_parser(matches_operator_binding_power() + 1),
        opt_note((
            token(Token::Matches),
            gen_pattern_parser(matches_operator_binding_power() + 1),
        )),
    )
        .map(|(a, b)| match b {
            Some((metadata, pattern)) => ExpressionOrCondPattern::CondPattern(
                Box::new(CondPattern(a, metadata, pattern)),
            ),
            None => ExpressionOrCondPattern::Expression(Box::new(a)),
        })
        .parse_next(input)
}

#[allow(dead_code)] // Integrated in expression_or_cond_pattern_parser
pub fn cond_pattern_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CondPattern<'s>, VerboseError<'s>> {
    (
        gen_expression_parser(matches_operator_binding_power() + 1),
        token(Token::Matches),
        gen_pattern_parser(matches_operator_binding_power() + 1),
    )
        .map(|(a, b, c)| CondPattern(a, b, c))
        .parse_next(input)
}
