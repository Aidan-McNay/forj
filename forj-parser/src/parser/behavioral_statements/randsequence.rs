// =======================================================================
// randsequence.rs
// =======================================================================
// Parsing for 1800-2023 A.6.12

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};

pub fn randsequence_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RandsequenceStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Randsequence),
        token(Token::Paren),
        opt_note(rs_production_identifier_parser),
        token(Token::EParen),
        rs_production_parser,
        repeat_note(rs_production_parser),
        token(Token::Endsequence),
    )
        .map(|(a, b, c, d, e, f, g)| RandsequenceStatement(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn rs_production_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsProduction<'s>, VerboseError<'s>> {
    (
        opt_note(terminated(
            data_type_or_void_parser,
            peek(rs_production_identifier_parser),
        )),
        rs_production_identifier_parser,
        opt_note((
            token(Token::Paren),
            tf_port_list_parser,
            token(Token::EParen),
        )),
        token(Token::Colon),
        rs_rule_parser,
        repeat_note((token(Token::Pipe), rs_rule_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g)| RsProduction(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn rs_rule_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsRule<'s>, VerboseError<'s>> {
    (
        rs_production_list_parser,
        opt_note((
            token(Token::ColonEq),
            rs_weight_specification_parser,
            opt_note(rs_code_block_parser),
        )),
    )
        .map(|(a, b)| RsRule(a, b))
        .parse_next(input)
}

pub fn rs_production_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsProductionList<'s>, VerboseError<'s>> {
    let _producers_parser = (rs_prod_parser, repeat_note(rs_prod_parser))
        .map(|(a, b)| RsProductionList::Producers(Box::new((a, b))));
    let _join_parser = (
        token(Token::Rand),
        token(Token::Join),
        opt_note((
            token(Token::Paren),
            expression_parser,
            token(Token::EParen),
        )),
        rs_production_item_parser,
        rs_production_item_parser,
        repeat_note(rs_production_item_parser),
    )
        .map(|(a, b, c, d, e, f)| {
            RsProductionList::Join(Box::new((a, b, c, d, e, f)))
        });
    alt((_producers_parser, _join_parser)).parse_next(input)
}

pub fn rs_weight_specification_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsWeightSpecification<'s>, VerboseError<'s>> {
    alt((
        integral_number_parser
            .map(|a| RsWeightSpecification::Integral(Box::new(a))),
        ps_identifier_parser.map(|a| RsWeightSpecification::Ps(Box::new(a))),
        (token(Token::Paren), expression_parser, token(Token::EParen)).map(
            |(a, b, c)| RsWeightSpecification::Expression(Box::new((a, b, c))),
        ),
    ))
    .parse_next(input)
}

pub fn rs_code_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsCodeBlock<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        repeat_note(data_declaration_parser),
        repeat_note(statement_or_null_parser),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| RsCodeBlock(a, b, c, d))
        .parse_next(input)
}

pub fn rs_prod_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsProd<'s>, VerboseError<'s>> {
    alt((
        rs_production_item_parser.map(|a| RsProd::Item(Box::new(a))),
        rs_code_block_parser.map(|a| RsProd::CodeBlock(Box::new(a))),
        rs_if_else_parser.map(|a| RsProd::IfElse(Box::new(a))),
        rs_repeat_parser.map(|a| RsProd::Repeat(Box::new(a))),
        rs_case_parser.map(|a| RsProd::Case(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn rs_production_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsProductionItem<'s>, VerboseError<'s>> {
    (
        rs_production_identifier_parser,
        opt_note((
            token(Token::Paren),
            list_of_arguments_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b)| RsProductionItem(a, b))
        .parse_next(input)
}

pub fn rs_if_else_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsIfElse<'s>, VerboseError<'s>> {
    (
        token(Token::If),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        rs_production_item_parser,
        opt_note((token(Token::Else), rs_production_item_parser)),
    )
        .map(|(a, b, c, d, e, f)| RsIfElse(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn rs_repeat_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsRepeat<'s>, VerboseError<'s>> {
    (
        token(Token::Repeat),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        rs_production_item_parser,
    )
        .map(|(a, b, c, d, e)| RsRepeat(a, b, c, d, e))
        .parse_next(input)
}

pub fn rs_case_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsCase<'s>, VerboseError<'s>> {
    (
        token(Token::Case),
        token(Token::Paren),
        case_expression_parser,
        token(Token::EParen),
        rs_case_item_parser,
        repeat_note(rs_case_item_parser),
        token(Token::Endcase),
    )
        .map(|(a, b, c, d, e, f, g)| RsCase(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn rs_case_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsCaseItem<'s>, VerboseError<'s>> {
    let _expression_parser = (
        case_item_expression_parser,
        repeat_note((token(Token::Comma), case_item_expression_parser)),
        token(Token::Colon),
        rs_production_item_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            RsCaseItem::Expression(Box::new((a, b, c, d, e)))
        });
    let _default_parser = (
        token(Token::Default),
        opt_note(token(Token::Comma)),
        rs_production_item_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| RsCaseItem::Default(Box::new((a, b, c, d))));
    alt((_default_parser, _expression_parser)).parse_next(input)
}
