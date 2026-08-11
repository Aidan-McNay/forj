// =======================================================================
// system_timing_check_event_definitions.rs
// =======================================================================
// Parsing for 1800-2023 A.7.5.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn timing_check_event_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimingCheckEvent<'s>, VerboseError<'s>> {
    (
        opt_note(timing_check_event_control_parser),
        specify_terminal_descriptor_parser,
        opt_note((token(Token::AmpAmpAmp), timing_check_condition_parser)),
    )
        .map(|(a, b, c)| TimingCheckEvent(a, b, c))
        .parse_next(input)
}

pub fn controlled_timing_check_event_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ControlledTimingCheckEvent<'s>, VerboseError<'s>> {
    (
        timing_check_event_control_parser,
        specify_terminal_descriptor_parser,
        opt_note((token(Token::AmpAmpAmp), timing_check_condition_parser)),
    )
        .map(|(a, b, c)| ControlledTimingCheckEvent(a, b, c))
        .parse_next(input)
}

pub fn timing_check_event_control_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimingCheckEventControl<'s>, VerboseError<'s>> {
    alt((
        token(Token::Posedge)
            .map(|a| TimingCheckEventControl::Posedge(Box::new(a))),
        token(Token::Negedge)
            .map(|a| TimingCheckEventControl::Negedge(Box::new(a))),
        token(Token::Edge).map(|a| TimingCheckEventControl::Edge(Box::new(a))),
        edge_control_specifier_parser
            .map(|a| TimingCheckEventControl::EdgeControl(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn specify_terminal_descriptor_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecifyTerminalDescriptor<'s>, VerboseError<'s>> {
    alt((
        specify_input_terminal_descriptor_parser
            .map(|a| SpecifyTerminalDescriptor::Input(Box::new(a))),
        specify_output_terminal_descriptor_parser
            .map(|a| SpecifyTerminalDescriptor::Output(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn edge_control_specifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EdgeControlSpecifier<'s>, VerboseError<'s>> {
    (
        token(Token::Edge),
        token(Token::Bracket),
        edge_descriptor_parser,
        repeat_note((token(Token::Comma), edge_descriptor_parser)),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| EdgeControlSpecifier(a, b, c, d, e))
        .parse_next(input)
}

pub fn edge_descriptor_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EdgeDescriptor<'s>, VerboseError<'s>> {
    let _number_parser = unsigned_number_parser
        .verify_map(|UnsignedNumber(num, metadata)| {
            if num == "01" {
                Some(EdgeDescriptor::ZeroOne(Box::new(metadata)))
            } else if num == "10" {
                Some(EdgeDescriptor::OneZero(Box::new(metadata)))
            } else {
                None
            }
        })
        .context("01 or 10");
    let _second_x_parser = (
        unsigned_number_parser
            .verify_map(|UnsignedNumber(num, metadata)| {
                if num == "0" {
                    Some((true, metadata))
                } else if num == "1" {
                    Some((false, metadata))
                } else {
                    None
                }
            })
            .context("0 or 1"),
        identifier_parser.verify_map(|a| match a {
            Identifier::SimpleIdentifier((text, metadata)) => {
                if text == "x" {
                    Some(metadata)
                } else if text == "X" {
                    Some(metadata)
                } else {
                    None
                }
            }
            _ => None,
        }),
    )
        .map(|((is_zero, metadata_1), metadata_2)| {
            if is_zero {
                EdgeDescriptor::ZeroX(Box::new((metadata_1, metadata_2)))
            } else {
                EdgeDescriptor::OneX(Box::new((metadata_1, metadata_2)))
            }
        })
        .context("0x or 1x");
    let _second_z_parser = (
        unsigned_number_parser
            .verify_map(|UnsignedNumber(num, metadata)| {
                if num == "0" {
                    Some((true, metadata))
                } else if num == "1" {
                    Some((false, metadata))
                } else {
                    None
                }
            })
            .context("0 or 1"),
        identifier_parser.verify_map(|a| match a {
            Identifier::SimpleIdentifier((text, metadata)) => {
                if text == "z" {
                    Some(metadata)
                } else if text == "Z" {
                    Some(metadata)
                } else {
                    None
                }
            }
            _ => None,
        }),
    )
        .map(|((is_zero, metadata_1), metadata_2)| {
            if is_zero {
                EdgeDescriptor::ZeroX(Box::new((metadata_1, metadata_2)))
            } else {
                EdgeDescriptor::OneX(Box::new((metadata_1, metadata_2)))
            }
        })
        .context("0x or 1x");
    let _first_x_parser = identifier_parser
        .verify_map(|a| match a {
            Identifier::SimpleIdentifier((text, metadata)) => {
                if text == "x0" {
                    Some(EdgeDescriptor::XZero(Box::new(metadata)))
                } else if text == "X0" {
                    Some(EdgeDescriptor::XZero(Box::new(metadata)))
                } else if text == "x1" {
                    Some(EdgeDescriptor::XOne(Box::new(metadata)))
                } else if text == "X1" {
                    Some(EdgeDescriptor::XOne(Box::new(metadata)))
                } else {
                    None
                }
            }
            _ => None,
        })
        .context("x0 or x1");
    let _first_z_parser = identifier_parser
        .verify_map(|a| match a {
            Identifier::SimpleIdentifier((text, metadata)) => {
                if text == "z0" {
                    Some(EdgeDescriptor::ZZero(Box::new(metadata)))
                } else if text == "Z0" {
                    Some(EdgeDescriptor::ZZero(Box::new(metadata)))
                } else if text == "z1" {
                    Some(EdgeDescriptor::ZOne(Box::new(metadata)))
                } else if text == "Z1" {
                    Some(EdgeDescriptor::ZOne(Box::new(metadata)))
                } else {
                    None
                }
            }
            _ => None,
        })
        .context("z0 or z1");
    alt((
        _number_parser,
        _first_x_parser,
        _first_z_parser,
        _second_x_parser,
        _second_z_parser,
    ))
    .parse_next(input)
}

pub fn timing_check_condition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimingCheckCondition<'s>, VerboseError<'s>> {
    alt((
        scalar_timing_check_condition_parser
            .map(|a| TimingCheckCondition::NoParen(Box::new(a))),
        (
            token(Token::Paren),
            scalar_timing_check_condition_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c)| TimingCheckCondition::Paren(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

fn scalar_constant<'s>(
    expression: Expression<'s>,
) -> Option<ScalarConstant<'s>> {
    match expression {
        Expression::Primary(inner_box) => match *inner_box {
            Primary::PrimaryLiteral(inner_box) => match *inner_box {
                PrimaryLiteral::Number(inner_box) => match *inner_box {
                    Number::Integral(inner_box) => match *inner_box {
                        IntegralNumber::Binary(inner_box) => match *inner_box {
                            BinaryNumber(text, metadata) => match text {
                                "1'b0" => {
                                    Some(ScalarConstant::Sizedb0(metadata))
                                }
                                "1'b1" => {
                                    Some(ScalarConstant::Sizedb1(metadata))
                                }
                                "1'B0" => {
                                    Some(ScalarConstant::SizedB0(metadata))
                                }
                                "1'B1" => {
                                    Some(ScalarConstant::SizedB1(metadata))
                                }
                                "'b0" => {
                                    Some(ScalarConstant::Unsizedb0(metadata))
                                }
                                "'b1" => {
                                    Some(ScalarConstant::Unsizedb1(metadata))
                                }
                                "'B0" => {
                                    Some(ScalarConstant::UnsizedB0(metadata))
                                }
                                "'B1" => {
                                    Some(ScalarConstant::UnsizedB1(metadata))
                                }
                                _ => None,
                            },
                        },
                        IntegralNumber::Decimal(inner_box) => {
                            match *inner_box {
                                DecimalNumber::Unsized(inner_box) => {
                                    match *inner_box {
                                        UnsignedNumber(text, metadata) => {
                                            match text {
                                                "1" => {
                                                    Some(ScalarConstant::One(
                                                        metadata,
                                                    ))
                                                }
                                                "0" => {
                                                    Some(ScalarConstant::Zero(
                                                        metadata,
                                                    ))
                                                }
                                                _ => None,
                                            }
                                        }
                                    }
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

pub fn scalar_timing_check_condition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ScalarTimingCheckCondition<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| match a {
            Expression::Unary(inner_box) => match *inner_box {
                (UnaryOperator::Tilde(tilde_metadata), attr_vec, primary) => {
                    if attr_vec.is_empty() {
                        ScalarTimingCheckCondition::Invert(Box::new((
                            tilde_metadata,
                            Expression::Primary(Box::new(primary)),
                        )))
                    } else {
                        ScalarTimingCheckCondition::Base(Box::new(
                            Expression::Unary(Box::new((
                                UnaryOperator::Tilde(tilde_metadata),
                                attr_vec,
                                primary,
                            ))),
                        ))
                    }
                }
                _ => ScalarTimingCheckCondition::Base(Box::new(
                    Expression::Unary(inner_box),
                )),
            },
            Expression::Binary(inner_box) => {
                match *inner_box {
                    (lhs, binop, attr_vec, rhs) => {
                        if attr_vec.is_empty() {
                            match (binop, scalar_constant(rhs.clone())) {
                                (
                                    BinaryOperator::EqEq(binop_metadata),
                                    Some(sc),
                                ) => ScalarTimingCheckCondition::EqEq(
                                    Box::new((lhs, binop_metadata, sc)),
                                ),
                                (
                                    BinaryOperator::EqEqEq(binop_metadata),
                                    Some(sc),
                                ) => ScalarTimingCheckCondition::EqEqEq(
                                    Box::new((lhs, binop_metadata, sc)),
                                ),
                                (
                                    BinaryOperator::ExclEq(binop_metadata),
                                    Some(sc),
                                ) => ScalarTimingCheckCondition::ExclEq(
                                    Box::new((lhs, binop_metadata, sc)),
                                ),
                                (
                                    BinaryOperator::ExclEqEq(binop_metadata),
                                    Some(sc),
                                ) => ScalarTimingCheckCondition::ExclEqEq(
                                    Box::new((lhs, binop_metadata, sc)),
                                ),
                                (binop, _) => ScalarTimingCheckCondition::Base(
                                    Box::new(Expression::Binary(Box::new((
                                        lhs, binop, attr_vec, rhs,
                                    )))),
                                ),
                            }
                        } else {
                            ScalarTimingCheckCondition::Base(Box::new(
                                Expression::Binary(Box::new((
                                    lhs, binop, attr_vec, rhs,
                                ))),
                            ))
                        }
                    }
                }
            }
            _ => ScalarTimingCheckCondition::Base(Box::new(a)),
        })
        .parse_next(input)
}
