// =======================================================================
// udp_instantiation.rs
// =======================================================================
// Parsing for 1800-2023 A.5.4

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;

pub fn udp_instantiation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpInstantiation<'s>, VerboseError<'s>> {
    (
        udp_identifier_parser,
        opt_note(drive_strength_parser),
        opt_note(delay2_parser),
        udp_instance_parser,
        repeat_note((token(Token::Comma), udp_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| UdpInstantiation(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn udp_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        output_terminal_parser,
        token(Token::Comma),
        input_terminal_parser,
        repeat_note((token(Token::Comma), input_terminal_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g)| UdpInstance(a, b, c, d, e, f, g))
        .parse_next(input)
}
