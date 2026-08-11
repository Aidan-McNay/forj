// =======================================================================
// declaration_lists.rs
// =======================================================================
// Parsing for 1800-2023 A.2.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;

pub fn list_of_defparam_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfDefparamAssignments<'s>, VerboseError<'s>> {
    (
        defparam_assignment_parser,
        repeat_note((token(Token::Comma), defparam_assignment_parser)),
    )
        .map(|(a, b)| ListOfDefparamAssignments(a, b))
        .parse_next(input)
}

pub fn list_of_genvar_identifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfGenvarIdentifiers<'s>, VerboseError<'s>> {
    (
        genvar_identifier_parser,
        repeat_note((token(Token::Comma), genvar_identifier_parser)),
    )
        .map(|(a, b)| ListOfGenvarIdentifiers(a, b))
        .parse_next(input)
}

pub fn list_of_interface_identifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfInterfaceIdentifiers<'s>, VerboseError<'s>> {
    (
        interface_identifier_parser,
        repeat_note(unpacked_dimension_parser),
        repeat_note((
            token(Token::Comma),
            interface_identifier_parser,
            repeat_note(unpacked_dimension_parser),
        )),
    )
        .map(|(a, b, c)| ListOfInterfaceIdentifiers(a, b, c))
        .parse_next(input)
}

pub fn list_of_net_decl_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfNetDeclAssignments<'s>, VerboseError<'s>> {
    (
        net_decl_assignment_parser,
        repeat_note((token(Token::Comma), net_decl_assignment_parser)),
    )
        .map(|(a, b)| ListOfNetDeclAssignments(a, b))
        .parse_next(input)
}

pub fn list_of_param_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfParamAssignments<'s>, VerboseError<'s>> {
    (
        param_assignment_parser,
        repeat_note((token(Token::Comma), param_assignment_parser)),
    )
        .map(|(a, b)| ListOfParamAssignments(a, b))
        .parse_next(input)
}

pub fn list_of_port_identifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfPortIdentifiers<'s>, VerboseError<'s>> {
    (
        port_identifier_parser,
        repeat_note(unpacked_dimension_parser),
        repeat_note((
            token(Token::Comma),
            port_identifier_parser,
            repeat_note(unpacked_dimension_parser),
        )),
    )
        .map(|(a, b, c)| ListOfPortIdentifiers(a, b, c))
        .parse_next(input)
}

pub fn list_of_udp_port_identifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfUdpPortIdentifiers<'s>, VerboseError<'s>> {
    (
        port_identifier_parser,
        repeat_note((token(Token::Comma), port_identifier_parser)),
    )
        .map(|(a, b)| ListOfUdpPortIdentifiers(a, b))
        .parse_next(input)
}

pub fn list_of_specparam_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfSpecparamAssignments<'s>, VerboseError<'s>> {
    (
        specparam_assignment_parser,
        repeat_note((token(Token::Comma), specparam_assignment_parser)),
    )
        .map(|(a, b)| ListOfSpecparamAssignments(a, b))
        .parse_next(input)
}

pub fn list_of_tf_variable_identifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfTfVariableIdentifiers<'s>, VerboseError<'s>> {
    (
        port_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), expression_parser)),
        repeat_note((
            token(Token::Comma),
            port_identifier_parser,
            repeat_note(variable_dimension_parser),
            opt_note((token(Token::Eq), expression_parser)),
        )),
    )
        .map(|(a, b, c, d)| ListOfTfVariableIdentifiers(a, b, c, d))
        .parse_next(input)
}

pub fn list_of_type_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfTypeAssignments<'s>, VerboseError<'s>> {
    (
        type_assignment_parser,
        repeat_note((token(Token::Comma), type_assignment_parser)),
    )
        .map(|(a, b)| ListOfTypeAssignments(a, b))
        .parse_next(input)
}

pub fn list_of_variable_decl_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfVariableDeclAssignments<'s>, VerboseError<'s>> {
    (
        variable_decl_assignment_parser,
        repeat_note((token(Token::Comma), variable_decl_assignment_parser)),
    )
        .map(|(a, b)| ListOfVariableDeclAssignments(a, b))
        .parse_next(input)
}

pub fn list_of_variable_identifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfVariableIdentifiers<'s>, VerboseError<'s>> {
    (
        variable_identifier_parser,
        repeat_note(variable_dimension_parser),
        repeat_note((
            token(Token::Comma),
            variable_identifier_parser,
            repeat_note(variable_dimension_parser),
        )),
    )
        .map(|(a, b, c)| ListOfVariableIdentifiers(a, b, c))
        .parse_next(input)
}

pub fn list_of_variable_port_identifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfVariablePortIdentifiers<'s>, VerboseError<'s>> {
    (
        port_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), constant_expression_parser)),
        repeat_note((
            token(Token::Comma),
            port_identifier_parser,
            repeat_note(variable_dimension_parser),
            opt_note((token(Token::Eq), constant_expression_parser)),
        )),
    )
        .map(|(a, b, c, d)| ListOfVariablePortIdentifiers(a, b, c, d))
        .parse_next(input)
}
