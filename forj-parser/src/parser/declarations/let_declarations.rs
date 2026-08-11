// =======================================================================
// let_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.12

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};

pub fn let_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LetDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Let),
        let_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(let_port_list_parser),
            token(Token::EParen),
        )),
        token(Token::Eq),
        expression_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| LetDeclaration(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn let_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LetIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| LetIdentifier(a))
        .parse_next(input)
}

pub fn let_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LetPortList<'s>, VerboseError<'s>> {
    (
        let_port_item_parser,
        repeat_note((token(Token::Comma), let_port_item_parser)),
    )
        .map(|(a, b)| LetPortList(a, b))
        .parse_next(input)
}

pub fn let_port_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LetPortItem<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        let_formal_type_parser,
        formal_port_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), expression_parser)),
    )
        .map(|(a, b, c, d, e)| LetPortItem(a, b, c, d, e))
        .parse_next(input)
}

pub fn let_formal_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LetFormalType<'s>, VerboseError<'s>> {
    alt((
        data_type_or_implicit_parser_let_formal_type
            .map(|a| LetFormalType::DataTypeOrImplicit(Box::new(a))),
        token(Token::Untyped).map(|a| LetFormalType::Untyped(Box::new(a))),
    ))
    .parse_next(input)
}

fn data_type_or_implicit_parser_let_formal_type<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataTypeOrImplicit<'s>, VerboseError<'s>> {
    alt((
        terminated(data_type_parser, peek(formal_port_identifier_parser))
            .map(|a| DataTypeOrImplicit::DataType(a)),
        terminated(
            implicit_data_type_parser,
            peek(formal_port_identifier_parser),
        )
        .map(|a| DataTypeOrImplicit::ImplicitDataType(a)),
    ))
    .parse_next(input)
}

pub fn let_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LetExpression<'s>, VerboseError<'s>> {
    (
        opt_note(package_scope_parser),
        let_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(let_list_of_arguments_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c)| LetExpression(a, b, c))
        .parse_next(input)
}

pub fn let_list_of_arguments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LetListOfArguments<'s>, VerboseError<'s>> {
    let _partial_identifier_parser = (
        opt_note(let_actual_arg_parser),
        repeat_note((token(Token::Comma), opt_note(let_actual_arg_parser))),
        repeat_note((
            token(Token::Comma),
            token(Token::Period),
            identifier_parser,
            token(Token::Paren),
            opt_note(let_actual_arg_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c)| LetListOfPartialIdentifierArguments(a, b, c));
    let _identifier_parser = (
        token(Token::Period),
        identifier_parser,
        token(Token::Paren),
        opt_note(let_actual_arg_parser),
        token(Token::EParen),
        repeat_note((
            token(Token::Comma),
            token(Token::Period),
            identifier_parser,
            token(Token::Paren),
            opt_note(let_actual_arg_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d, e, f)| {
            LetListOfIdentifierArguments(a, b, c, d, e, f)
        });
    alt((
        _partial_identifier_parser
            .map(|a| LetListOfArguments::PartialIdentifier(Box::new(a))),
        _identifier_parser.map(|a| LetListOfArguments::Identifier(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn let_actual_arg_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LetActualArg<'s>, VerboseError<'s>> {
    expression_parser.map(|a| LetActualArg(a)).parse_next(input)
}
