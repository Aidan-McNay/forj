// =======================================================================
// module_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.4

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::token::any;

pub fn system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SeveritySystemTask<'s>, VerboseError<'s>> {
    alt((
        fatal_system_severity_task_parser
            .map(|a| SeveritySystemTask::Fatal(Box::new(a))),
        error_system_severity_task_parser
            .map(|a| SeveritySystemTask::Error(Box::new(a))),
        warning_system_severity_task_parser
            .map(|a| SeveritySystemTask::Warning(Box::new(a))),
        info_system_severity_task_parser
            .map(|a| SeveritySystemTask::Info(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn fatal_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FatalSeveritySystemTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarFatal),
        opt_note((
            token(Token::Paren),
            finish_number_parser,
            opt_note((token(Token::Comma), list_of_arguments_parser)),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| FatalSeveritySystemTask(a, b, c))
        .parse_next(input)
}

pub fn error_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ErrorSeveritySystemTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarError),
        opt_note((
            token(Token::Paren),
            opt_note(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| ErrorSeveritySystemTask(a, b, c))
        .parse_next(input)
}

pub fn warning_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<WarningSeveritySystemTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarWarning),
        opt_note((
            token(Token::Paren),
            opt_note(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| WarningSeveritySystemTask(a, b, c))
        .parse_next(input)
}

pub fn info_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InfoSeveritySystemTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarInfo),
        opt_note((
            token(Token::Paren),
            opt_note(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| InfoSeveritySystemTask(a, b, c))
        .parse_next(input)
}

pub fn finish_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FinishNumber<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::UnsignedNumber("0") => {
                Some(FinishNumber::Zero(Metadata::new(s.1.clone(), vec![])))
            }
            Token::UnsignedNumber("1") => {
                Some(FinishNumber::One(Metadata::new(s.1.clone(), vec![])))
            }
            Token::UnsignedNumber("2") => {
                Some(FinishNumber::Two(Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|(finish_number, non_trivia)| match finish_number {
            FinishNumber::Zero(metadata) => {
                FinishNumber::Zero(replace_non_trivia(metadata, non_trivia))
            }
            FinishNumber::One(metadata) => {
                FinishNumber::One(replace_non_trivia(metadata, non_trivia))
            }
            FinishNumber::Two(metadata) => {
                FinishNumber::Two(replace_non_trivia(metadata, non_trivia))
            }
        })
        .parse_next(input)
}

pub fn elaboration_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ElaborationSeveritySystemTask<'s>, VerboseError<'s>> {
    system_severity_task_parser
        .map(|a| ElaborationSeveritySystemTask(a))
        .parse_next(input)
}

pub fn module_common_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleCommonItem<'s>, VerboseError<'s>> {
    alt((
        continuous_assign_parser.map(|a| ModuleCommonItem::Assign(Box::new(a))),
        always_construct_parser.map(|a| ModuleCommonItem::Always(Box::new(a))),
        initial_construct_parser
            .map(|a| ModuleCommonItem::Initial(Box::new(a))),
        final_construct_parser.map(|a| ModuleCommonItem::Final(Box::new(a))),
        interface_instantiation_parser
            .map(|a| ModuleCommonItem::Interface(Box::new(a))),
        program_instantiation_parser
            .map(|a| ModuleCommonItem::Program(Box::new(a))),
        assertion_item_parser.map(|a| ModuleCommonItem::Assertion(Box::new(a))),
        bind_directive_parser.map(|a| ModuleCommonItem::Bind(Box::new(a))),
        net_alias_parser.map(|a| ModuleCommonItem::Alias(Box::new(a))),
        loop_generate_construct_parser
            .map(|a| ModuleCommonItem::LoopGenerate(Box::new(a))),
        conditional_generate_construct_parser.map(|a| {
            ModuleCommonItem::ConditionalGenerateConstruct(Box::new(a))
        }),
        elaboration_system_severity_task_parser
            .map(|a| ModuleCommonItem::SystemSeverity(Box::new(a))),
        module_or_generate_item_declaration_parser.map(|a| {
            ModuleCommonItem::ModuleOrGenerateDeclaration(Box::new(a))
        }),
    ))
    .parse_next(input)
}

pub fn module_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleItem<'s>, VerboseError<'s>> {
    alt((
        (port_declaration_parser, token(Token::SColon))
            .map(|(a, b)| ModuleItem::Port(Box::new((a, b)))),
        non_port_module_item_parser.map(|a| ModuleItem::NonPort(Box::new(a))),
    ))
    .parse_next(input)
}

enum ModuleOrGenerateItemBody<'a> {
    ParameterOverride(ParameterOverride<'a>),
    Gate(GateInstantiation<'a>),
    Udp(UdpInstantiation<'a>),
    Module(ModuleInstantiation<'a>),
    ModuleCommon(ModuleCommonItem<'a>),
}

pub fn module_or_generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleOrGenerateItem<'s>, VerboseError<'s>> {
    let _body_parser = alt((
        parameter_override_parser
            .map(|a| ModuleOrGenerateItemBody::ParameterOverride(a)),
        gate_instantiation_parser.map(|a| ModuleOrGenerateItemBody::Gate(a)),
        udp_instantiation_parser.map(|a| ModuleOrGenerateItemBody::Udp(a)),
        module_instantiation_parser
            .map(|a| ModuleOrGenerateItemBody::Module(a)),
        module_common_item_parser
            .map(|a| ModuleOrGenerateItemBody::ModuleCommon(a)),
    ));
    (attribute_instance_vec_parser, _body_parser)
        .map(|(a, b)| match b {
            ModuleOrGenerateItemBody::ParameterOverride(c) => {
                ModuleOrGenerateItem::ParameterOverride(Box::new((a, c)))
            }
            ModuleOrGenerateItemBody::Gate(c) => {
                ModuleOrGenerateItem::Gate(Box::new((a, c)))
            }
            ModuleOrGenerateItemBody::Udp(c) => {
                ModuleOrGenerateItem::Udp(Box::new((a, c)))
            }
            ModuleOrGenerateItemBody::Module(c) => {
                ModuleOrGenerateItem::Module(Box::new((a, c)))
            }
            ModuleOrGenerateItemBody::ModuleCommon(c) => {
                ModuleOrGenerateItem::ModuleCommon(Box::new((a, c)))
            }
        })
        .parse_next(input)
}

pub fn module_or_generate_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleOrGenerateItemDeclaration<'s>, VerboseError<'s>> {
    alt((
        package_or_generate_item_declaration_parser.map(|a| {
            ModuleOrGenerateItemDeclaration::PackageOrGenerate(Box::new(a))
        }),
        genvar_declaration_parser
            .map(|a| ModuleOrGenerateItemDeclaration::Genvar(Box::new(a))),
        clocking_declaration_parser
            .map(|a| ModuleOrGenerateItemDeclaration::Clocking(Box::new(a))),
        (
            token(Token::Default),
            token(Token::Clocking),
            clocking_identifier_parser,
            token(Token::SColon),
        )
            .map(|(a, b, c, d)| {
                ModuleOrGenerateItemDeclaration::DefaultClocking(Box::new((
                    a, b, c, d,
                )))
            }),
        (
            token(Token::Default),
            token(Token::Disable),
            token(Token::Iff),
            expression_or_dist_parser,
            token(Token::SColon),
        )
            .map(|(a, b, c, d, e)| {
                ModuleOrGenerateItemDeclaration::DefaultDisable(Box::new((
                    a, b, c, d, e,
                )))
            }),
    ))
    .parse_next(input)
}

pub fn non_port_module_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonPortModuleItem<'s>, VerboseError<'s>> {
    alt((
        generate_region_parser.map(|a| NonPortModuleItem::Region(Box::new(a))),
        specify_block_parser.map(|a| NonPortModuleItem::Specify(Box::new(a))),
        (attribute_instance_vec_parser, specparam_assignment_parser)
            .map(|(a, b)| NonPortModuleItem::Specparam(Box::new((a, b)))),
        program_declaration_parser
            .map(|a| NonPortModuleItem::Program(Box::new(a))),
        module_declaration_parser
            .map(|a| NonPortModuleItem::Module(Box::new(a))),
        interface_declaration_parser
            .map(|a| NonPortModuleItem::Interface(Box::new(a))),
        timeunits_declaration_parser
            .map(|a| NonPortModuleItem::Timeunits(Box::new(a))),
        module_or_generate_item_parser
            .map(|a| NonPortModuleItem::ModuleOrGenerate(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn parameter_override_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParameterOverride<'s>, VerboseError<'s>> {
    (
        token(Token::Defparam),
        list_of_defparam_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| ParameterOverride(a, b, c))
        .parse_next(input)
}

pub fn bind_directive_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BindDirective<'s>, VerboseError<'s>> {
    let _instance_parser = (
        token(Token::Bind),
        bind_target_instance_parser,
        bind_instantiation_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| BindDirective::Instance(Box::new((a, b, c, d))));
    let _scope_parser = (
        token(Token::Bind),
        bind_target_scope_parser,
        opt_note((token(Token::Colon), bind_target_instance_list_parser)),
        bind_instantiation_parser,
        token(Token::SColon),
    )
        .verify_map(|(a, b, c, d, e)| {
            let scope_is_instance = match b {
                BindTargetScope::Interface(_) => true,
                BindTargetScope::Module(_) => false,
            };
            let instantiation_is_checker_or_interface = match d {
                BindInstantiation::Checker(_)
                | BindInstantiation::Interface(_) => true,
                BindInstantiation::Module(_)
                | BindInstantiation::Program(_) => false,
            };
            if scope_is_instance & !instantiation_is_checker_or_interface {
                None // BNF clarification 4
            } else {
                Some(BindDirective::Scope(Box::new((a, b, c, d, e))))
            }
        });
    alt((_instance_parser, _scope_parser)).parse_next(input)
}

pub fn bind_target_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BindTargetScope<'s>, VerboseError<'s>> {
    alt((
        module_identifier_parser.map(|a| BindTargetScope::Module(a)),
        interface_identifier_parser.map(|a| BindTargetScope::Interface(a)),
    ))
    .parse_next(input)
}

pub fn bind_target_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BindTargetInstance<'s>, VerboseError<'s>> {
    (hierarchical_identifier_parser, constant_bit_select_parser)
        .map(|(a, b)| BindTargetInstance(a, b))
        .parse_next(input)
}

pub fn bind_target_instance_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BindTargetInstanceList<'s>, VerboseError<'s>> {
    (
        bind_target_instance_parser,
        repeat_note((token(Token::Comma), bind_target_instance_parser)),
    )
        .map(|(a, b)| BindTargetInstanceList(a, b))
        .parse_next(input)
}

pub fn bind_instantiation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BindInstantiation<'s>, VerboseError<'s>> {
    alt((
        program_instantiation_parser
            .map(|a| BindInstantiation::Program(Box::new(a))),
        module_instantiation_parser
            .map(|a| BindInstantiation::Module(Box::new(a))),
        interface_instantiation_parser
            .map(|a| BindInstantiation::Interface(Box::new(a))),
        checker_instantiation_parser
            .map(|a| BindInstantiation::Checker(Box::new(a))),
    ))
    .parse_next(input)
}
