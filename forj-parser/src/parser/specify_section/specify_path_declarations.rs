// =======================================================================
// specify_path_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.7.2

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn path_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PathDeclaration<'s>, VerboseError<'s>> {
    alt((
        (simple_path_declaration_parser, token(Token::SColon))
            .map(|(a, b)| PathDeclaration::Simple(Box::new((a, b)))),
        (edge_sensitive_path_declaration_parser, token(Token::SColon))
            .map(|(a, b)| PathDeclaration::EdgeSensitive(Box::new((a, b)))),
        (
            state_dependent_path_declaration_parser,
            token(Token::SColon),
        )
            .map(|(a, b)| PathDeclaration::StateDependent(Box::new((a, b)))),
    ))
    .parse_next(input)
}

pub fn simple_path_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SimplePathDeclaration<'s>, VerboseError<'s>> {
    alt((
        (
            parallel_path_description_parser,
            token(Token::Eq),
            path_delay_value_parser,
        )
            .map(|(a, b, c)| {
                SimplePathDeclaration::Parallel(Box::new((a, b, c)))
            }),
        (
            full_path_description_parser,
            token(Token::Eq),
            path_delay_value_parser,
        )
            .map(|(a, b, c)| SimplePathDeclaration::Full(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn parallel_path_description_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParallelPathDescription<'s>, VerboseError<'s>> {
    (
        token(Token::Paren),
        specify_input_terminal_descriptor_parser,
        opt_note(polarity_operator_parser),
        token(Token::EqGt),
        specify_output_terminal_descriptor_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| ParallelPathDescription(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn full_path_description_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FullPathDescription<'s>, VerboseError<'s>> {
    (
        token(Token::Paren),
        list_of_path_inputs_parser,
        opt_note(polarity_operator_parser),
        token(Token::StarGt),
        list_of_path_outputs_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| FullPathDescription(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn edge_sensitive_path_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EdgeSensitivePathDeclaration<'s>, VerboseError<'s>> {
    alt((
        (
            parallel_edge_sensitive_path_description_parser,
            token(Token::Eq),
            path_delay_value_parser,
        )
            .map(|(a, b, c)| {
                EdgeSensitivePathDeclaration::Parallel(Box::new((a, b, c)))
            }),
        (
            full_edge_sensitive_path_description_parser,
            token(Token::Eq),
            path_delay_value_parser,
        )
            .map(|(a, b, c)| {
                EdgeSensitivePathDeclaration::Full(Box::new((a, b, c)))
            }),
    ))
    .parse_next(input)
}

pub fn parallel_edge_sensitive_path_description_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParallelEdgeSensitivePathDescription<'s>, VerboseError<'s>> {
    let _data_source_parser = (
        token(Token::Paren),
        opt_note(edge_identifier_parser),
        specify_input_terminal_descriptor_parser,
        opt_note(polarity_operator_parser),
        token(Token::EqGt),
        token(Token::Paren),
        specify_output_terminal_descriptor_parser,
        opt_note(polarity_operator_parser),
        token(Token::Colon),
        data_source_expression_parser,
        token(Token::EParen),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            ParallelEdgeSensitivePathDescription::DataSource(Box::new((
                a, b, c, d, e, f, g, h, i, j, k, l,
            )))
        });
    let _no_data_source_parser = (
        token(Token::Paren),
        opt_note(edge_identifier_parser),
        specify_input_terminal_descriptor_parser,
        opt_note(polarity_operator_parser),
        token(Token::EqGt),
        specify_output_terminal_descriptor_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g)| {
            ParallelEdgeSensitivePathDescription::NoDataSource(Box::new((
                a, b, c, d, e, f, g,
            )))
        });
    alt((_data_source_parser, _no_data_source_parser)).parse_next(input)
}

pub fn full_edge_sensitive_path_description_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FullEdgeSensitivePathDescription<'s>, VerboseError<'s>> {
    let _data_source_parser = (
        token(Token::Paren),
        opt_note(edge_identifier_parser),
        list_of_path_inputs_parser,
        opt_note(polarity_operator_parser),
        token(Token::StarGt),
        token(Token::Paren),
        list_of_path_outputs_parser,
        opt_note(polarity_operator_parser),
        token(Token::Colon),
        data_source_expression_parser,
        token(Token::EParen),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            FullEdgeSensitivePathDescription::DataSource(Box::new((
                a, b, c, d, e, f, g, h, i, j, k, l,
            )))
        });
    let _no_data_source_parser = (
        token(Token::Paren),
        opt_note(edge_identifier_parser),
        list_of_path_inputs_parser,
        opt_note(polarity_operator_parser),
        token(Token::StarGt),
        list_of_path_outputs_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g)| {
            FullEdgeSensitivePathDescription::NoDataSource(Box::new((
                a, b, c, d, e, f, g,
            )))
        });
    alt((_data_source_parser, _no_data_source_parser)).parse_next(input)
}

pub fn state_dependent_path_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StateDependentPathDeclaration<'s>, VerboseError<'s>> {
    alt((
        (
            token(Token::If),
            token(Token::Paren),
            module_path_expression_parser,
            token(Token::EParen),
            simple_path_declaration_parser,
        )
            .map(|(a, b, c, d, e)| {
                StateDependentPathDeclaration::Simple(Box::new((a, b, c, d, e)))
            }),
        (
            token(Token::If),
            token(Token::Paren),
            module_path_expression_parser,
            token(Token::EParen),
            edge_sensitive_path_declaration_parser,
        )
            .map(|(a, b, c, d, e)| {
                StateDependentPathDeclaration::EdgeSensitive(Box::new((
                    a, b, c, d, e,
                )))
            }),
        (token(Token::Ifnone), simple_path_declaration_parser).map(|(a, b)| {
            StateDependentPathDeclaration::NoCondition(Box::new((a, b)))
        }),
    ))
    .parse_next(input)
}

pub fn data_source_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataSourceExpression<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| DataSourceExpression(a))
        .parse_next(input)
}

pub fn edge_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EdgeIdentifier<'s>, VerboseError<'s>> {
    alt((
        token(Token::Posedge).map(|a| EdgeIdentifier::Posedge(a)),
        token(Token::Negedge).map(|a| EdgeIdentifier::Negedge(a)),
        token(Token::Edge).map(|a| EdgeIdentifier::Edge(a)),
    ))
    .parse_next(input)
}

pub fn polarity_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PolarityOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::Plus).map(|a| PolarityOperator::Plus(a)),
        token(Token::Minus).map(|a| PolarityOperator::Minus(a)),
    ))
    .parse_next(input)
}
