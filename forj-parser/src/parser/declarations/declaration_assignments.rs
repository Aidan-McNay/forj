// =======================================================================
// declaration_assignments.rs
// =======================================================================
// Parsing for 1800-2023 A.2.4

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, not, peek, terminated};

pub fn defparam_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DefparamAssignment<'s>, VerboseError<'s>> {
    (
        hierarchical_parameter_identifier_parser,
        token(Token::Eq),
        constant_mintypmax_expression_parser,
    )
        .map(|(a, b, c)| DefparamAssignment(a, b, c))
        .parse_next(input)
}

pub fn net_decl_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetDeclAssignment<'s>, VerboseError<'s>> {
    (
        net_identifier_parser,
        repeat_note(unpacked_dimension_parser),
        opt_note((token(Token::Eq), expression_parser)),
    )
        .map(|(a, b, c)| NetDeclAssignment(a, b, c))
        .parse_next(input)
}

pub fn param_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParamAssignment<'s>, VerboseError<'s>> {
    (
        parameter_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), constant_param_expression_parser)),
    )
        .map(|(a, b, c)| ParamAssignment(a, b, c))
        .parse_next(input)
}

pub fn specparam_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecparamAssignment<'s>, VerboseError<'s>> {
    let _base_parser = (
        specparam_identifier_parser,
        token(Token::Eq),
        constant_mintypmax_expression_parser,
    )
        .map(|(a, b, c)| SpecparamAssignment::Base(Box::new((a, b, c))));
    let _pulse_parser = pulse_control_specparam_parser
        .map(|a| SpecparamAssignment::Pulse(Box::new(a)));
    alt((_base_parser, _pulse_parser)).parse_next(input)
}

fn pathpulse_pathspecific_path_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Identifier<'s>, VerboseError<'s>> {
    identifier_parser
        .verify_map(|a| match a {
            Identifier::EscapedIdentifier(_) => None,
            Identifier::SimpleIdentifier((text, metadata)) => {
                if !text.starts_with("PATHPULSE$") {
                    None
                } else {
                    if text.chars().filter(|&c| c == '$').count() != 2 {
                        None
                    } else {
                        Some(Identifier::SimpleIdentifier((text, metadata)))
                    }
                }
            }
        })
        .context("PATHPULSE path")
        .parse_next(input)
}

pub fn pulse_control_specparam_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PulseControlSpecparam<'s>, VerboseError<'s>> {
    let _nonpath_specific_parser = (
        token(Token::PathpulseDollar),
        token(Token::Eq),
        token(Token::Paren),
        reject_limit_value_parser,
        opt_note((token(Token::Comma), error_limit_value_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| {
            PulseControlSpecparam::NonpathSpecific(Box::new((a, b, c, d, e, f)))
        });
    let _path_specific_parser = (
        pathpulse_pathspecific_path_parser,
        token(Token::Eq),
        token(Token::Paren),
        reject_limit_value_parser,
        opt_note((token(Token::Comma), error_limit_value_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| {
            PulseControlSpecparam::PathSpecific(Box::new((a, b, c, d, e, f)))
        });
    alt((_nonpath_specific_parser, _path_specific_parser)).parse_next(input)
}

pub fn error_limit_value_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ErrorLimitValue<'s>, VerboseError<'s>> {
    limit_value_parser
        .map(|a| ErrorLimitValue(a))
        .parse_next(input)
}

pub fn reject_limit_value_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RejectLimitValue<'s>, VerboseError<'s>> {
    limit_value_parser
        .map(|a| RejectLimitValue(a))
        .parse_next(input)
}

pub fn limit_value_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LimitValue<'s>, VerboseError<'s>> {
    constant_mintypmax_expression_parser
        .map(|a| LimitValue(a))
        .parse_next(input)
}

pub fn type_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TypeAssignment<'s>, VerboseError<'s>> {
    (
        type_identifier_parser,
        opt_note((
            token(Token::Eq),
            data_type_or_incomplete_class_scoped_type_parser,
        )),
    )
        .map(|(a, b)| TypeAssignment(a, b))
        .parse_next(input)
}

pub fn variable_decl_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariableDeclAssignment<'s>, VerboseError<'s>> {
    let _variable_parser = (
        variable_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), expression_parser)),
    )
        .map(|(a, b, c)| VariableDeclAssignment::Variable(Box::new((a, b, c))));
    let _dynamic_variable_parser = terminated(
        (
            dynamic_array_variable_identifier_parser,
            unsized_dimension_parser,
            repeat_note(variable_dimension_parser),
            opt_note((token(Token::Eq), dynamic_array_new_parser)),
        ),
        peek(not(token(Token::Eq))),
    )
    .map(|(a, b, c, d)| {
        VariableDeclAssignment::DynamicVariable(Box::new((a, b, c, d)))
    });
    let _class_variable_parser = terminated(
        (
            class_variable_identifier_parser,
            ((token(Token::Eq), class_new_parser)), // Optional, but gets picked up by variable parser
        ),
        peek(not(token(Token::Eq))),
    )
    .map(|(a, b)| {
        VariableDeclAssignment::ClassVariable(Box::new((a, Some(b))))
    });
    alt((
        _dynamic_variable_parser,
        _class_variable_parser,
        _variable_parser,
    ))
    .parse_next(input)
}

pub fn class_new_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassNew<'s>, VerboseError<'s>> {
    let _args_parser = (
        opt_note(class_scope_parser),
        token(Token::New),
        opt_note((
            token(Token::Paren),
            list_of_arguments_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c)| ClassNew::Args(Box::new((a, b, c))));
    let _expression_parser = (token(Token::New), expression_parser)
        .map(|(a, b)| ClassNew::Expression(Box::new((a, b))));
    alt((_args_parser, _expression_parser)).parse_next(input)
}

pub fn dynamic_array_new_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DynamicArrayNew<'s>, VerboseError<'s>> {
    (
        token(Token::New),
        token(Token::Bracket),
        expression_parser,
        token(Token::EBracket),
        opt_note((
            token(Token::Paren),
            expression_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d, e)| DynamicArrayNew(a, b, c, d, e))
        .parse_next(input)
}
