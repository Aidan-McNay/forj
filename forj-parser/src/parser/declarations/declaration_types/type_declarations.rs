// =======================================================================
// type_declarations.rs
// =======================================================================
// Parsing 1800-2023 A.2.1.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};

pub fn data_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataDeclaration<'s>, VerboseError<'s>> {
    let _variable_data_declaration_parser = (
        opt_note(token(Token::Const)),
        opt_note(token(Token::Var)),
        opt_note(lifetime_parser),
        data_type_or_implicit_parser_data_declaration,
        list_of_variable_decl_assignments_parser,
        token(Token::SColon),
    )
        .verify_map(|(a, b, c, d, e, f)| {
            if matches!(d, DataTypeOrImplicit::ImplicitDataType(_))
                & b.is_none()
            {
                None // BNF clarification 14
            } else {
                Some(DataDeclaration::Variable(Box::new((a, b, c, d, e, f))))
            }
        });
    alt((
        _variable_data_declaration_parser,
        type_declaration_parser.map(|a| DataDeclaration::Type(Box::new(a))),
        package_import_declaration_parser
            .map(|a| DataDeclaration::PackageImport(Box::new(a))),
        nettype_declaration_parser
            .map(|a| DataDeclaration::Nettype(Box::new(a))),
    ))
    .parse_next(input)
}

fn data_type_or_implicit_parser_data_declaration<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataTypeOrImplicit<'s>, VerboseError<'s>> {
    alt((
        terminated(data_type_parser, peek(variable_decl_assignment_parser))
            .map(|a| DataTypeOrImplicit::DataType(a)),
        terminated(
            implicit_data_type_parser,
            peek(variable_decl_assignment_parser),
        )
        .map(|a| DataTypeOrImplicit::ImplicitDataType(a)),
    ))
    .parse_next(input)
}

pub fn package_import_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageImportDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Import),
        package_import_item_parser,
        repeat_note((token(Token::Comma), package_import_item_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| PackageImportDeclaration(a, b, c, d))
        .parse_next(input)
}

