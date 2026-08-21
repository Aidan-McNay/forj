// =======================================================================
// procedural_blocks_and_assignments.rs
// =======================================================================
// Parsing for 1800-2023 A.6.2

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn initial_construct_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InitialConstruct<'s>, VerboseError<'s>> {
    (token(Token::Initial), statement_or_null_parser)
        .map(|(a, b)| InitialConstruct(a, b))
        .parse_next(input)
}

pub fn always_construct_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AlwaysConstruct<'s>, VerboseError<'s>> {
    (always_keyword_parser, statement_parser)
        .map(|(a, b)| AlwaysConstruct(a, b))
        .parse_next(input)
}

pub fn always_keyword_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AlwaysKeyword<'s>, VerboseError<'s>> {
    alt((
        token(Token::Always).map(|a| AlwaysKeyword::Always(a)),
        token(Token::AlwaysComb).map(|a| AlwaysKeyword::AlwaysComb(a)),
        token(Token::AlwaysLatch).map(|a| AlwaysKeyword::AlwaysLatch(a)),
        token(Token::AlwaysFf).map(|a| AlwaysKeyword::AlwaysFf(a)),
    ))
    .parse_next(input)
}

pub fn final_construct_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FinalConstruct<'s>, VerboseError<'s>> {
    (token(Token::Final), function_statement_parser)
        .map(|(a, b)| FinalConstruct(a, b))
        .parse_next(input)
}

pub fn implicit_class_handle_or_class_scope_or_package_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<
    ImplicitClassHandleOrClassScopeOrPackageScope<'s>,
    VerboseError<'s>,
> {
    alt((
        (implicit_class_handle_parser, token(Token::Period)).map(|(a, b)| {
            ImplicitClassHandleOrClassScopeOrPackageScope::ImplicitClassHandle(
                Box::new((a, b)),
            )
        }),
        class_scope_parser.map(|a| {
            ImplicitClassHandleOrClassScopeOrPackageScope::Class(Box::new(a))
        }),
        package_scope_parser.map(|a| {
            ImplicitClassHandleOrClassScopeOrPackageScope::Package(Box::new(a))
        }),
    ))
    .parse_next(input)
}

pub fn blocking_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BlockingAssignment<'s>, VerboseError<'s>> {
    let _variable_parser = (
        variable_lvalue_parser,
        token(Token::Eq),
        delay_or_event_control_parser,
        expression_parser,
    )
        .map(|(a, b, c, d)| {
            BlockingAssignment::Variable(Box::new((a, b, c, d)))
        });
    let _dynamic_parser = (
        nonrange_variable_lvalue_parser,
        token(Token::Eq),
        dynamic_array_new_parser,
    )
        .map(|(a, b, c)| BlockingAssignment::Dynamic(Box::new((a, b, c))));
    let _class_parser = (
        opt_note(implicit_class_handle_or_class_scope_or_package_scope_parser),
        hierarchical_variable_identifier_parser,
        select_parser,
        token(Token::Eq),
        class_new_parser,
    )
        .map(|(a, b, c, d, e)| {
            BlockingAssignment::Class(Box::new((a, b, c, d, e)))
        });
    let _operator_parser = operator_assignment_parser
        .map(|a| BlockingAssignment::Operator(Box::new(a)));
    let _inc_or_dec_parser = inc_or_dec_expression_parser
        .map(|a| BlockingAssignment::IncOrDec(Box::new(a)));
    alt((
        _dynamic_parser,
        _class_parser,
        _variable_parser,
        _operator_parser,
        _inc_or_dec_parser,
    ))
    .parse_next(input)
}

pub fn operator_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OperatorAssignment<'s>, VerboseError<'s>> {
    (
        variable_lvalue_parser,
        assignment_operator_parser,
        expression_parser,
    )
        .map(|(a, b, c)| OperatorAssignment(a, b, c))
        .parse_next(input)
}

pub fn assignment_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::Eq).map(|a| AssignmentOperator::Eq(a)),
        token(Token::PlusEq).map(|a| AssignmentOperator::PlusEq(a)),
        token(Token::MinusEq).map(|a| AssignmentOperator::MinusEq(a)),
        token(Token::StarEq).map(|a| AssignmentOperator::StarEq(a)),
        token(Token::SlashEq).map(|a| AssignmentOperator::SlashEq(a)),
        token(Token::PercentEq).map(|a| AssignmentOperator::PercentEq(a)),
        token(Token::AmpEq).map(|a| AssignmentOperator::AmpEq(a)),
        token(Token::PipeEq).map(|a| AssignmentOperator::PipeEq(a)),
        token(Token::CaretEq).map(|a| AssignmentOperator::CaretEq(a)),
        token(Token::LtLtEq).map(|a| AssignmentOperator::LtLtEq(a)),
        token(Token::GtGtEq).map(|a| AssignmentOperator::GtGtEq(a)),
        token(Token::LtLtLtEq).map(|a| AssignmentOperator::LtLtLtEq(a)),
        token(Token::GtGtGtEq).map(|a| AssignmentOperator::GtGtGtEq(a)),
    ))
    .parse_next(input)
}

pub fn nonblocking_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonblockingAssignment<'s>, VerboseError<'s>> {
    (
        variable_lvalue_parser,
        token(Token::LtEq),
        opt_note(delay_or_event_control_parser),
        expression_parser,
    )
        .map(|(a, b, c, d)| NonblockingAssignment(a, b, c, d))
        .parse_next(input)
}

pub fn procedural_continuous_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProceduralContinuousAssignment<'s>, VerboseError<'s>> {
    alt((
        (token(Token::Assign), variable_assignment_parser).map(|(a, b)| {
            ProceduralContinuousAssignment::Assign(Box::new((a, b)))
        }),
        (token(Token::Deassign), variable_lvalue_parser).map(|(a, b)| {
            ProceduralContinuousAssignment::Deassign(Box::new((a, b)))
        }),
        (token(Token::Force), variable_assignment_parser).map(|(a, b)| {
            ProceduralContinuousAssignment::ForceVar(Box::new((a, b)))
        }),
        (token(Token::Force), net_assignment_parser).map(|(a, b)| {
            ProceduralContinuousAssignment::ForceNet(Box::new((a, b)))
        }),
        (token(Token::Release), variable_lvalue_parser).map(|(a, b)| {
            ProceduralContinuousAssignment::ReleaseVar(Box::new((a, b)))
        }),
        (token(Token::Release), net_lvalue_parser).map(|(a, b)| {
            ProceduralContinuousAssignment::ReleaseNet(Box::new((a, b)))
        }),
    ))
    .parse_next(input)
}

pub fn variable_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariableAssignment<'s>, VerboseError<'s>> {
    (variable_lvalue_parser, token(Token::Eq), expression_parser)
        .map(|(a, b, c)| VariableAssignment(a, b, c))
        .parse_next(input)
}
