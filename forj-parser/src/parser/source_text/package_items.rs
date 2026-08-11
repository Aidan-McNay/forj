// =======================================================================
// package_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.11

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn package_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageItem<'s>, VerboseError<'s>> {
    alt((
        package_or_generate_item_declaration_parser
            .map(|a| PackageItem::PackageOrGenerate(Box::new(a))),
        anonymous_program_parser
            .map(|a| PackageItem::AnonymousProgram(Box::new(a))),
        package_export_declaration_parser
            .map(|a| PackageItem::PackageExport(Box::new(a))),
        timeunits_declaration_parser
            .map(|a| PackageItem::Timeunits(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn package_or_generate_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageOrGenerateItemDeclaration<'s>, VerboseError<'s>> {
    alt((
        net_declaration_parser
            .map(|a| PackageOrGenerateItemDeclaration::Net(Box::new(a))),
        data_declaration_parser
            .map(|a| PackageOrGenerateItemDeclaration::Data(Box::new(a))),
        task_declaration_parser
            .map(|a| PackageOrGenerateItemDeclaration::Task(Box::new(a))),
        function_declaration_parser
            .map(|a| PackageOrGenerateItemDeclaration::Function(Box::new(a))),
        checker_declaration_parser
            .map(|a| PackageOrGenerateItemDeclaration::Checker(Box::new(a))),
        dpi_import_export_parser.map(|a| {
            PackageOrGenerateItemDeclaration::DpiImportExport(Box::new(a))
        }),
        extern_constraint_declaration_parser.map(|a| {
            PackageOrGenerateItemDeclaration::ExternConstraint(Box::new(a))
        }),
        class_declaration_parser
            .map(|a| PackageOrGenerateItemDeclaration::Class(Box::new(a))),
        interface_class_declaration_parser.map(|a| {
            PackageOrGenerateItemDeclaration::InterfaceClass(Box::new(a))
        }),
        class_constructor_declaration_parser.map(|a| {
            PackageOrGenerateItemDeclaration::ClassConstructor(Box::new(a))
        }),
        (local_parameter_declaration_parser, token(Token::SColon)).map(
            |(a, b)| {
                PackageOrGenerateItemDeclaration::LocalParameter(Box::new((
                    a, b,
                )))
            },
        ),
        (parameter_declaration_parser, token(Token::SColon)).map(|(a, b)| {
            PackageOrGenerateItemDeclaration::Parameter(Box::new((a, b)))
        }),
        covergroup_declaration_parser
            .map(|a| PackageOrGenerateItemDeclaration::Covergroup(Box::new(a))),
        assertion_item_declaration_parser.map(|a| {
            PackageOrGenerateItemDeclaration::AssertionItem(Box::new(a))
        }),
        token(Token::SColon)
            .map(|a| PackageOrGenerateItemDeclaration::Null(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn anonymous_program_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AnonymousProgram<'s>, VerboseError<'s>> {
    (
        token(Token::Program),
        token(Token::SColon),
        repeat_note(anonymous_program_item_parser),
        token(Token::Endprogram),
    )
        .map(|(a, b, c, d)| AnonymousProgram(a, b, c, d))
        .parse_next(input)
}

pub fn anonymous_program_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AnonymousProgramItem<'s>, VerboseError<'s>> {
    alt((
        task_declaration_parser
            .map(|a| AnonymousProgramItem::Task(Box::new(a))),
        function_declaration_parser
            .map(|a| AnonymousProgramItem::Function(Box::new(a))),
        class_declaration_parser
            .map(|a| AnonymousProgramItem::Class(Box::new(a))),
        interface_class_declaration_parser
            .map(|a| AnonymousProgramItem::InterfaceClass(Box::new(a))),
        covergroup_declaration_parser
            .map(|a| AnonymousProgramItem::Covergroup(Box::new(a))),
        class_constructor_declaration_parser
            .map(|a| AnonymousProgramItem::ClassConstructor(Box::new(a))),
        token(Token::SColon).map(|a| AnonymousProgramItem::Null(Box::new(a))),
    ))
    .parse_next(input)
}