pub fn package_export_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageExportDeclaration<'s>, VerboseError<'s>> {
    let _import_parser = (
        token(Token::Import),
        package_import_item_parser,
        repeat_note((token(Token::Comma), package_import_item_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| {
            PackageExportDeclaration::Import(Box::new((a, b, c, d)))
        });
    alt((
        _import_parser,
        (
            token(Token::Export),
            token(Token::Star),
            token(Token::ColonColon),
            token(Token::Star),
            token(Token::SColon),
        )
            .map(|(a, b, c, d, e)| {
                PackageExportDeclaration::Wildcard(Box::new((a, b, c, d, e)))
            }),
    ))
    .parse_next(input)
}

pub fn package_import_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageImportItem<'s>, VerboseError<'s>> {
    alt((
        (
            package_identifier_parser,
            token(Token::ColonColon),
            identifier_parser,
        )
            .map(|(a, b, c)| {
                PackageImportItem::Identifier(Box::new((a, b, c)))
            }),
        (
            package_identifier_parser,
            token(Token::ColonColon),
            token(Token::Star),
        )
            .map(|(a, b, c)| PackageImportItem::Wildcard(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn genvar_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenvarDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Genvar),
        list_of_genvar_identifiers_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| GenvarDeclaration(a, b, c))
        .parse_next(input)
}

pub fn net_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetDeclaration<'s>, VerboseError<'s>> {
    let _drive_or_charge_strength_parser = alt((
        drive_strength_parser.map(|a| DriveOrChargeStrength::DriveStrength(a)),
        charge_strength_parser
            .map(|a| DriveOrChargeStrength::ChargeStrength(a)),
    ));
    let _vectored_or_scalared_parser = alt((
        token(Token::Vectored).map(|a| VectoredOrScalared::Vectored(a)),
        token(Token::Scalared).map(|a| VectoredOrScalared::Scalared(a)),
    ));
    let _net_type_parser = (
        net_type_parser,
        opt_note(_drive_or_charge_strength_parser),
        opt_note(_vectored_or_scalared_parser),
        data_type_or_implicit_parser_net_declaration,
        opt_note(delay3_parser),
        list_of_net_decl_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g)| {
            NetDeclaration::NetType(Box::new((a, b, c, d, e, f, g)))
        });
    let _nettype_identifier_parser = (
        nettype_identifier_parser,
        opt_note(delay_control_parser),
        list_of_net_decl_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| {
            NetDeclaration::NettypeIdentifier(Box::new((a, b, c, d)))
        });
    let _interconnect_parser = (
        token(Token::Interconnect),
        implicit_data_type_parser,
        opt_note((token(Token::Pound), delay_value_parser)),
        net_identifier_parser,
        repeat_note(unpacked_dimension_parser),
        opt_note((
            token(Token::Comma),
            net_identifier_parser,
            repeat_note(unpacked_dimension_parser),
        )),
    )
        .map(|(a, b, c, d, e, f)| {
            NetDeclaration::Interconnect(Box::new((a, b, c, d, e, f)))
        });
    alt((
        _net_type_parser,
        _nettype_identifier_parser,
        _interconnect_parser,
    ))
    .parse_next(input)
}

fn data_type_or_implicit_parser_net_declaration<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataTypeOrImplicit<'s>, VerboseError<'s>> {
    alt((
        terminated(
            data_type_parser,
            peek((opt_note(delay3_parser), net_decl_assignment_parser)),
        )
        .map(|a| DataTypeOrImplicit::DataType(a)),
        terminated(
            implicit_data_type_parser,
            peek((opt_note(delay3_parser), net_decl_assignment_parser)),
        )
        .map(|a| DataTypeOrImplicit::ImplicitDataType(a)),
    ))
    .parse_next(input)
}

pub fn type_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TypeDeclaration<'s>, VerboseError<'s>> {
    let _data_type_or_incomplete_class_scoped_parser = (
        token(Token::Typedef),
        data_type_or_incomplete_class_scoped_type_parser,
        type_identifier_parser,
        repeat_note(variable_dimension_parser),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            TypeDeclaration::DataTypeOrIncompleteClassScoped(Box::new((
                a, b, c, d, e,
            )))
        });
    let _interface_port_parser = (
        token(Token::Typedef),
        interface_port_identifier_parser,
        constant_bit_select_parser,
        token(Token::Period),
        type_identifier_parser,
        type_identifier_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g)| {
            TypeDeclaration::InterfacePort(Box::new((a, b, c, d, e, f, g)))
        });
    let _forward_type_parser = (
        token(Token::Typedef),
        opt_note(forward_type_parser),
        type_identifier_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| {
            TypeDeclaration::ForwardType(Box::new((a, b, c, d)))
        });
    alt((
        _data_type_or_incomplete_class_scoped_parser,
        _interface_port_parser,
        _forward_type_parser,
    ))
    .parse_next(input)
}

pub fn forward_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ForwardType<'s>, VerboseError<'s>> {
    alt((
        token(Token::Enum).map(|a| ForwardType::Enum(a)),
        token(Token::Struct).map(|a| ForwardType::Struct(a)),
        token(Token::Union).map(|a| ForwardType::Union(a)),
        token(Token::Class).map(|a| ForwardType::Class(a)),
        (token(Token::Interface), token(Token::Class))
            .map(|(a, b)| ForwardType::InterfaceClass(a, b)),
    ))
    .parse_next(input)
}

pub fn nettype_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NettypeDeclaration<'s>, VerboseError<'s>> {
    let _with_scope_parser = (
        token(Token::Nettype),
        data_type_parser,
        nettype_identifier_parser,
        opt_note((
            token(Token::With),
            opt_note(package_or_class_scope_parser),
            tf_identifier_parser,
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            NettypeDeclaration::WithScope(Box::new((a, b, c, d, e)))
        });
    let _scoped_parser = (
        token(Token::Nettype),
        opt_note(package_or_class_scope_parser),
        nettype_identifier_parser,
        nettype_identifier_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            NettypeDeclaration::Scoped(Box::new((a, b, c, d, e)))
        });
    alt((_with_scope_parser, _scoped_parser)).parse_next(input)
}

pub fn lifetime_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Lifetime<'s>, VerboseError<'s>> {
    alt((
        token(Token::Static).map(|a| Lifetime::Static(a)),
        token(Token::Automatic).map(|a| Lifetime::Automatic(a)),
    ))
    .parse_next(input)
}
