// =======================================================================
// utils.rs
// =======================================================================
// Helper functions for implementing parsers

use crate::*;
use forj_syntax::*;
use winnow::Parser;
#[cfg(feature = "parse_lossless")]
use winnow::combinator::alt;
use winnow::error::{ErrMode, ModalResult};
use winnow::token::any;

// A parser for matching extra nodes
#[cfg(feature = "parse_lossless")]
pub fn non_trivia_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Vec<NonTriviaToken<'s>>, VerboseError<'s>> {
    let comment_parser = any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
        Token::OnelineComment(text) => {
            Some(NonTriviaToken::OnelineComment(text, s.1.clone()))
        }
        Token::BlockComment(text) => {
            Some(NonTriviaToken::BlockComment(text, s.1.clone()))
        }
        _ => None,
    });
    let newline_parser = any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
        Token::Newline => Some(NonTriviaToken::Newline),
        _ => None,
    });
    repeat_note(alt((comment_parser, newline_parser))).parse_next(input)
}

#[inline]
#[cfg(not(feature = "parse_lossless"))]
pub fn non_trivia_parser<'s>(
    _: &mut Tokens<'s>,
) -> ModalResult<Vec<NonTriviaToken<'s>>, VerboseError<'s>> {
    Ok(vec![])
}

// A mapping function for replacing extra nodes in metadata
#[cfg(feature = "parse_lossless")]
pub fn replace_non_trivia<'a>(
    old_metadata: Metadata<'a>,
    non_trivia: Vec<NonTriviaToken<'a>>,
) -> Metadata<'a> {
    Metadata::new(old_metadata.span, non_trivia)
}

// A mapping function for replacing extra nodes in metadata
#[inline]
#[cfg(not(feature = "parse_lossless"))]
pub fn replace_non_trivia<'a>(
    old_metadata: Metadata<'a>,
    _: Vec<NonTriviaToken<'a>>,
) -> Metadata<'a> {
    old_metadata
}

// A parser for matching a token and extra nodes, producing metadata
#[cfg(feature = "parse_lossless")]
pub fn token<'s>(
    token_to_match: Token<'s>,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<Metadata<'s>, VerboseError<'s>>
{
    move |input: &mut Tokens<'s>| {
        (token_to_match, non_trivia_parser)
            .context(token_to_match)
            .parse_next(input)
            .map(|(spanned_token, extra_nodes)| {
                Metadata::new(spanned_token.1.clone(), extra_nodes)
            })
    }
}

// A parser for matching a token and extra nodes, producing metadata
#[cfg(not(feature = "parse_lossless"))]
pub fn token<'s>(
    token_to_match: Token<'s>,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<Metadata<'s>, VerboseError<'s>>
{
    move |input: &mut Tokens<'s>| {
        token_to_match
            .context(token_to_match)
            .map(|spanned_token| Metadata::new(spanned_token.1.clone(), vec![]))
            .parse_next(input)
    }
}

// A parser for matching a specific identifier name and extra nodes, producing metadata
#[cfg(feature = "parse_lossless")]
pub fn name<'s>(
    name: &'s str,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<Metadata<'s>, VerboseError<'s>>
{
    move |input: &mut Tokens<'s>| {
        let name_parser = any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::SimpleIdentifier(text) if text == name => Some(s.1.clone()),
            _ => None,
        });
        (name_parser, non_trivia_parser)
            .map(|(span, extra_nodes)| Metadata::new(span, extra_nodes))
            .parse_next(input)
    }
}

// A parser for matching a specific identifier name and extra nodes, producing metadata
#[cfg(not(feature = "parse_lossless"))]
pub fn name<'s>(
    name: &'s str,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<Metadata<'s>, VerboseError<'s>>
{
    move |input: &mut Tokens<'s>| {
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::SimpleIdentifier(text) if text == name => {
                Some(Metadata::new(s.1.clone(), vec![]))
            }
            _ => None,
        })
        .parse_next(input)
    }
}

