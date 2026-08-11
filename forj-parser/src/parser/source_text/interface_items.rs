// =======================================================================
// interface_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.6

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn interface_or_generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceOrGenerateItem<'s>, VerboseError<'s>> {
    alt((
        (attribute_instance_vec_parser, module_common_item_parser).map(
            |(a, b)| InterfaceOrGenerateItem::ModuleCommon(Box::new((a, b))),
        ),
        (attribute_instance_vec_parser, extern_tf_declaration_parser)
            .map(|(a, b)| InterfaceOrGenerateItem::ExternTf(Box::new((a, b)))),
    ))
    .parse_next(input)
}

pub fn extern_tf_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ExternTfDeclaration<'s>, VerboseError<'s>> {
    let _method_parser = (
        token(Token::Extern),
        method_prototype_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| ExternTfDeclaration::Method(Box::new((a, b, c))));
    let _task_parser = (
        token(Token::Extern),
        token(Token::Forkjoin),
        task_prototype_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| ExternTfDeclaration::Task(Box::new((a, b, c, d))));
    alt((_task_parser, _method_parser)).parse_next(input)
}

pub fn interface_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceItem<'s>, VerboseError<'s>> {
    alt((
        (port_declaration_parser, token(Token::SColon))
            .map(|(a, b)| InterfaceItem::Port(Box::new((a, b)))),
        non_port_interface_item_parser
            .map(|a| InterfaceItem::NonPort(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn non_port_interface_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonPortInterfaceItem<'s>, VerboseError<'s>> {
    alt((
        program_declaration_parser
            .map(|a| NonPortInterfaceItem::Program(Box::new(a))),
        modport_declaration_parser
            .map(|a| NonPortInterfaceItem::Modport(Box::new(a))),
        interface_declaration_parser
            .map(|a| NonPortInterfaceItem::Interface(Box::new(a))),
        timeunits_declaration_parser
            .map(|a| NonPortInterfaceItem::Timeunits(Box::new(a))),
        generate_region_parser
            .map(|a| NonPortInterfaceItem::Generate(Box::new(a))),
        interface_or_generate_item_parser
            .map(|a| NonPortInterfaceItem::InterfaceOrGenerate(Box::new(a))),
    ))
    .parse_next(input)
}
