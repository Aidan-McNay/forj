// =======================================================================
// generated_instantiation.rs
// =======================================================================
// Parsing for 1800-2023 A.4.2

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn generate_region_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenerateRegion<'s>, VerboseError<'s>> {
    (
        token(Token::Generate),
        repeat_note(generate_item_parser),
        token(Token::Endgenerate),
    )
        .map(|(a, b, c)| GenerateRegion(a, b, c))
        .parse_next(input)
}

pub fn loop_generate_construct_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LoopGenerateConstruct<'s>, VerboseError<'s>> {
    (
        token(Token::For),
        token(Token::Paren),
        genvar_initialization_parser,
        token(Token::SColon),
        genvar_expression_parser,
        token(Token::SColon),
        genvar_iteration_parser,
        token(Token::EParen),
        generate_block_parser,
    )
        .map(|(a, b, c, d, e, f, g, h, i)| {
            LoopGenerateConstruct(a, b, c, d, e, f, g, h, i)
        })
        .parse_next(input)
}

pub fn genvar_initialization_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenvarInitialization<'s>, VerboseError<'s>> {
    (
        opt_note(token(Token::Genvar)),
        genvar_identifier_parser,
        token(Token::Eq),
        constant_expression_parser,
    )
        .map(|(a, b, c, d)| GenvarInitialization(a, b, c, d))
        .parse_next(input)
}

pub fn genvar_iteration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenvarIteration<'s>, VerboseError<'s>> {
    let _assignment_parser = (
        genvar_identifier_parser,
        assignment_operator_parser,
        genvar_expression_parser,
    )
        .map(|(a, b, c)| GenvarIteration::Assignment(Box::new((a, b, c))));
    let _prefix_parser = (inc_or_dec_operator_parser, genvar_identifier_parser)
        .map(|(a, b)| GenvarIteration::Prefix(Box::new((a, b))));
    let _postfix_parser =
        (genvar_identifier_parser, inc_or_dec_operator_parser)
            .map(|(a, b)| GenvarIteration::Postfix(Box::new((a, b))));
    alt((_assignment_parser, _prefix_parser, _postfix_parser)).parse_next(input)
}

pub fn conditional_generate_construct_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConditionalGenerateConstruct<'s>, VerboseError<'s>> {
    alt((
        if_generate_construct_parser
            .map(|a| ConditionalGenerateConstruct::If(Box::new(a))),
        case_generate_construct_parser
            .map(|a| ConditionalGenerateConstruct::Case(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn if_generate_construct_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IfGenerateConstruct<'s>, VerboseError<'s>> {
    (
        token(Token::If),
        token(Token::Paren),
        constant_expression_parser,
        token(Token::EParen),
        generate_block_parser,
        opt_note((token(Token::Else), generate_block_parser)),
    )
        .map(|(a, b, c, d, e, f)| IfGenerateConstruct(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn case_generate_construct_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CaseGenerateConstruct<'s>, VerboseError<'s>> {
    (
        token(Token::Case),
        token(Token::Paren),
        constant_expression_parser,
        token(Token::EParen),
        case_generate_item_parser,
        repeat_note(case_generate_item_parser),
        token(Token::Endcase),
    )
        .map(|(a, b, c, d, e, f, g)| CaseGenerateConstruct(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn case_generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CaseGenerateItem<'s>, VerboseError<'s>> {
    let _expression_parser = (
        constant_expression_parser,
        repeat_note((token(Token::Comma), constant_expression_parser)),
        token(Token::Colon),
        generate_block_parser,
    )
        .map(|(a, b, c, d)| {
            CaseGenerateItem::Expression(Box::new((a, b, c, d)))
        });
    let _default_parser = (
        token(Token::Default),
        opt_note(token(Token::Comma)),
        generate_block_parser,
    )
        .map(|(a, b, c)| CaseGenerateItem::Default(Box::new((a, b, c))));
    alt((_default_parser, _expression_parser)).parse_next(input)
}

pub fn generate_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenerateBlock<'s>, VerboseError<'s>> {
    let _block_parser = (
        opt_note((generate_block_identifier_parser, token(Token::Colon))),
        token(Token::Begin),
        opt_note((token(Token::Colon), generate_block_identifier_parser)),
        repeat_note(generate_item_parser),
        token(Token::End),
        opt_note((token(Token::Colon), generate_block_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f)| {
            GenerateBlock::Block(Box::new((a, b, c, d, e, f)))
        });
    alt((
        _block_parser,
        generate_item_parser.map(|a| GenerateBlock::Item(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenerateItem<'s>, VerboseError<'s>> {
    alt((
        module_or_generate_item_parser
            .map(|a| GenerateItem::Module(Box::new(a))),
        interface_or_generate_item_parser
            .map(|a| GenerateItem::Interface(Box::new(a))),
        checker_or_generate_item_parser
            .map(|a| GenerateItem::Checker(Box::new(a))),
    ))
    .parse_next(input)
}
