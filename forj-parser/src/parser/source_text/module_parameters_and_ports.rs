// =======================================================================
// module_parameters_and_ports.rs
// =======================================================================
// Parsing for 1800-2023 A.1.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn parameter_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParameterPortList<'s>, VerboseError<'s>> {
    let defaults_parser = (
        token(Token::Pound),
        token(Token::Paren),
        list_of_param_assignments_parser,
        repeat_note((token(Token::Comma), parameter_port_declaration_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| ParameterPortList::Defaults(a, b, c, d, e));
    let no_defaults_parser = (
        token(Token::Pound),
        token(Token::Paren),
        parameter_port_declaration_parser,
        repeat_note((token(Token::Comma), parameter_port_declaration_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| ParameterPortList::NoDefaults(a, b, c, d, e));
    let empty_parser = (
        token(Token::Pound),
        token(Token::Paren),
        token(Token::EParen),
    )
        .map(|(a, b, c)| ParameterPortList::Empty(a, b, c));
    alt((defaults_parser, no_defaults_parser, empty_parser)).parse_next(input)
}

pub fn parameter_port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParameterPortDeclaration<'s>, VerboseError<'s>> {
    alt((
        parameter_declaration_parser.map(|a| {
            ParameterPortDeclaration::ParameterDeclaration(Box::new(a))
        }),
        local_parameter_declaration_parser.map(|a| {
            ParameterPortDeclaration::LocalParameterDeclaration(Box::new(a))
        }),
        (data_type_parser, list_of_param_assignments_parser).map(|(a, b)| {
            ParameterPortDeclaration::DataAssignments(Box::new((a, b)))
        }),
        type_parameter_declaration_parser.map(|a| {
            ParameterPortDeclaration::TypeParameterDeclaration(Box::new(a))
        }),
    ))
    .parse_next(input)
}

pub fn list_of_ports_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfPorts<'s>, VerboseError<'s>> {
    (
        token(Token::Paren),
        port_parser,
        repeat_note((token(Token::Comma), port_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| ListOfPorts(a, b, c, d))
        .parse_next(input)
}

pub fn list_of_port_declarations_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfPortDeclarations<'s>, VerboseError<'s>> {
    (
        token(Token::Paren),
        opt_note((
            attribute_instance_vec_parser,
            ansi_port_declaration_parser,
            repeat_note((
                token(Token::Comma),
                attribute_instance_vec_parser,
                ansi_port_declaration_parser,
            )),
        )),
        token(Token::EParen),
    )
        .map(|(a, b, c)| ListOfPortDeclarations(a, b, c))
        .parse_next(input)
}

pub fn port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PortDeclaration<'s>, VerboseError<'s>> {
    let _inout_declaration_parser =
        (attribute_instance_vec_parser, inout_declaration_parser)
            .map(|(a, b)| PortDeclaration::InoutDeclaration(Box::new((a, b))));
    let _input_declaration_parser =
        (attribute_instance_vec_parser, input_declaration_parser)
            .map(|(a, b)| PortDeclaration::InputDeclaration(Box::new((a, b))));
    let _output_declaration_parser =
        (attribute_instance_vec_parser, output_declaration_parser)
            .map(|(a, b)| PortDeclaration::OutputDeclaration(Box::new((a, b))));
    let _ref_declaration_parser =
        (attribute_instance_vec_parser, ref_declaration_parser)
            .map(|(a, b)| PortDeclaration::RefDeclaration(Box::new((a, b))));
    let _interface_port_declaration_parser = (
        attribute_instance_vec_parser,
        interface_port_declaration_parser,
    )
        .map(|(a, b)| {
            PortDeclaration::InterfacePortDeclaration(Box::new((a, b)))
        });
    alt((
        _inout_declaration_parser,
        _input_declaration_parser,
        _output_declaration_parser,
        _ref_declaration_parser,
        _interface_port_declaration_parser,
    ))
    .parse_next(input)
}

pub fn port_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Port<'s>, VerboseError<'s>> {
    let _port_expression_parser = opt_note(port_expression_parser)
        .map(|a| Port::PortExpression(Box::new(a)));
    let _port_identifier_parser = (
        token(Token::Period),
        port_identifier_parser,
        token(Token::Paren),
        opt_note(port_expression_parser),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| Port::PortIdentifier(Box::new((a, b, c, d, e))));
    alt((_port_expression_parser, _port_identifier_parser)).parse_next(input)
}

pub fn port_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PortExpression<'s>, VerboseError<'s>> {
    let single_port_reference_parser = port_reference_parser
        .map(|a| PortExpression::SinglePortReference(Box::new(a)));
    let multi_port_reference_parser = (
        token(Token::Brace),
        port_reference_parser,
        repeat_note((token(Token::Comma), port_reference_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| {
            PortExpression::MultiPortReference(Box::new((a, b, c, d)))
        });
    alt((single_port_reference_parser, multi_port_reference_parser))
        .parse_next(input)
}

pub fn port_reference_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PortReference<'s>, VerboseError<'s>> {
    (port_identifier_parser, constant_select_parser)
        .map(|(a, b)| PortReference(a, b))
        .parse_next(input)
}

pub fn port_direction_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PortDirection<'s>, VerboseError<'s>> {
    alt((
        token(Token::Input).map(|a| PortDirection::Input(a)),
        token(Token::Output).map(|a| PortDirection::Output(a)),
        token(Token::Inout).map(|a| PortDirection::Inout(a)),
        token(Token::Ref).map(|a| PortDirection::Ref(a)),
    ))
    .parse_next(input)
}

pub fn net_port_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetPortHeader<'s>, VerboseError<'s>> {
    (opt_note(port_direction_parser), net_port_type_parser)
        .map(|(a, b)| NetPortHeader(a, b))
        .parse_next(input)
}

pub fn variable_port_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariablePortHeader<'s>, VerboseError<'s>> {
    (opt_note(port_direction_parser), variable_port_type_parser)
        .map(|(a, b)| VariablePortHeader(a, b))
        .parse_next(input)
}

pub fn interface_port_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfacePortHeader<'s>, VerboseError<'s>> {
    let _interface_identifier_parser = (
        interface_identifier_parser,
        opt_note((token(Token::Period), modport_identifier_parser)),
    )
        .map(|(a, b)| InterfacePortHeader::InterfaceIdentifier((a, b)));
    let _interface_parser = (
        token(Token::Interface),
        opt_note((token(Token::Period), modport_identifier_parser)),
    )
        .map(|(a, b)| InterfacePortHeader::Interface((a, b)));
    alt((_interface_identifier_parser, _interface_parser)).parse_next(input)
}

pub fn ansi_port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AnsiPortDeclaration<'s>, VerboseError<'s>> {
    alt((
        ansi_net_port_declaration_parser
            .map(|a| AnsiPortDeclaration::NetPort(Box::new(a))),
        ansi_variable_port_declaration_parser
            .map(|a| AnsiPortDeclaration::VariablePort(Box::new(a))),
        ansi_constant_port_declaration_parser
            .map(|a| AnsiPortDeclaration::ConstantPort(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn ansi_net_port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AnsiNetPortDeclaration<'s>, VerboseError<'s>> {
    let net_or_interface_port_header_parser = alt((
        interface_port_header_parser.map(|a| {
            NetOrInterfacePortHeader::InterfacePortHeader(Box::new(a))
        }),
        net_port_header_parser
            .map(|a| NetOrInterfacePortHeader::NetPortHeader(Box::new(a))),
    ));
    (
        opt_note(net_or_interface_port_header_parser),
        port_identifier_parser,
        repeat_note(unpacked_dimension_parser),
        opt_note((token(Token::Eq), constant_expression_parser)),
    )
        .map(|(a, b, c, d)| AnsiNetPortDeclaration(a, b, c, d))
        .parse_next(input)
}

pub fn ansi_variable_port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AnsiVariablePortDeclaration<'s>, VerboseError<'s>> {
    (
        opt_note(variable_port_header_parser),
        port_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), constant_expression_parser)),
    )
        .map(|(a, b, c, d)| AnsiVariablePortDeclaration(a, b, c, d))
        .parse_next(input)
}

pub fn ansi_constant_port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AnsiConstantPortDeclaration<'s>, VerboseError<'s>> {
    (
        opt_note(port_direction_parser),
        token(Token::Period),
        port_identifier_parser,
        token(Token::Paren),
        opt_note(expression_parser),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| AnsiConstantPortDeclaration(a, b, c, d, e, f))
        .parse_next(input)
}
