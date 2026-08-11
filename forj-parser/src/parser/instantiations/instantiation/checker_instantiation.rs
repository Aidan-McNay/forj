// =======================================================================
// checker_instantiation.rs
// =======================================================================
// Parsing for 1800-2023 A.4.1.4

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn checker_instantiation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerInstantiation<'s>, VerboseError<'s>> {
    (
        ps_checker_identifier_parser,
        name_of_instance_parser,
        token(Token::Paren),
        opt_note(list_of_checker_port_connections_parser),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| CheckerInstantiation(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn list_of_checker_port_connections_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfCheckerPortConnections<'s>, VerboseError<'s>> {
    let _ordered_parser = (
        ordered_checker_port_connection_parser,
        repeat_note((
            token(Token::Comma),
            ordered_checker_port_connection_parser,
        )),
    )
        .map(|(a, b)| ListOfCheckerPortConnections::Ordered(Box::new((a, b))));
    let _named_parser = (
        named_checker_port_connection_parser,
        repeat_note((
            token(Token::Comma),
            named_checker_port_connection_parser,
        )),
    )
        .map(|(a, b)| ListOfCheckerPortConnections::Named(Box::new((a, b))));
    alt((_named_parser, _ordered_parser)).parse_next(input)
}

pub fn ordered_checker_port_connection_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OrderedCheckerPortConnection<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        opt_note(property_actual_arg_parser),
    )
        .map(|(a, b)| OrderedCheckerPortConnection(a, b))
        .parse_next(input)
}

pub fn named_checker_port_connection_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NamedCheckerPortConnection<'s>, VerboseError<'s>> {
    let _identifier_parser = (
        attribute_instance_vec_parser,
        token(Token::Period),
        formal_port_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(property_actual_arg_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d)| {
            NamedCheckerPortConnection::Identifier(Box::new((a, b, c, d)))
        });
    let _wildcard_parser = (
        attribute_instance_vec_parser,
        token(Token::Period),
        token(Token::Star),
    )
        .map(|(a, b, c)| {
            NamedCheckerPortConnection::Wildcard(Box::new((a, b, c)))
        });
    alt((_identifier_parser, _wildcard_parser)).parse_next(input)
}
