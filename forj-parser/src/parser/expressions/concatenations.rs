// =======================================================================
// concatenations.rs
// =======================================================================
// Parsing for 1800-2023 A.8.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};

pub fn concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Concatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        expression_parser,
        repeat_note((token(Token::Comma), expression_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| Concatenation(a, b, c, d))
        .parse_next(input)
}

pub fn constant_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        constant_expression_parser,
        repeat_note((token(Token::Comma), constant_expression_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| ConstantConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn constant_multiple_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantMultipleConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        constant_expression_parser,
        constant_concatenation_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| ConstantMultipleConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn module_path_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        module_path_expression_parser,
        repeat_note((token(Token::Comma), module_path_expression_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| ModulePathConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn module_path_multiple_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathMultipleConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        constant_expression_parser,
        module_path_concatenation_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| ModulePathMultipleConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn multiple_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MultipleConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        expression_parser,
        concatenation_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| MultipleConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn streaming_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StreamingConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        stream_operator_parser,
        opt_note(terminated(slice_size_parser, peek(token(Token::Brace)))),
        stream_concatenation_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| StreamingConcatenation(a, b, c, d, e))
        .parse_next(input)
}

pub fn stream_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StreamOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::GtGt).map(|a| StreamOperator::Right(a)),
        token(Token::LtLt).map(|a| StreamOperator::Left(a)),
    ))
    .parse_next(input)
}

pub fn slice_size_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SliceSize<'s>, VerboseError<'s>> {
    alt((
        simple_type_parser.map(|a| SliceSize::Simple(Box::new(a))),
        constant_expression_parser.map(|a| SliceSize::ConstExpr(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn stream_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StreamConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        stream_expression_parser,
        repeat_note((token(Token::Comma), stream_expression_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| StreamConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn stream_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StreamExpression<'s>, VerboseError<'s>> {
    (
        expression_parser,
        opt_note((
            token(Token::With),
            token(Token::Bracket),
            array_range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| StreamExpression(a, b))
        .parse_next(input)
}

enum ArrayExtraRange<'s> {
    Range((Metadata<'s>, Expression<'s>)),
    PlusRange((Metadata<'s>, Expression<'s>)),
    MinusRange((Metadata<'s>, Expression<'s>)),
}

pub fn array_range_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ArrayRangeExpression<'s>, VerboseError<'s>> {
    let _extra_range_parser = alt((
        (token(Token::Colon), expression_parser)
            .map(|(a, b)| ArrayExtraRange::Range((a, b))),
        (token(Token::PlusColon), expression_parser)
            .map(|(a, b)| ArrayExtraRange::PlusRange((a, b))),
        (token(Token::MinusColon), expression_parser)
            .map(|(a, b)| ArrayExtraRange::MinusRange((a, b))),
    ));
    (expression_parser, opt_note(_extra_range_parser))
        .map(|(start_range, b)| match b {
            Some(ArrayExtraRange::Range((op, end_range))) => {
                ArrayRangeExpression::Range(Box::new((
                    start_range,
                    op,
                    end_range,
                )))
            }
            Some(ArrayExtraRange::PlusRange((op, end_range))) => {
                ArrayRangeExpression::PlusRange(Box::new((
                    start_range,
                    op,
                    end_range,
                )))
            }
            Some(ArrayExtraRange::MinusRange((op, end_range))) => {
                ArrayRangeExpression::MinusRange(Box::new((
                    start_range,
                    op,
                    end_range,
                )))
            }
            None => ArrayRangeExpression::Select(Box::new(start_range)),
        })
        .parse_next(input)
}

pub fn empty_unpacked_array_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EmptyUnpackedArrayConcatenation<'s>, VerboseError<'s>> {
    (token(Token::Brace), token(Token::EBrace))
        .map(|(a, b)| EmptyUnpackedArrayConcatenation(a, b))
        .parse_next(input)
}
