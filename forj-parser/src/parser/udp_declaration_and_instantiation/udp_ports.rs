// =======================================================================
// udp_ports.rs
// =======================================================================
// Parsing for 1800-2023 A.5.2

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn udp_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpPortList<'s>, VerboseError<'s>> {
    (
        output_port_identifier_parser,
        token(Token::Comma),
        input_port_identifier_parser,
        repeat_note((token(Token::Comma), input_port_identifier_parser)),
    )
        .map(|(a, b, c, d)| UdpPortList(a, b, c, d))
        .parse_next(input)
}

pub fn udp_declaration_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpDeclarationPortList<'s>, VerboseError<'s>> {
    (
        udp_output_declaration_parser,
        token(Token::Comma),
        udp_input_declaration_parser,
        repeat_note((token(Token::Comma), udp_input_declaration_parser)),
    )
        .map(|(a, b, c, d)| UdpDeclarationPortList(a, b, c, d))
        .parse_next(input)
}

pub fn udp_port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpPortDeclaration<'s>, VerboseError<'s>> {
    alt((
        (udp_output_declaration_parser, token(Token::SColon))
            .map(|(a, b)| UdpPortDeclaration::Output(Box::new((a, b)))),
        (udp_input_declaration_parser, token(Token::SColon))
            .map(|(a, b)| UdpPortDeclaration::Input(Box::new((a, b)))),
        (udp_reg_declaration_parser, token(Token::SColon))
            .map(|(a, b)| UdpPortDeclaration::Reg(Box::new((a, b)))),
    ))
    .parse_next(input)
}

pub fn udp_output_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpOutputDeclaration<'s>, VerboseError<'s>> {
    let _combinational_parser = (
        attribute_instance_vec_parser,
        token(Token::Output),
        port_identifier_parser,
    )
        .map(|(a, b, c)| {
            UdpOutputDeclaration::Combinational(Box::new((a, b, c)))
        });
    let _sequential_parser = (
        attribute_instance_vec_parser,
        token(Token::Output),
        token(Token::Reg),
        port_identifier_parser,
        opt_note((token(Token::Eq), constant_expression_parser)),
    )
        .map(|(a, b, c, d, e)| {
            UdpOutputDeclaration::Sequential(Box::new((a, b, c, d, e)))
        });
    alt((_combinational_parser, _sequential_parser)).parse_next(input)
}

pub fn udp_input_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpInputDeclaration<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Input),
        list_of_udp_port_identifiers_parser,
    )
        .map(|(a, b, c)| UdpInputDeclaration(a, b, c))
        .parse_next(input)
}

pub fn udp_reg_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpRegDeclaration<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Reg),
        variable_identifier_parser,
    )
        .map(|(a, b, c)| UdpRegDeclaration(a, b, c))
        .parse_next(input)
}
