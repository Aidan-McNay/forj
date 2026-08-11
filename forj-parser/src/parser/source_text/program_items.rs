// =======================================================================
// program_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.7

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn program_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramItem<'s>, VerboseError<'s>> {
    alt((
        (port_declaration_parser, token(Token::SColon))
            .map(|(a, b)| ProgramItem::Port(Box::new((a, b)))),
        non_port_program_item_parser.map(|a| ProgramItem::NonPort(Box::new(a))),
    ))
    .parse_next(input)
}

enum NonPortProgramItemBody<'a> {
    Assign(ContinuousAssign<'a>),
    ModuleOrGenerateDeclaration(ModuleOrGenerateItemDeclaration<'a>),
    Initial(InitialConstruct<'a>),
    Final(FinalConstruct<'a>),
    Assertion(ConcurrentAssertionItem<'a>),
}

pub fn non_port_program_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonPortProgramItem<'s>, VerboseError<'s>> {
    let _attribute_item_parser = (
        attribute_instance_vec_parser,
        alt((
            continuous_assign_parser.map(|a| NonPortProgramItemBody::Assign(a)),
            module_or_generate_item_declaration_parser.map(|a| {
                NonPortProgramItemBody::ModuleOrGenerateDeclaration(a)
            }),
            initial_construct_parser
                .map(|a| NonPortProgramItemBody::Initial(a)),
            final_construct_parser.map(|a| NonPortProgramItemBody::Final(a)),
            concurrent_assertion_item_parser
                .map(|a| NonPortProgramItemBody::Assertion(a)),
        )),
    )
        .map(|(a, b)| match b {
            NonPortProgramItemBody::Assign(c) => {
                NonPortProgramItem::Assign(Box::new((a, c)))
            }
            NonPortProgramItemBody::ModuleOrGenerateDeclaration(c) => {
                NonPortProgramItem::ModuleOrGenerateDeclaration(Box::new((
                    a, c,
                )))
            }
            NonPortProgramItemBody::Initial(c) => {
                NonPortProgramItem::Initial(Box::new((a, c)))
            }
            NonPortProgramItemBody::Final(c) => {
                NonPortProgramItem::Final(Box::new((a, c)))
            }
            NonPortProgramItemBody::Assertion(c) => {
                NonPortProgramItem::Assertion(Box::new((a, c)))
            }
        });
    alt((
        _attribute_item_parser,
        timeunits_declaration_parser
            .map(|a| NonPortProgramItem::Timeunits(Box::new(a))),
        program_generate_item_parser
            .map(|a| NonPortProgramItem::Generate(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn program_generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramGenerateItem<'s>, VerboseError<'s>> {
    alt((
        loop_generate_construct_parser
            .map(|a| ProgramGenerateItem::Loop(Box::new(a))),
        conditional_generate_construct_parser
            .map(|a| ProgramGenerateItem::Conditional(Box::new(a))),
        generate_region_parser
            .map(|a| ProgramGenerateItem::Region(Box::new(a))),
        elaboration_system_severity_task_parser
            .map(|a| ProgramGenerateItem::ElaborationSeverity(Box::new(a))),
    ))
    .parse_next(input)
}
