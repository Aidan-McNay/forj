// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
// Parsing for 1800-2023 A.1.2

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn source_text_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SourceText<'s>, VerboseError<'s>> {
    let non_trivia = non_trivia_parser(input)?;
    let timeunits_declaration =
        opt_note(timeunits_declaration_parser).parse_next(input)?;
    let mut descriptions: Vec<Description<'s>> = vec![];
    loop {
        if input.is_empty() {
            break;
        }
        let new_description = description_parser(input)?;
        descriptions.push(new_description);
    }
    Ok(SourceText(non_trivia, timeunits_declaration, descriptions))
}

pub fn description_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Description<'s>, VerboseError<'s>> {
    let description_package_item_parser =
        (attribute_instance_vec_parser, package_item_parser)
            .map(|(a, b)| DescriptionPackageItem(a, b));
    let description_bind_directive_parser =
        (attribute_instance_vec_parser, bind_directive_parser)
            .map(|(a, b)| DescriptionBindDirective(a, b));
    alt((
        module_declaration_parser
            .map(|a| Description::ModuleDeclaration(Box::new(a))),
        udp_declaration_parser
            .map(|a| Description::UdpDeclaration(Box::new(a))),
        interface_declaration_parser
            .map(|a| Description::InterfaceDeclaration(Box::new(a))),
        program_declaration_parser
            .map(|a| Description::ProgramDeclaration(Box::new(a))),
        package_declaration_parser
            .map(|a| Description::PackageDeclaration(Box::new(a))),
        description_package_item_parser
            .map(|a| Description::DescriptionPackageItem(Box::new(a))),
        description_bind_directive_parser
            .map(|a| Description::DescriptionBindDirective(Box::new(a))),
        config_declaration_parser
            .map(|a| Description::ConfigDeclaration(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn module_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleDeclaration<'s>, VerboseError<'s>> {
    alt((
        module_declaration_nonansi_parser
            .map(|a| ModuleDeclaration::Nonansi(Box::new(a))),
        module_declaration_ansi_parser
            .map(|a| ModuleDeclaration::Ansi(Box::new(a))),
        module_declaration_wildcard_parser
            .map(|a| ModuleDeclaration::Wildcard(Box::new(a))),
        module_declaration_extern_nonansi_parser
            .map(|a| ModuleDeclaration::ExternNonansi(Box::new(a))),
        module_declaration_extern_ansi_parser
            .map(|a| ModuleDeclaration::ExternAnsi(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn module_declaration_nonansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleDeclarationNonansi<'s>, VerboseError<'s>> {
    (
        module_nonansi_header_parser,
        opt_note(timeunits_declaration_parser),
        repeat_note(module_item_parser),
        token(Token::Endmodule),
        opt_note((token(Token::Colon), module_identifier_parser)),
    )
        .map(|(a, b, c, d, e)| ModuleDeclarationNonansi(a, b, c, d, e))
        .parse_next(input)
}

pub fn module_declaration_ansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleDeclarationAnsi<'s>, VerboseError<'s>> {
    (
        module_ansi_header_parser,
        opt_note(timeunits_declaration_parser),
        repeat_note(non_port_module_item_parser),
        token(Token::Endmodule),
        opt_note((token(Token::Colon), module_identifier_parser)),
    )
        .map(|(a, b, c, d, e)| ModuleDeclarationAnsi(a, b, c, d, e))
        .parse_next(input)
}

pub fn module_nonansi_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleNonansiHeader<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        module_keyword_parser,
        opt_note(lifetime_parser),
        module_identifier_parser,
        repeat_note(package_import_declaration_parser),
        opt_note(parameter_port_list_parser),
        list_of_ports_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            ModuleNonansiHeader(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn module_ansi_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleAnsiHeader<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        module_keyword_parser,
        opt_note(lifetime_parser),
        module_identifier_parser,
        repeat_note(package_import_declaration_parser),
        opt_note(parameter_port_list_parser),
        opt_note(list_of_port_declarations_parser),
        token(Token::SColon),
    )
        .verify_map(|(a, b, c, d, e, f, g, h)| {
            if (e.len() > 0) & f.is_none() & g.is_none() {
                None // BNF clarification 1
            } else {
                Some(ModuleAnsiHeader(a, b, c, d, e, f, g, h))
            }
        })
        .parse_next(input)
}

pub fn module_declaration_wildcard_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleDeclarationWildcard<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        module_keyword_parser,
        opt_note(lifetime_parser),
        module_identifier_parser,
        token(Token::Paren),
        token(Token::Period),
        token(Token::Star),
        token(Token::EParen),
        token(Token::SColon),
        opt_note(timeunits_declaration_parser),
        repeat_note(module_item_parser),
        token(Token::Endmodule),
        opt_note((token(Token::Colon), module_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m)| {
            ModuleDeclarationWildcard(a, b, c, d, e, f, g, h, i, j, k, l, m)
        })
        .parse_next(input)
}

pub fn module_declaration_extern_nonansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleDeclarationExternNonansi<'s>, VerboseError<'s>> {
    (token(Token::Extern), module_nonansi_header_parser)
        .map(|(a, b)| ModuleDeclarationExternNonansi(a, b))
        .parse_next(input)
}

pub fn module_declaration_extern_ansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleDeclarationExternAnsi<'s>, VerboseError<'s>> {
    (token(Token::Extern), module_ansi_header_parser)
        .map(|(a, b)| ModuleDeclarationExternAnsi(a, b))
        .parse_next(input)
}

pub fn module_keyword_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleKeyword<'s>, VerboseError<'s>> {
    alt((
        token(Token::Module).map(|a| ModuleKeyword::Module(a)),
        token(Token::Macromodule).map(|a| ModuleKeyword::Macromodule(a)),
    ))
    .parse_next(input)
}

pub fn interface_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceDeclaration<'s>, VerboseError<'s>> {
    alt((
        interface_declaration_nonansi_parser
            .map(|a| InterfaceDeclaration::Nonansi(Box::new(a))),
        interface_declaration_ansi_parser
            .map(|a| InterfaceDeclaration::Ansi(Box::new(a))),
        interface_declaration_wildcard_parser
            .map(|a| InterfaceDeclaration::Wildcard(Box::new(a))),
        interface_declaration_extern_nonansi_parser
            .map(|a| InterfaceDeclaration::ExternNonansi(Box::new(a))),
        interface_declaration_extern_ansi_parser
            .map(|a| InterfaceDeclaration::ExternAnsi(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn interface_declaration_nonansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceDeclarationNonansi<'s>, VerboseError<'s>> {
    (
        interface_nonansi_header_parser,
        opt_note(timeunits_declaration_parser),
        repeat_note(interface_item_parser),
        token(Token::Endinterface),
        opt_note((token(Token::Colon), interface_identifier_parser)),
    )
        .map(|(a, b, c, d, e)| InterfaceDeclarationNonansi(a, b, c, d, e))
        .parse_next(input)
}

pub fn interface_declaration_ansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceDeclarationAnsi<'s>, VerboseError<'s>> {
    (
        interface_ansi_header_parser,
        opt_note(timeunits_declaration_parser),
        repeat_note(non_port_interface_item_parser),
        token(Token::Endinterface),
        opt_note((token(Token::Colon), interface_identifier_parser)),
    )
        .map(|(a, b, c, d, e)| InterfaceDeclarationAnsi(a, b, c, d, e))
        .parse_next(input)
}

pub fn interface_declaration_wildcard_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceDeclarationWildcard<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Interface),
        interface_identifier_parser,
        token(Token::Paren),
        token(Token::Period),
        token(Token::Star),
        token(Token::EParen),
        token(Token::SColon),
        opt_note(timeunits_declaration_parser),
        repeat_note(interface_item_parser),
        token(Token::Endinterface),
        opt_note((token(Token::Colon), interface_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            InterfaceDeclarationWildcard(a, b, c, d, e, f, g, h, i, j, k, l)
        })
        .parse_next(input)
}

pub fn interface_declaration_extern_nonansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceDeclarationExternNonansi<'s>, VerboseError<'s>> {
    (token(Token::Extern), interface_nonansi_header_parser)
        .map(|(a, b)| InterfaceDeclarationExternNonansi(a, b))
        .parse_next(input)
}

pub fn interface_declaration_extern_ansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceDeclarationExternAnsi<'s>, VerboseError<'s>> {
    (token(Token::Extern), interface_ansi_header_parser)
        .map(|(a, b)| InterfaceDeclarationExternAnsi(a, b))
        .parse_next(input)
}

pub fn interface_nonansi_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceNonansiHeader<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Interface),
        opt_note(lifetime_parser),
        interface_identifier_parser,
        repeat_note(package_import_declaration_parser),
        opt_note(parameter_port_list_parser),
        list_of_ports_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            InterfaceNonansiHeader(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn interface_ansi_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceAnsiHeader<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Interface),
        opt_note(lifetime_parser),
        interface_identifier_parser,
        repeat_note(package_import_declaration_parser),
        opt_note(parameter_port_list_parser),
        opt_note(list_of_port_declarations_parser),
        token(Token::SColon),
    )
        .verify_map(|(a, b, c, d, e, f, g, h)| {
            if (e.len() > 0) & f.is_none() & g.is_none() {
                None // BNF clarification 1
            } else {
                Some(InterfaceAnsiHeader(a, b, c, d, e, f, g, h))
            }
        })
        .parse_next(input)
}

pub fn program_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramDeclaration<'s>, VerboseError<'s>> {
    alt((
        program_declaration_nonansi_parser
            .map(|a| ProgramDeclaration::Nonansi(Box::new(a))),
        program_declaration_ansi_parser
            .map(|a| ProgramDeclaration::Ansi(Box::new(a))),
        program_declaration_wildcard_parser
            .map(|a| ProgramDeclaration::Wildcard(Box::new(a))),
        program_declaration_extern_nonansi_parser
            .map(|a| ProgramDeclaration::ExternNonansi(Box::new(a))),
        program_declaration_extern_ansi_parser
            .map(|a| ProgramDeclaration::ExternAnsi(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn program_declaration_nonansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramDeclarationNonansi<'s>, VerboseError<'s>> {
    (
        program_nonansi_header_parser,
        opt_note(timeunits_declaration_parser),
        repeat_note(program_item_parser),
        token(Token::Endprogram),
        opt_note((token(Token::Colon), program_identifier_parser)),
    )
        .map(|(a, b, c, d, e)| ProgramDeclarationNonansi(a, b, c, d, e))
        .parse_next(input)
}

pub fn program_declaration_ansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramDeclarationAnsi<'s>, VerboseError<'s>> {
    (
        program_ansi_header_parser,
        opt_note(timeunits_declaration_parser),
        repeat_note(non_port_program_item_parser),
        token(Token::Endprogram),
        opt_note((token(Token::Colon), program_identifier_parser)),
    )
        .map(|(a, b, c, d, e)| ProgramDeclarationAnsi(a, b, c, d, e))
        .parse_next(input)
}

pub fn program_declaration_wildcard_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramDeclarationWildcard<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Program),
        program_identifier_parser,
        token(Token::Paren),
        token(Token::Period),
        token(Token::Star),
        token(Token::EParen),
        token(Token::SColon),
        opt_note(timeunits_declaration_parser),
        repeat_note(program_item_parser),
        token(Token::Endprogram),
        opt_note((token(Token::Colon), program_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            ProgramDeclarationWildcard(a, b, c, d, e, f, g, h, i, j, k, l)
        })
        .parse_next(input)
}

pub fn program_declaration_extern_nonansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramDeclarationExternNonansi<'s>, VerboseError<'s>> {
    (token(Token::Extern), program_nonansi_header_parser)
        .map(|(a, b)| ProgramDeclarationExternNonansi(a, b))
        .parse_next(input)
}

pub fn program_declaration_extern_ansi_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramDeclarationExternAnsi<'s>, VerboseError<'s>> {
    (token(Token::Extern), program_ansi_header_parser)
        .map(|(a, b)| ProgramDeclarationExternAnsi(a, b))
        .parse_next(input)
}

pub fn program_nonansi_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramNonansiHeader<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Program),
        opt_note(lifetime_parser),
        program_identifier_parser,
        repeat_note(package_import_declaration_parser),
        opt_note(parameter_port_list_parser),
        list_of_ports_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            ProgramNonansiHeader(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn program_ansi_header_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramAnsiHeader<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        token(Token::Program),
        opt_note(lifetime_parser),
        program_identifier_parser,
        repeat_note(package_import_declaration_parser),
        opt_note(parameter_port_list_parser),
        opt_note(list_of_port_declarations_parser),
        token(Token::SColon),
    )
        .verify_map(|(a, b, c, d, e, f, g, h)| {
            if (e.len() > 0) & f.is_none() & g.is_none() {
                None // BNF clarification 1
            } else {
                Some(ProgramAnsiHeader(a, b, c, d, e, f, g, h))
            }
        })
        .parse_next(input)
}

pub fn checker_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerDeclaration<'s>, VerboseError<'s>> {
    let checker_declaration_port_list_parser = (
        token(Token::Paren),
        checker_port_list_parser,
        token(Token::EParen),
    );
    let checker_declaration_item_parser = repeat_note((
        attribute_instance_vec_parser,
        checker_or_generate_item_parser,
    ));
    (
        token(Token::Checker),
        checker_identifier_parser,
        opt_note(checker_declaration_port_list_parser),
        token(Token::SColon),
        checker_declaration_item_parser,
        token(Token::Endchecker),
        opt_note((token(Token::Colon), checker_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g)| CheckerDeclaration(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn class_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassDeclaration<'s>, VerboseError<'s>> {
    let class_declaration_extension_parser = (
        token(Token::Extends),
        class_type_parser,
        opt_note((
            token(Token::Paren),
            alt((
                list_of_arguments_parser.map(|a| {
                    ClassDeclarationExtensionArguments::ListOfArguments(
                        Box::new(a),
                    )
                }),
                token(Token::Default).map(|metadata| {
                    ClassDeclarationExtensionArguments::Default(metadata)
                }),
            )),
            token(Token::EParen),
        )),
    );
    let class_declaration_implementation_parser = (
        token(Token::Implements),
        interface_class_type_parser,
        repeat_note((token(Token::Comma), interface_class_type_parser)),
    );
    (
        opt_note(token(Token::Virtual)),
        token(Token::Class),
        opt_note(final_specifier_parser),
        class_identifier_parser,
        opt_note(parameter_port_list_parser),
        opt_note(class_declaration_extension_parser),
        opt_note(class_declaration_implementation_parser),
        token(Token::SColon),
        repeat_note(class_item_parser),
        token(Token::Endclass),
        opt_note((token(Token::Colon), class_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k)| {
            ClassDeclaration(a, b, c, d, e, f, g, h, i, j, k)
        })
        .parse_next(input)
}

pub fn interface_class_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceClassDeclaration<'s>, VerboseError<'s>> {
    let interface_class_declaration_extension_parser = (
        token(Token::Extends),
        interface_class_type_parser,
        repeat_note((token(Token::Comma), interface_class_type_parser)),
    );
    (
        token(Token::Interface),
        token(Token::Class),
        class_identifier_parser,
        opt_note(parameter_port_list_parser),
        opt_note(interface_class_declaration_extension_parser),
        token(Token::SColon),
        repeat_note(interface_class_item_parser),
        token(Token::Endclass),
        opt_note((token(Token::Colon), class_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i)| {
            InterfaceClassDeclaration(a, b, c, d, e, f, g, h, i)
        })
        .parse_next(input)
}

pub fn package_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageDeclaration<'s>, VerboseError<'s>> {
    let attribute_package_items_parser =
        repeat_note((attribute_instance_vec_parser, package_item_parser));
    (
        attribute_instance_vec_parser,
        token(Token::Package),
        opt_note(lifetime_parser),
        package_identifier_parser,
        token(Token::SColon),
        opt_note(timeunits_declaration_parser),
        attribute_package_items_parser,
        token(Token::Endpackage),
        opt_note((token(Token::Colon), package_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i)| {
            PackageDeclaration(a, b, c, d, e, f, g, h, i)
        })
        .parse_next(input)
}

pub fn timeunits_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimeunitsDeclaration<'s>, VerboseError<'s>> {
    let timeunit_parser = (
        token(Token::Timeunit),
        time_literal_parser,
        opt_note((token(Token::Slash), time_literal_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| TimeunitsDeclaration::Timeunit(a, b, c, d));
    let timeprecision_parser = (
        token(Token::Timeprecision),
        time_literal_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| TimeunitsDeclaration::Timeprecision(a, b, c));
    let timeunitprecision_parser = (
        token(Token::Timeunit),
        time_literal_parser,
        token(Token::SColon),
        token(Token::Timeprecision),
        time_literal_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            TimeunitsDeclaration::Timeunitprecision(a, b, c, d, e, f)
        });
    let timeprecisionunit_parser = (
        token(Token::Timeprecision),
        time_literal_parser,
        token(Token::SColon),
        token(Token::Timeunit),
        time_literal_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            TimeunitsDeclaration::Timeprecisionunit(a, b, c, d, e, f)
        });
    alt((
        timeunit_parser,
        timeprecision_parser,
        timeunitprecision_parser,
        timeprecisionunit_parser,
    ))
    .parse_next(input)
}
