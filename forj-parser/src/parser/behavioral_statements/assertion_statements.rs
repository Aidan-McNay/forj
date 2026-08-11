// =======================================================================
// assertion_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.10

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn assertion_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssertionItem<'s>, VerboseError<'s>> {
    alt((
        concurrent_assertion_item_parser
            .map(|a| AssertionItem::Concurrent(Box::new(a))),
        deferred_immediate_assertion_item_parser
            .map(|a| AssertionItem::DeferredImmediate(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn deferred_immediate_assertion_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DeferredImmediateAssertionItem<'s>, VerboseError<'s>> {
    (
        opt_note((block_identifier_parser, token(Token::Colon))),
        deferred_immediate_assertion_statement_parser,
    )
        .map(|(a, b)| DeferredImmediateAssertionItem(a, b))
        .parse_next(input)
}

pub fn procedural_assertion_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProceduralAssertionStatement<'s>, VerboseError<'s>> {
    alt((
        concurrent_assertion_statement_parser
            .map(|a| ProceduralAssertionStatement::Concurrent(Box::new(a))),
        immediate_assertion_statement_parser
            .map(|a| ProceduralAssertionStatement::Immediate(Box::new(a))),
        checker_instantiation_parser
            .map(|a| ProceduralAssertionStatement::Checker(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn immediate_assertion_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ImmediateAssertionStatement<'s>, VerboseError<'s>> {
    alt((
        simple_immediate_assertion_statement_parser
            .map(|a| ImmediateAssertionStatement::Simple(Box::new(a))),
        deferred_immediate_assertion_statement_parser
            .map(|a| ImmediateAssertionStatement::Deferred(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn simple_immediate_assertion_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SimpleImmediateAssertionStatement<'s>, VerboseError<'s>> {
    alt((
        simple_immediate_assert_statement_parser
            .map(|a| SimpleImmediateAssertionStatement::Assert(Box::new(a))),
        simple_immediate_assume_statement_parser
            .map(|a| SimpleImmediateAssertionStatement::Assume(Box::new(a))),
        simple_immediate_cover_statement_parser
            .map(|a| SimpleImmediateAssertionStatement::Cover(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn deferred_immediate_assertion_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DeferredImmediateAssertionStatement<'s>, VerboseError<'s>> {
    alt((
        deferred_immediate_assert_statement_parser
            .map(|a| DeferredImmediateAssertionStatement::Assert(Box::new(a))),
        deferred_immediate_assume_statement_parser
            .map(|a| DeferredImmediateAssertionStatement::Assume(Box::new(a))),
        deferred_immediate_cover_statement_parser
            .map(|a| DeferredImmediateAssertionStatement::Cover(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn simple_immediate_assert_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SimpleImmediateAssertStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Assert),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e)| SimpleImmediateAssertStatement(a, b, c, d, e))
        .parse_next(input)
}

pub fn simple_immediate_assume_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SimpleImmediateAssumeStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Assume),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e)| SimpleImmediateAssumeStatement(a, b, c, d, e))
        .parse_next(input)
}

pub fn simple_immediate_cover_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SimpleImmediateCoverStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Cover),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e)| SimpleImmediateCoverStatement(a, b, c, d, e))
        .parse_next(input)
}

pub fn deferred_immediate_assert_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DeferredImmediateAssertStatement<'s>, VerboseError<'s>> {
    let _now_parser = (
        token(Token::Assert),
        token(Token::Pound),
        unsigned_number_parser
            .verify_map(
                |UnsignedNumber(text, metadata)| {
                    if text == "0" { Some(metadata) } else { None }
                },
            )
            .context("0"),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e, f, g)| {
            DeferredImmediateAssertStatement::Now(Box::new((
                a, b, c, d, e, f, g,
            )))
        });
    let _final_parser = (
        token(Token::Assert),
        token(Token::Final),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e, f)| {
            DeferredImmediateAssertStatement::Final(Box::new((
                a, b, c, d, e, f,
            )))
        });
    alt((_now_parser, _final_parser)).parse_next(input)
}

pub fn deferred_immediate_assume_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DeferredImmediateAssumeStatement<'s>, VerboseError<'s>> {
    let _now_parser = (
        token(Token::Assume),
        token(Token::Pound),
        unsigned_number_parser
            .verify_map(
                |UnsignedNumber(text, metadata)| {
                    if text == "0" { Some(metadata) } else { None }
                },
            )
            .context("0"),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e, f, g)| {
            DeferredImmediateAssumeStatement::Now(Box::new((
                a, b, c, d, e, f, g,
            )))
        });
    let _final_parser = (
        token(Token::Assume),
        token(Token::Final),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e, f)| {
            DeferredImmediateAssumeStatement::Final(Box::new((
                a, b, c, d, e, f,
            )))
        });
    alt((_now_parser, _final_parser)).parse_next(input)
}

pub fn deferred_immediate_cover_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DeferredImmediateCoverStatement<'s>, VerboseError<'s>> {
    let _now_parser = (
        token(Token::Cover),
        token(Token::Pound),
        unsigned_number_parser
            .verify_map(
                |UnsignedNumber(text, metadata)| {
                    if text == "0" { Some(metadata) } else { None }
                },
            )
            .context("0"),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e, f, g)| {
            DeferredImmediateCoverStatement::Now(Box::new((
                a, b, c, d, e, f, g,
            )))
        });
    let _final_parser = (
        token(Token::Cover),
        token(Token::Final),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e, f)| {
            DeferredImmediateCoverStatement::Final(Box::new((a, b, c, d, e, f)))
        });
    alt((_now_parser, _final_parser)).parse_next(input)
}
