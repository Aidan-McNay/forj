// =======================================================================
// continuous_assignment_and_net_alias_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn continuous_assign_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ContinuousAssign<'s>, VerboseError<'s>> {
    let _net_assignments_parser = (
        token(Token::Assign),
        opt_note(drive_strength_parser),
        opt_note(delay3_parser),
        list_of_net_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            ContinuousAssign::NetAssignments(Box::new((a, b, c, d, e)))
        });
    let _variable_assignments_parser = (
        token(Token::Assign),
        opt_note(delay_control_parser),
        list_of_variable_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| {
            ContinuousAssign::VariableAssignments(Box::new((a, b, c, d)))
        });
    alt((_net_assignments_parser, _variable_assignments_parser))
        .parse_next(input)
}

pub fn list_of_net_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfNetAssignments<'s>, VerboseError<'s>> {
    (
        net_assignment_parser,
        repeat_note((token(Token::Comma), net_assignment_parser)),
    )
        .map(|(a, b)| ListOfNetAssignments(a, b))
        .parse_next(input)
}

pub fn list_of_variable_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfVariableAssignments<'s>, VerboseError<'s>> {
    (
        variable_assignment_parser,
        repeat_note((token(Token::Comma), variable_assignment_parser)),
    )
        .map(|(a, b)| ListOfVariableAssignments(a, b))
        .parse_next(input)
}

pub fn net_alias_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetAlias<'s>, VerboseError<'s>> {
    (
        token(Token::Alias),
        net_lvalue_parser,
        token(Token::Eq),
        net_lvalue_parser,
        repeat_note((token(Token::Eq), net_lvalue_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| NetAlias(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn net_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetAssignment<'s>, VerboseError<'s>> {
    (net_lvalue_parser, token(Token::Eq), expression_parser)
        .map(|(a, b, c)| NetAssignment(a, b, c))
        .parse_next(input)
}
