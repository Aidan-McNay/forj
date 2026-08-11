// =======================================================================
// statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.4

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn statement_or_null_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StatementOrNull<'s>, VerboseError<'s>> {
    alt((
        statement_parser.map(|a| StatementOrNull::Statement(Box::new(a))),
        (attribute_instance_vec_parser, token(Token::SColon))
            .map(|(a, b)| StatementOrNull::Null(Box::new((a, b)))),
    ))
    .parse_next(input)
}

pub fn statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Statement<'s>, VerboseError<'s>> {
    (
        opt_note((block_identifier_parser, token(Token::Colon))),
        attribute_instance_vec_parser,
        statement_item_parser,
    )
        .map(|(a, b, c)| Statement(a, b, c))
        .parse_next(input)
}

pub fn statement_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StatementItem<'s>, VerboseError<'s>> {
    alt((
        (blocking_assignment_parser, token(Token::SColon))
            .map(|(a, b)| StatementItem::Blocking(Box::new((a, b)))),
        (nonblocking_assignment_parser, token(Token::SColon))
            .map(|(a, b)| StatementItem::Nonblocking(Box::new((a, b)))),
        (
            procedural_continuous_assignment_parser,
            token(Token::SColon),
        )
            .map(|(a, b)| {
                StatementItem::ProceduralContinuous(Box::new((a, b)))
            }),
        case_statement_parser.map(|a| StatementItem::Case(Box::new(a))),
        conditional_statement_parser
            .map(|a| StatementItem::Conditional(Box::new(a))),
        subroutine_call_statement_parser
            .map(|a| StatementItem::SubroutineCall(Box::new(a))),
        disable_statement_parser.map(|a| StatementItem::Disable(Box::new(a))),
        event_trigger_parser.map(|a| StatementItem::Event(Box::new(a))),
        loop_statement_parser.map(|a| StatementItem::Loop(Box::new(a))),
        jump_statement_parser.map(|a| StatementItem::Jump(Box::new(a))),
        par_block_parser.map(|a| StatementItem::Par(Box::new(a))),
        procedural_timing_control_statement_parser
            .map(|a| StatementItem::ProceduralTimingControl(Box::new(a))),
        seq_block_parser.map(|a| StatementItem::Seq(Box::new(a))),
        wait_statement_parser.map(|a| StatementItem::Wait(Box::new(a))),
        procedural_assertion_statement_parser
            .map(|a| StatementItem::ProceduralAssertion(Box::new(a))),
        (clocking_drive_parser, token(Token::SColon))
            .map(|(a, b)| StatementItem::Clocking(Box::new((a, b)))),
        randsequence_statement_parser
            .map(|a| StatementItem::Randsequence(Box::new(a))),
        randcase_statement_parser.map(|a| StatementItem::Randcase(Box::new(a))),
        expect_property_statement_parser
            .map(|a| StatementItem::Expect(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn function_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionStatement<'s>, VerboseError<'s>> {
    statement_parser
        .map(|a| FunctionStatement(a))
        .parse_next(input)
}

pub fn function_statement_or_null_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionStatementOrNull<'s>, VerboseError<'s>> {
    alt((
        function_statement_parser
            .map(|a| FunctionStatementOrNull::FunctionStatement(Box::new(a))),
        (attribute_instance_vec_parser, token(Token::SColon))
            .map(|(a, b)| FunctionStatementOrNull::Null(Box::new((a, b)))),
    ))
    .parse_next(input)
}
