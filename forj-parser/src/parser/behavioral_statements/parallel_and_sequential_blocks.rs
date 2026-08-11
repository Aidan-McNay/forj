// =======================================================================
// parallel_and_sequential_blocks.rs
// =======================================================================
// Parsing for 1800-2023 A.6.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn action_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ActionBlock<'s>, VerboseError<'s>> {
    alt((
        statement_or_null_parser.map(|a| ActionBlock::Basic(Box::new(a))),
        (
            opt_note(statement_parser),
            token(Token::Else),
            statement_or_null_parser,
        )
            .map(|(a, b, c)| ActionBlock::Conditional(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn seq_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SeqBlock<'s>, VerboseError<'s>> {
    (
        token(Token::Begin),
        opt_note((token(Token::Colon), block_identifier_parser)),
        repeat_note(block_item_declaration_parser),
        repeat_note(statement_or_null_parser),
        token(Token::End),
        opt_note((token(Token::Colon), block_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f)| SeqBlock(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn par_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParBlock<'s>, VerboseError<'s>> {
    (
        token(Token::Fork),
        opt_note((token(Token::Colon), block_identifier_parser)),
        repeat_note(block_item_declaration_parser),
        repeat_note(statement_or_null_parser),
        join_keyword_parser,
        opt_note((token(Token::Colon), block_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f)| ParBlock(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn join_keyword_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<JoinKeyword<'s>, VerboseError<'s>> {
    alt((
        token(Token::Join).map(|a| JoinKeyword::Join(a)),
        token(Token::JoinAny).map(|a| JoinKeyword::JoinAny(a)),
        token(Token::JoinNone).map(|a| JoinKeyword::JoinNone(a)),
    ))
    .parse_next(input)
}
