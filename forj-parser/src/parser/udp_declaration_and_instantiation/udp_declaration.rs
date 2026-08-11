// =======================================================================
// udp_declaration.rs
// =======================================================================
// Parsing for 1800-2023 A.5.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn udp_nonansi_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpNonansiDeclaration<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Primitive),
        udp_identifier_parser,
        token(Token::Paren),
        udp_port_list_parser,
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g)| UdpNonansiDeclaration(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn udp_ansi_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpAnsiDeclaration<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Primitive),
        udp_identifier_parser,
        token(Token::Paren),
        udp_declaration_port_list_parser,
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g)| UdpAnsiDeclaration(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn udp_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpDeclaration<'s>, VerboseError<'s>> {
    let _nonansi_parser = (
        udp_nonansi_declaration_parser,
        udp_port_declaration_parser,
        repeat_note(udp_port_declaration_parser),
        udp_body_parser,
        token(Token::Endprimitive),
        opt_note((token(Token::Colon), udp_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f)| {
            UdpDeclaration::Nonansi(Box::new((a, b, c, d, e, f)))
        });
    let _ansi_parser = (
        udp_ansi_declaration_parser,
        udp_body_parser,
        token(Token::Endprimitive),
        opt_note((token(Token::Colon), udp_identifier_parser)),
    )
        .map(|(a, b, c, d)| UdpDeclaration::Ansi(Box::new((a, b, c, d))));
    let _extern_nonansi_parser =
        (token(Token::Extern), udp_nonansi_declaration_parser)
            .map(|(a, b)| UdpDeclaration::ExternNonansi(Box::new((a, b))));
    let _extern_ansi_parser =
        (token(Token::Extern), udp_ansi_declaration_parser)
            .map(|(a, b)| UdpDeclaration::ExternAnsi(Box::new((a, b))));
    let _widlcard_parser = (
        attribute_instance_vec_parser,
        token(Token::Primitive),
        udp_identifier_parser,
        token(Token::Paren),
        token(Token::Period),
        token(Token::Star),
        token(Token::EParen),
        token(Token::SColon),
        repeat_note(udp_port_declaration_parser),
        udp_body_parser,
        token(Token::Endprimitive),
        opt_note((token(Token::Colon), udp_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            UdpDeclaration::Wildcard(Box::new((
                a, b, c, d, e, f, g, h, i, j, k, l,
            )))
        });
    alt((
        _nonansi_parser,
        _ansi_parser,
        _extern_nonansi_parser,
        _extern_ansi_parser,
        _widlcard_parser,
    ))
    .parse_next(input)
}
