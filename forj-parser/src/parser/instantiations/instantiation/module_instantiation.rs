// =======================================================================
// module_instantiation.rs
// =======================================================================
// Parsing for 1800-2023 A.4.1.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn module_instantiation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleInstantiation<'s>, VerboseError<'s>> {
    (
        module_identifier_parser,
        opt_note(parameter_value_assignment_parser),
        hierarchical_instance_parser,
        repeat_note((token(Token::Comma), hierarchical_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| ModuleInstantiation(a, b, c, d, e))
        .parse_next(input)
}

pub fn parameter_value_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParameterValueAssignment<'s>, VerboseError<'s>> {
    (
        token(Token::Pound),
        token(Token::Paren),
        list_of_parameter_value_assignments_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| ParameterValueAssignment(a, b, c, d))
        .parse_next(input)
}

pub fn list_of_parameter_value_assignments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfParameterValueAssignments<'s>, VerboseError<'s>> {
    let _ordered_ports_parser = (
        ordered_parameter_assignment_parser,
        repeat_note((token(Token::Comma), ordered_parameter_assignment_parser)),
    )
        .map(|(a, b)| {
            ListOfParameterValueAssignments::Ordered(Box::new((a, b)))
        });
    let _named_ports_parser = (
        named_parameter_assignment_parser,
        repeat_note((token(Token::Comma), named_parameter_assignment_parser)),
    )
        .map(|(a, b)| ListOfParameterValueAssignments::Named(Box::new((a, b))));
    alt((_ordered_ports_parser, _named_ports_parser)).parse_next(input)
}

pub fn ordered_parameter_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OrderedParameterAssignment<'s>, VerboseError<'s>> {
    param_expression_parser
        .map(|a| OrderedParameterAssignment(a))
        .parse_next(input)
}

pub fn named_parameter_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NamedParameterAssignment<'s>, VerboseError<'s>> {
    (
        token(Token::Period),
        parameter_identifier_parser,
        token(Token::Paren),
        opt_note(param_expression_parser),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| NamedParameterAssignment(a, b, c, d, e))
        .parse_next(input)
}

pub fn hierarchical_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalInstance<'s>, VerboseError<'s>> {
    (
        name_of_instance_parser,
        token(Token::Paren),
        opt_note(list_of_port_connections_parser),
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| HierarchicalInstance(a, b, c, d))
        .parse_next(input)
}

pub fn name_of_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NameOfInstance<'s>, VerboseError<'s>> {
    (
        instance_identifier_parser,
        repeat_note(unpacked_dimension_parser),
    )
        .map(|(a, b)| NameOfInstance(a, b))
        .parse_next(input)
}

pub fn list_of_port_connections_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfPortConnections<'s>, VerboseError<'s>> {
    let _ordered_ports_parser = (
        ordered_port_connection_parser,
        repeat_note((token(Token::Comma), ordered_port_connection_parser)),
    )
        .map(|(a, b)| ListOfPortConnections::Ordered(Box::new((a, b))));
    let _named_ports_parser = (
        named_port_connection_parser,
        repeat_note((token(Token::Comma), named_port_connection_parser)),
    )
        .map(|(a, b)| ListOfPortConnections::Named(Box::new((a, b))));
    alt((_named_ports_parser, _ordered_ports_parser)).parse_next(input)
}

pub fn ordered_port_connection_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OrderedPortConnection<'s>, VerboseError<'s>> {
    (attribute_instance_vec_parser, opt_note(expression_parser))
        .map(|(a, b)| OrderedPortConnection(a, b))
        .parse_next(input)
}

pub fn named_port_connection_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NamedPortConnection<'s>, VerboseError<'s>> {
    let _named_parser = (
        attribute_instance_vec_parser,
        token(Token::Period),
        port_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(expression_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d)| NamedPortConnection::Named(Box::new((a, b, c, d))));
    let _wildcard_parser = (
        attribute_instance_vec_parser,
        token(Token::Period),
        token(Token::Star),
    )
        .map(|(a, b, c)| NamedPortConnection::Wildcard(Box::new((a, b, c))));
    alt((_named_parser, _wildcard_parser)).parse_next(input)
}
