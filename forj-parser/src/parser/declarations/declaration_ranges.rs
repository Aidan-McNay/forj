// =======================================================================
// declaration_ranges.rs
// =======================================================================
// Parsing for 1800-2023 A.2.5

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn unpacked_dimension_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UnpackedDimension<'s>, VerboseError<'s>> {
    let unpacked_range_parser = (
        token(Token::Bracket),
        constant_range_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c)| UnpackedDimension::UnpackedRange(Box::new((a, b, c))));
    let unpacked_expression_parser = (
        token(Token::Bracket),
        constant_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c)| {
            UnpackedDimension::UnpackedExpression(Box::new((a, b, c)))
        });
    alt((unpacked_range_parser, unpacked_expression_parser)).parse_next(input)
}

pub fn packed_dimension_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackedDimension<'s>, VerboseError<'s>> {
    let packed_range_parser = (
        token(Token::Bracket),
        constant_range_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c)| PackedDimension::PackedRange(Box::new((a, b, c))));
    alt((
        packed_range_parser,
        unsized_dimension_parser
            .map(|a| PackedDimension::UnsizedDimension(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn associative_dimension_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssociativeDimension<'s>, VerboseError<'s>> {
    let data_parser = (
        token(Token::Bracket),
        data_type_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c)| AssociativeDimension::Data(Box::new((a, b, c))));
    let star_parser = (
        token(Token::Bracket),
        token(Token::Star),
        token(Token::EBracket),
    )
        .map(|(a, b, c)| AssociativeDimension::Star(Box::new((a, b, c))));
    alt((data_parser, star_parser)).parse_next(input)
}

pub fn variable_dimension_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariableDimension<'s>, VerboseError<'s>> {
    alt((
        unsized_dimension_parser
            .map(|a| VariableDimension::UnsizedDimension(Box::new(a))),
        unpacked_dimension_parser
            .map(|a| VariableDimension::UnpackedDimension(Box::new(a))),
        associative_dimension_parser
            .map(|a| VariableDimension::AssociativeDimension(Box::new(a))),
        queue_dimension_parser
            .map(|a| VariableDimension::QueueDimension(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn queue_dimension_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<QueueDimension<'s>, VerboseError<'s>> {
    (
        token(Token::Bracket),
        token(Token::Dollar),
        opt_note((token(Token::Colon), constant_expression_parser)),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| QueueDimension(a, b, c, d))
        .parse_next(input)
}

pub fn unsized_dimension_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UnsizedDimension<'s>, VerboseError<'s>> {
    (token(Token::Bracket), token(Token::EBracket))
        .map(|(a, b)| UnsizedDimension(a, b))
        .parse_next(input)
}