/// Check that the end identifier for a block matches the beginning
pub(crate) fn check_block_identifiers<'s>(
    begin_id: Option<&Identifier<'s>>,
    end_id: Option<&Identifier<'s>>,
) -> ModalResult<(), VerboseError<'s>> {
    match (begin_id, end_id) {
        (_, None) => Ok(()),
        (Some(begin_block_id), Some(end_block_id)) => {
            match (begin_block_id, end_block_id) {
                (
                    Identifier::SimpleIdentifier((begin_block_id_text, _)),
                    Identifier::SimpleIdentifier((
                        end_block_id_text,
                        end_block_id_metadata,
                    )),
                ) => {
                    if begin_block_id_text != end_block_id_text {
                        Err(ErrMode::Backtrack(VerboseError {
                            span: end_block_id_metadata.span.clone(),
                            found: Some(Token::SimpleIdentifier(
                                (*end_block_id_text).into(),
                            )),
                            reason: VerboseErrorReason::Expected(vec![
                                Expectation::Token(Token::SimpleIdentifier(
                                    (*begin_block_id_text).into(),
                                )),
                            ]),
                        }))
                    } else {
                        Ok(())
                    }
                }
                (
                    Identifier::EscapedIdentifier((begin_block_id_text, _)),
                    Identifier::EscapedIdentifier((
                        end_block_id_text,
                        end_block_id_metadata,
                    )),
                ) => {
                    if begin_block_id_text != end_block_id_text {
                        Err(ErrMode::Backtrack(VerboseError {
                            span: end_block_id_metadata.span.clone(),
                            found: Some(Token::EscapedIdentifier(
                                (*end_block_id_text).into(),
                            )),
                            reason: VerboseErrorReason::Expected(vec![
                                Expectation::Token(Token::EscapedIdentifier(
                                    (*begin_block_id_text).into(),
                                )),
                            ]),
                        }))
                    } else {
                        Ok(())
                    }
                }
                (
                    Identifier::SimpleIdentifier((begin_block_id_text, _)),
                    Identifier::EscapedIdentifier((
                        end_block_id_text,
                        end_block_id_metadata,
                    )),
                ) => Err(ErrMode::Backtrack(VerboseError {
                    span: end_block_id_metadata.span.clone(),
                    found: Some(Token::EscapedIdentifier(
                        (*end_block_id_text).into(),
                    )),
                    reason: VerboseErrorReason::Expected(vec![
                        Expectation::Token(Token::SimpleIdentifier(
                            (*begin_block_id_text).into(),
                        )),
                    ]),
                })),
                (
                    Identifier::EscapedIdentifier((begin_block_id_text, _)),
                    Identifier::SimpleIdentifier((
                        end_block_id_text,
                        end_block_id_metadata,
                    )),
                ) => Err(ErrMode::Backtrack(VerboseError {
                    span: end_block_id_metadata.span.clone(),
                    found: Some(Token::SimpleIdentifier(
                        (*end_block_id_text).into(),
                    )),
                    reason: VerboseErrorReason::Expected(vec![
                        Expectation::Token(Token::EscapedIdentifier(
                            (*begin_block_id_text).into(),
                        )),
                    ]),
                })),
            }
        }
        (None, Some(end_block_id)) => {
            let (err_token, err_span) = match end_block_id {
                Identifier::SimpleIdentifier((text, Metadata { span })) => {
                    (Token::SimpleIdentifier((*text).into()), span.clone())
                }
                Identifier::EscapedIdentifier((text, Metadata { span })) => {
                    (Token::EscapedIdentifier((*text).into()), span.clone())
                }
            };
            Err(ErrMode::Backtrack(VerboseError {
                span: err_span,
                found: Some(err_token),
                reason: VerboseErrorReason::Diagnostic(
                    "no matching beginning block name",
                ),
            }))
        }
    }
}
