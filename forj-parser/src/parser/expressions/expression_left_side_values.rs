// =======================================================================
// expression_left_side_values.rs
// =======================================================================
// Parsing for 1800-2023 A.8.5

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn net_lvalue_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetLvalue<'s>, VerboseError<'s>> {
    let _selection_net_lvalue_parser = (
        ps_or_hierarchical_net_identifier_parser,
        constant_select_parser,
    )
        .map(|(a, b)| NetLvalue::Selection(Box::new(SelectionNetLvalue(a, b))));
    let _nested_net_lvalue_parser = (
        token(Token::Brace),
        net_lvalue_parser,
        repeat_note((token(Token::Comma), net_lvalue_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| {
            NetLvalue::Nested(Box::new(NestedNetLvalue(a, b, c, d)))
        });
    let _assignment_net_lvalue_parser = (
        opt_note(assignment_pattern_expression_type_parser),
        assignment_pattern_net_lvalue_parser,
    )
        .map(|(a, b)| {
            NetLvalue::Assignment(Box::new(AssignmentNetLvalue(a, b)))
        });
    alt((
        _selection_net_lvalue_parser,
        _nested_net_lvalue_parser,
        _assignment_net_lvalue_parser,
    ))
    .parse_next(input)
}

fn implicit_class_handle_or_package_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ImplicitClassHandleOrPackageScope<'s>, VerboseError<'s>> {
    alt((
        (implicit_class_handle_parser, token(Token::Period)).map(|(a, b)| {
            ImplicitClassHandleOrPackageScope::ImplicitClassHandle(Box::new((
                a, b,
            )))
        }),
        package_scope_parser.map(|a| {
            ImplicitClassHandleOrPackageScope::PackageScope(Box::new(a))
        }),
    ))
    .parse_next(input)
}

pub fn variable_lvalue_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariableLvalue<'s>, VerboseError<'s>> {
    let _selection_variable_lvalue_parser = (
        opt_note(implicit_class_handle_or_package_scope_parser),
        hierarchical_variable_identifier_parser,
        select_parser,
    )
        .map(|(a, b, c)| {
            VariableLvalue::Selection(Box::new(SelectionVariableLvalue(
                a, b, c,
            )))
        });
    let _nested_variable_lvalue_parser = (
        token(Token::Brace),
        variable_lvalue_parser,
        repeat_note((token(Token::Comma), variable_lvalue_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| {
            VariableLvalue::Nested(Box::new(NestedVariableLvalue(a, b, c, d)))
        });
    let _assignment_variable_lvalue_parser = (
        opt_note(assignment_pattern_expression_type_parser),
        assignment_pattern_variable_lvalue_parser,
    )
        .map(|(a, b)| {
            VariableLvalue::Assignment(Box::new(AssignmentVariableLvalue(a, b)))
        });
    let _streaming_variable_lvalue_parser = streaming_concatenation_parser
        .map(|a| VariableLvalue::Streaming(Box::new(a)));
    alt((
        _selection_variable_lvalue_parser,
        _nested_variable_lvalue_parser,
        _assignment_variable_lvalue_parser,
        _streaming_variable_lvalue_parser,
    ))
    .parse_next(input)
}

pub fn nonrange_variable_lvalue_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonrangeVariableLvalue<'s>, VerboseError<'s>> {
    (
        opt_note(implicit_class_handle_or_package_scope_parser),
        hierarchical_variable_identifier_parser,
        nonrange_select_parser,
    )
        .map(|(a, b, c)| NonrangeVariableLvalue(a, b, c))
        .parse_next(input)
}
