// =======================================================================
// checker_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.8

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn checker_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerPortList<'s>, VerboseError<'s>> {
    (
        checker_port_item_parser,
        repeat_note((token(Token::Comma), checker_port_item_parser)),
    )
        .map(|(a, b)| CheckerPortList(a, b))
        .parse_next(input)
}

pub fn checker_port_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerPortItem<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        opt_note(checker_port_direction_parser),
        property_formal_type_parser,
        formal_port_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), property_actual_arg_parser)),
    )
        .map(|(a, b, c, d, e, f)| CheckerPortItem(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn checker_port_direction_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerPortDirection<'s>, VerboseError<'s>> {
    alt((
        token(Token::Input).map(|a| CheckerPortDirection::Input(a)),
        token(Token::Output).map(|a| CheckerPortDirection::Output(a)),
    ))
    .parse_next(input)
}

pub fn checker_or_generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerOrGenerateItem<'s>, VerboseError<'s>> {
    alt((
        initial_construct_parser
            .map(|a| CheckerOrGenerateItem::Initial(Box::new(a))),
        always_construct_parser
            .map(|a| CheckerOrGenerateItem::Always(Box::new(a))),
        final_construct_parser
            .map(|a| CheckerOrGenerateItem::Final(Box::new(a))),
        assertion_item_parser
            .map(|a| CheckerOrGenerateItem::Assertion(Box::new(a))),
        continuous_assign_parser
            .map(|a| CheckerOrGenerateItem::Assign(Box::new(a))),
        checker_or_generate_item_declaration_parser
            .map(|a| CheckerOrGenerateItem::Declaration(Box::new(a))),
        checker_generate_item_parser
            .map(|a| CheckerOrGenerateItem::Generate(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn checker_or_generate_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerOrGenerateItemDeclaration<'s>, VerboseError<'s>> {
    alt((
        (opt_note(token(Token::Rand)), data_declaration_parser).map(
            |(a, b)| CheckerOrGenerateItemDeclaration::Data(Box::new((a, b))),
        ),
        function_declaration_parser
            .map(|a| CheckerOrGenerateItemDeclaration::Function(Box::new(a))),
        checker_declaration_parser
            .map(|a| CheckerOrGenerateItemDeclaration::Checker(Box::new(a))),
        assertion_item_declaration_parser.map(|a| {
            CheckerOrGenerateItemDeclaration::AssertionItem(Box::new(a))
        }),
        covergroup_declaration_parser
            .map(|a| CheckerOrGenerateItemDeclaration::Covergroup(Box::new(a))),
        genvar_declaration_parser
            .map(|a| CheckerOrGenerateItemDeclaration::Genvar(Box::new(a))),
        (
            token(Token::Default),
            token(Token::Clocking),
            clocking_identifier_parser,
            token(Token::SColon),
        )
            .map(|(a, b, c, d)| {
                CheckerOrGenerateItemDeclaration::DefaultClocking(Box::new((
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
                CheckerOrGenerateItemDeclaration::DefaultDisable(Box::new((
                    a, b, c, d, e,
                )))
            }),
        token(Token::SColon)
            .map(|a| CheckerOrGenerateItemDeclaration::Null(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn checker_generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerGenerateItem<'s>, VerboseError<'s>> {
    alt((
        loop_generate_construct_parser
            .map(|a| CheckerGenerateItem::Loop(Box::new(a))),
        conditional_generate_construct_parser
            .map(|a| CheckerGenerateItem::Conditional(Box::new(a))),
        generate_region_parser
            .map(|a| CheckerGenerateItem::Region(Box::new(a))),
        elaboration_system_severity_task_parser
            .map(|a| CheckerGenerateItem::ElaborationSeverity(Box::new(a))),
    ))
    .parse_next(input)
}
