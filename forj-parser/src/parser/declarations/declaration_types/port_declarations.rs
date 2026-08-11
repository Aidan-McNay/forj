// =======================================================================
// port_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.1.2

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn inout_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InoutDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Inout),
        net_port_type_parser,
        list_of_port_identifiers_parser,
    )
        .map(|(a, b, c)| InoutDeclaration(a, b, c))
        .parse_next(input)
}

pub fn input_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InputDeclaration<'s>, VerboseError<'s>> {
    let net_input_declaration_parser = (
        token(Token::Input),
        net_port_type_parser,
        list_of_port_identifiers_parser,
    )
        .map(|(a, b, c)| {
            InputDeclaration::NetInputDeclaration(Box::new((a, b, c)))
        });
    let variable_input_declaration_parser = (
        token(Token::Input),
        variable_port_type_parser,
        list_of_variable_identifiers_parser,
    )
        .map(|(a, b, c)| {
            InputDeclaration::VariableInputDeclaration(Box::new((a, b, c)))
        });
    alt((
        net_input_declaration_parser,
        variable_input_declaration_parser,
    ))
    .parse_next(input)
}

pub fn output_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OutputDeclaration<'s>, VerboseError<'s>> {
    let net_output_declaration_parser = (
        token(Token::Output),
        net_port_type_parser,
        list_of_port_identifiers_parser,
    )
        .map(|(a, b, c)| {
            OutputDeclaration::NetOutputDeclaration(Box::new((a, b, c)))
        });
    let variable_output_declaration_parser = (
        token(Token::Output),
        variable_port_type_parser,
        list_of_variable_port_identifiers_parser,
    )
        .map(|(a, b, c)| {
            OutputDeclaration::VariableOutputDeclaration(Box::new((a, b, c)))
        });
    alt((
        net_output_declaration_parser,
        variable_output_declaration_parser,
    ))
    .parse_next(input)
}

pub fn interface_port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfacePortDeclaration<'s>, VerboseError<'s>> {
    let interface_parser = (
        interface_identifier_parser,
        list_of_interface_identifiers_parser,
    )
        .map(|(a, b)| InterfacePortDeclaration::Interface(Box::new((a, b))));
    let modport_parser = (
        interface_identifier_parser,
        token(Token::Period),
        modport_identifier_parser,
        list_of_interface_identifiers_parser,
    )
        .map(|(a, b, c, d)| {
            InterfacePortDeclaration::Modport(Box::new((a, b, c, d)))
        });
    alt((interface_parser, modport_parser)).parse_next(input)
}

pub fn ref_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RefDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Ref),
        variable_port_type_parser,
        list_of_variable_identifiers_parser,
    )
        .map(|(a, b, c)| RefDeclaration(a, b, c))
        .parse_next(input)
}
