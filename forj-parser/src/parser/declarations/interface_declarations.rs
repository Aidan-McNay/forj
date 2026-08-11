// =======================================================================
// interface_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.9

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn modport_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Modport),
        modport_item_parser,
        repeat_note((token(Token::Comma), modport_item_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| ModportDeclaration(a, b, c, d))
        .parse_next(input)
}

pub fn modport_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportItem<'s>, VerboseError<'s>> {
    (
        modport_identifier_parser,
        token(Token::Paren),
        modport_ports_declaration_parser,
        repeat_note((token(Token::Comma), modport_ports_declaration_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| ModportItem(a, b, c, d, e))
        .parse_next(input)
}

enum ModportsPortsDeclarationBody<'a> {
    Simple(ModportSimplePortsDeclaration<'a>),
    Tf(ModportTfPortsDeclaration<'a>),
    Clocking(ModportClockingDeclaration<'a>),
}

pub fn modport_ports_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportPortsDeclaration<'s>, VerboseError<'s>> {
    let _body_parser = alt((
        modport_simple_ports_declaration_parser
            .map(|a| ModportsPortsDeclarationBody::Simple(a)),
        modport_tf_ports_declaration_parser
            .map(|a| ModportsPortsDeclarationBody::Tf(a)),
        modport_clocking_declaration_parser
            .map(|a| ModportsPortsDeclarationBody::Clocking(a)),
    ));
    (attribute_instance_vec_parser, _body_parser)
        .map(|(a, b)| match b {
            ModportsPortsDeclarationBody::Simple(c) => {
                ModportPortsDeclaration::Simple(Box::new((a, c)))
            }
            ModportsPortsDeclarationBody::Tf(c) => {
                ModportPortsDeclaration::Tf(Box::new((a, c)))
            }
            ModportsPortsDeclarationBody::Clocking(c) => {
                ModportPortsDeclaration::Clocking(Box::new((a, c)))
            }
        })
        .parse_next(input)
}

pub fn modport_clocking_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportClockingDeclaration<'s>, VerboseError<'s>> {
    (token(Token::Clocking), clocking_identifier_parser)
        .map(|(a, b)| ModportClockingDeclaration(a, b))
        .parse_next(input)
}

pub fn modport_simple_ports_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportSimplePortsDeclaration<'s>, VerboseError<'s>> {
    (
        port_direction_parser,
        modport_simple_port_parser,
        repeat_note((token(Token::Comma), modport_simple_port_parser)),
    )
        .map(|(a, b, c)| ModportSimplePortsDeclaration(a, b, c))
        .parse_next(input)
}

pub fn modport_simple_port_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportSimplePort<'s>, VerboseError<'s>> {
    alt((
        port_identifier_parser.map(|a| ModportSimplePort::Name(Box::new(a))),
        (
            token(Token::Period),
            port_identifier_parser,
            token(Token::Paren),
            opt_note(expression_parser),
            token(Token::EParen),
        )
            .map(|(a, b, c, d, e)| {
                ModportSimplePort::Expression(Box::new((a, b, c, d, e)))
            }),
    ))
    .parse_next(input)
}

pub fn modport_tf_ports_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportTfPortsDeclaration<'s>, VerboseError<'s>> {
    (
        import_export_parser,
        modport_tf_port_parser,
        repeat_note((token(Token::Comma), modport_tf_port_parser)),
    )
        .map(|(a, b, c)| ModportTfPortsDeclaration(a, b, c))
        .parse_next(input)
}

pub fn modport_tf_port_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportTfPort<'s>, VerboseError<'s>> {
    alt((
        method_prototype_parser.map(|a| ModportTfPort::Method(Box::new(a))),
        tf_identifier_parser.map(|a| ModportTfPort::Tf(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn import_export_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ImportExport<'s>, VerboseError<'s>> {
    alt((
        token(Token::Import).map(|a| ImportExport::Import(a)),
        token(Token::Export).map(|a| ImportExport::Export(a)),
    ))
    .parse_next(input)
}
