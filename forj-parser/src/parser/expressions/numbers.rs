// =======================================================================
// numbers.rs
// =======================================================================
// Parsing for 1800-2023 A.8.7

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::token::any;

pub fn number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Number<'s>, VerboseError<'s>> {
    alt((
        integral_number_parser.map(|a| Number::Integral(Box::new(a))),
        real_number_parser.map(|a| Number::Real(Box::new(a))),
    ))
    .context("a number")
    .parse_next(input)
}

pub fn integral_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IntegralNumber<'s>, VerboseError<'s>> {
    alt((
        decimal_number_parser.map(|a| IntegralNumber::Decimal(Box::new(a))),
        octal_number_parser.map(|a| IntegralNumber::Octal(Box::new(a))),
        binary_number_parser.map(|a| IntegralNumber::Binary(Box::new(a))),
        hex_number_parser.map(|a| IntegralNumber::Hex(Box::new(a))),
    ))
    .context("an integral number")
    .parse_next(input)
}

pub fn decimal_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DecimalNumber<'s>, VerboseError<'s>> {
    let _sized_parser = (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::DecimalNumber(num) => {
                Some((num, Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|((num, metadata), non_trivia)| {
            DecimalNumber::Sized(Box::new((
                num,
                replace_non_trivia(metadata, non_trivia),
            )))
        });
    alt((
        _sized_parser,
        unsigned_number_parser.map(|a| DecimalNumber::Unsized(Box::new(a))),
    ))
    .context("a decimal number")
    .parse_next(input)
}

pub fn binary_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinaryNumber<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::BinaryNumber(num) => {
                Some((num, Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|((num, metadata), non_trivia)| {
            BinaryNumber(num, replace_non_trivia(metadata, non_trivia))
        })
        .context("a binary number")
        .parse_next(input)
}

pub fn octal_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OctalNumber<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::OctalNumber(num) => {
                Some((num, Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|((num, metadata), non_trivia)| {
            OctalNumber(num, replace_non_trivia(metadata, non_trivia))
        })
        .context("an octal number")
        .parse_next(input)
}

pub fn hex_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HexNumber<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::HexNumber(num) => {
                Some((num, Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|((num, metadata), non_trivia)| {
            HexNumber(num, replace_non_trivia(metadata, non_trivia))
        })
        .context("a hex number")
        .parse_next(input)
}

pub fn real_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RealNumber<'s>, VerboseError<'s>> {
    alt((
        fixed_point_number_parser.map(|a| RealNumber::FixedPoint(Box::new(a))),
        scientific_number_parser.map(|a| RealNumber::Scientific(Box::new(a))),
    ))
    .context("a real number")
    .parse_next(input)
}

pub fn fixed_point_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FixedPointNumber<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::FixedPointNumber(num) => {
                Some((num, Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|((num, metadata), non_trivia)| {
            FixedPointNumber(num, replace_non_trivia(metadata, non_trivia))
        })
        .context("a fixed-point number")
        .parse_next(input)
}

pub fn scientific_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ScientificNumber<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::ScientificNumber(num) => {
                Some((num, Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|((num, metadata), non_trivia)| {
            ScientificNumber(num, replace_non_trivia(metadata, non_trivia))
        })
        .context("a scientific-notation number")
        .parse_next(input)
}

pub fn unsigned_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UnsignedNumber<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::UnsignedNumber(num) => {
                Some((num, Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|((num, metadata), non_trivia)| {
            UnsignedNumber(num, replace_non_trivia(metadata, non_trivia))
        })
        .context("an unsigned number")
        .parse_next(input)
}

pub fn unbased_unsized_literal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UnbasedUnsizedLiteral<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::UnbasedUnsizedLiteral(num) => {
                Some((num, Metadata::new(s.1.clone(), vec![])))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|((num, metadata), non_trivia)| {
            UnbasedUnsizedLiteral(num, replace_non_trivia(metadata, non_trivia))
        })
        .context("an unsized literal")
        .parse_next(input)
}
