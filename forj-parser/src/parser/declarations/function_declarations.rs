// =======================================================================
// function_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.6

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};
use winnow::token::any;

pub fn function_data_type_or_implicit_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionDataTypeOrImplicit<'s>, VerboseError<'s>> {
    alt((
        terminated(
            data_type_or_void_parser,
            peek((
                opt_note(interface_identifier_or_class_scope_parser),
                function_identifier_parser,
            )),
        )
        .map(|a| FunctionDataTypeOrImplicit::DataType(Box::new(a))),
        terminated(
            implicit_data_type_parser,
            peek((
                opt_note(interface_identifier_or_class_scope_parser),
                function_identifier_parser,
            )),
        )
        .map(|a| FunctionDataTypeOrImplicit::Implicit(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn function_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Function),
        opt_dynamic_override_specifiers_parser,
        opt_note(lifetime_parser),
        function_body_declaration_parser,
    )
        .map(|(a, b, c, d)| FunctionDeclaration(a, b, c, d))
        .parse_next(input)
}

pub fn function_body_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionBodyDeclaration<'s>, VerboseError<'s>> {
    let _tf_parser = (
        function_data_type_or_implicit_parser,
        opt_note(interface_identifier_or_class_scope_parser),
        function_identifier_parser,
        token(Token::SColon),
        repeat_note(tf_item_declaration_parser),
        repeat_note(function_statement_or_null_parser),
        token(Token::Endfunction),
        opt_note((token(Token::Colon), function_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            FunctionBodyDeclaration::Tf(Box::new((a, b, c, d, e, f, g, h)))
        });
    let _block_parser = (
        function_data_type_or_implicit_parser,
        opt_note(interface_identifier_or_class_scope_parser),
        function_identifier_parser,
        token(Token::Paren),
        opt_note(tf_port_list_parser),
        token(Token::EParen),
        token(Token::SColon),
        repeat_note(block_item_declaration_parser),
        repeat_note(function_statement_or_null_parser),
        token(Token::Endfunction),
        opt_note((token(Token::Colon), function_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k)| {
            FunctionBodyDeclaration::Block(Box::new((
                a, b, c, d, e, f, g, h, i, j, k,
            )))
        });
    alt((_tf_parser, _block_parser)).parse_next(input)
}

pub fn function_prototype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionPrototype<'s>, VerboseError<'s>> {
    (
        token(Token::Function),
        opt_dynamic_override_specifiers_parser,
        data_type_or_void_parser,
        function_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(tf_port_list_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d, e)| FunctionPrototype(a, b, c, d, e))
        .parse_next(input)
}

pub fn dpi_import_export_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DpiImportExport<'s>, VerboseError<'s>> {
    let _function_import_parser = (
        token(Token::Import),
        dpi_spec_string_parser,
        opt_note(dpi_function_import_property_parser),
        opt_note((c_identifier_parser, token(Token::Eq))),
        dpi_function_proto_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            DpiImportExport::FunctionImport(Box::new((a, b, c, d, e, f)))
        });
    let _task_import_parser = (
        token(Token::Import),
        dpi_spec_string_parser,
        opt_note(dpi_task_import_property_parser),
        opt_note((c_identifier_parser, token(Token::Eq))),
        dpi_task_proto_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            DpiImportExport::TaskImport(Box::new((a, b, c, d, e, f)))
        });
    let _function_export_parser = (
        token(Token::Export),
        dpi_spec_string_parser,
        opt_note((c_identifier_parser, token(Token::Eq))),
        token(Token::Function),
        function_identifier_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            DpiImportExport::FunctionExport(Box::new((a, b, c, d, e, f)))
        });
    let _task_export_parser = (
        token(Token::Export),
        dpi_spec_string_parser,
        opt_note((c_identifier_parser, token(Token::Eq))),
        token(Token::Task),
        task_identifier_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            DpiImportExport::TaskExport(Box::new((a, b, c, d, e, f)))
        });
    alt((
        _function_import_parser,
        _task_import_parser,
        _function_export_parser,
        _task_export_parser,
    ))
    .parse_next(input)
}

pub fn dpi_spec_string_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DpiSpecString<'s>, VerboseError<'s>> {
    any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
        Token::StringLiteral("DPI-C") => {
            Some(DpiSpecString::DpiC(Metadata::new(s.1.clone(), vec![])))
        }
        Token::StringLiteral("DPI") => {
            Some(DpiSpecString::Dpi(Metadata::new(s.1.clone(), vec![])))
        }
        _ => None,
    })
    .parse_next(input)
}

pub fn dpi_function_import_property_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DpiFunctionImportProperty<'s>, VerboseError<'s>> {
    alt((
        token(Token::Context).map(|a| DpiFunctionImportProperty::Context(a)),
        token(Token::Pure).map(|a| DpiFunctionImportProperty::Pure(a)),
    ))
    .parse_next(input)
}

pub fn dpi_task_import_property_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DpiTaskImportProperty<'s>, VerboseError<'s>> {
    token(Token::Context)
        .map(|a| DpiTaskImportProperty(a))
        .parse_next(input)
}

pub fn dpi_function_proto_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DpiFunctionProto<'s>, VerboseError<'s>> {
    function_prototype_parser
        .map(|a| DpiFunctionProto(a))
        .parse_next(input)
}

pub fn dpi_task_proto_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DpiTaskProto<'s>, VerboseError<'s>> {
    task_prototype_parser
        .map(|a| DpiTaskProto(a))
        .parse_next(input)
}
