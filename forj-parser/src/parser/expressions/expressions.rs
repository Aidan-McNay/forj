// =======================================================================
// expressions.rs
// =======================================================================
// Parsing for 1800-2023 A.8.3
//
// Pratt parsing adapted from https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, not, peek, terminated};

pub fn inc_or_dec_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IncOrDecExpression<'s>, VerboseError<'s>> {
    let _preop_parser = (
        inc_or_dec_operator_parser,
        attribute_instance_vec_parser,
        variable_lvalue_parser,
    )
        .map(|(a, b, c)| IncOrDecExpression::Preop(Box::new((a, b, c))));
    let _postop_parser = (
        variable_lvalue_parser,
        attribute_instance_vec_parser,
        inc_or_dec_operator_parser,
    )
        .map(|(a, b, c)| IncOrDecExpression::Postop(Box::new((a, b, c))));
    alt((_preop_parser, _postop_parser)).parse_next(input)
}

#[allow(dead_code)] // Parsed in Pratt parsing
pub fn conditional_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConditionalExpression<'s>, VerboseError<'s>> {
    (
        cond_predicate_parser,
        token(Token::Quest),
        attribute_instance_vec_parser,
        expression_parser,
        token(Token::Colon),
        expression_parser,
    )
        .map(|(a, b, c, d, e, f)| ConditionalExpression(a, b, c, d, e, f))
        .parse_next(input)
}

enum ConstantExpressionPrattOp<'a> {
    Binary(BinaryOperator<'a>),
    Ternary(Metadata<'a>),
}

#[inline]
fn binary_operator_binding_power<'s>(binop: &BinaryOperator<'s>) -> (u8, u8) {
    match binop {
        BinaryOperator::StarStar(_) => left_assoc(15),
        BinaryOperator::Star(_) => left_assoc(14),
        BinaryOperator::Slash(_) => left_assoc(14),
        BinaryOperator::Percent(_) => left_assoc(14),
        BinaryOperator::Plus(_) => left_assoc(13),
        BinaryOperator::Minus(_) => left_assoc(13),
        BinaryOperator::GtGt(_) => left_assoc(12),
        BinaryOperator::LtLt(_) => left_assoc(12),
        BinaryOperator::GtGtGt(_) => left_assoc(12),
        BinaryOperator::LtLtLt(_) => left_assoc(12),
        BinaryOperator::Lt(_) => left_assoc(11),
        BinaryOperator::LtEq(_) => left_assoc(11),
        BinaryOperator::Gt(_) => left_assoc(11),
        BinaryOperator::GtEq(_) => left_assoc(11),
        BinaryOperator::EqEq(_) => left_assoc(10),
        BinaryOperator::ExclEq(_) => left_assoc(10),
        BinaryOperator::EqEqEq(_) => left_assoc(10),
        BinaryOperator::ExclEqEq(_) => left_assoc(10),
        BinaryOperator::EqEqQuest(_) => left_assoc(10),
        BinaryOperator::ExclEqQuest(_) => left_assoc(10),
        BinaryOperator::Amp(_) => left_assoc(9),
        BinaryOperator::Caret(_) => left_assoc(8),
        BinaryOperator::CaretTilde(_) => left_assoc(8),
        BinaryOperator::TildeCaret(_) => left_assoc(8),
        BinaryOperator::Pipe(_) => left_assoc(7),
        BinaryOperator::AmpAmp(_) => left_assoc(6),
        BinaryOperator::PipePipe(_) => left_assoc(5),
        BinaryOperator::MinusGt(_) => right_assoc(1),
        BinaryOperator::LtMinusGt(_) => right_assoc(1),
    }
}

#[inline(always)]
fn ternary_operator_binding_power<'s>() -> (u8, u8) {
    right_assoc(2)
}

#[inline(always)]
fn amp_amp_amp_operator_binding_power<'s>() -> u8 {
    no_assoc(3)
}

#[inline(always)]
pub(crate) fn matches_operator_binding_power<'s>() -> u8 {
    no_assoc(4)
}

#[inline(always)]
fn inside_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(11)
}

fn constant_expression_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<ConstantExpression<'s>, VerboseError<'s>> {
    let mut lhs = alt((
        constant_primary_parser
            .map(|a| ConstantExpression::Primary(Box::new(a))),
        (
            unary_operator_parser,
            attribute_instance_vec_parser,
            constant_primary_parser,
        )
            .map(|(a, b, c)| ConstantExpression::Unary(Box::new((a, b, c)))),
    ))
    .parse_next(input)?;
    loop {
        let Ok((op, r_bp)) = alt((
            binary_operator_parser.verify_map(|a| {
                let (l_bp, r_bp) = binary_operator_binding_power(&a);
                if l_bp < min_bp {
                    return None;
                }
                Some((ConstantExpressionPrattOp::Binary(a), r_bp))
            }),
            token(Token::Quest).verify_map(|a| {
                let (l_bp, r_bp) = ternary_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((ConstantExpressionPrattOp::Ternary(a), r_bp))
            }),
        ))
        .parse_next(input) else {
            return Ok(lhs);
        };
        lhs = match op {
            ConstantExpressionPrattOp::Binary(binop) => {
                let attrs = attribute_instance_vec_parser(input)?;
                let rhs = constant_expression_bp_parser(input, r_bp)?;
                ConstantExpression::Binary(Box::new((lhs, binop, attrs, rhs)))
            }
            ConstantExpressionPrattOp::Ternary(quest) => {
                let attrs = attribute_instance_vec_parser(input)?;
                let mhs = constant_expression_bp_parser(input, 0)?;
                let colon = token(Token::Colon)(input)?;
                let rhs = constant_expression_bp_parser(input, r_bp)?;
                ConstantExpression::Ternary(Box::new((
                    lhs, quest, attrs, mhs, colon, rhs,
                )))
            }
        }
    }
}

pub fn constant_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantExpression<'s>, VerboseError<'s>> {
    constant_expression_bp_parser(input, 0)
}

pub fn constant_mintypmax_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantMintypmaxExpression<'s>, VerboseError<'s>> {
    alt((
        (
            constant_expression_parser,
            token(Token::Colon),
            constant_expression_parser,
            token(Token::Colon),
            constant_expression_parser,
        )
            .map(|(a, b, c, d, e)| {
                ConstantMintypmaxExpression::Mintypmax(Box::new((
                    a, b, c, d, e,
                )))
            }),
        constant_expression_parser
            .map(|a| ConstantMintypmaxExpression::Single(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn constant_param_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantParamExpression<'s>, VerboseError<'s>> {
    let _mintypmax_parser = constant_mintypmax_expression_parser
        .map(|a| ConstantParamExpression::Mintypmax(Box::new(a)));
    let _data_parser =
        data_type_parser.map(|a| ConstantParamExpression::Data(Box::new(a)));
    let _dollar_parser = token(Token::Dollar)
        .map(|a| ConstantParamExpression::Dollar(Box::new(a)));
    alt((_mintypmax_parser, _data_parser, _dollar_parser)).parse_next(input)
}

pub fn param_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParamExpression<'s>, VerboseError<'s>> {
    let _mintypmax_parser =
        terminated(mintypmax_expression_parser, peek(not(token(Token::Pound))))
            .map(|a| ParamExpression::Mintypmax(Box::new(a)));
    let _data_parser =
        data_type_parser.map(|a| ParamExpression::Data(Box::new(a)));
    let _dollar_parser =
        token(Token::Dollar).map(|a| ParamExpression::Dollar(Box::new(a)));
    alt((_mintypmax_parser, _data_parser, _dollar_parser)).parse_next(input)
}

pub fn constant_range_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantRangeExpression<'s>, VerboseError<'s>> {
    alt((
        constant_expression_parser
            .map(|a| ConstantRangeExpression::Expression(Box::new(a))),
        constant_part_select_range_parser
            .map(|a| ConstantRangeExpression::PartSelectRange(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn constant_part_select_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantPartSelectRange<'s>, VerboseError<'s>> {
    alt((
        constant_range_parser
            .map(|a| ConstantPartSelectRange::Range(Box::new(a))),
        constant_indexed_range_parser
            .map(|a| ConstantPartSelectRange::IndexedRange(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn constant_indexed_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantIndexedRange<'s>, VerboseError<'s>> {
    let _plus_parser = (
        constant_expression_parser,
        token(Token::PlusColon),
        constant_expression_parser,
    )
        .map(|(a, b, c)| ConstantIndexedRange::Plus(Box::new((a, b, c))));
    let _minus_parser = (
        constant_expression_parser,
        token(Token::MinusColon),
        constant_expression_parser,
    )
        .map(|(a, b, c)| ConstantIndexedRange::Minus(Box::new((a, b, c))));
    alt((_plus_parser, _minus_parser)).parse_next(input)
}

pub fn constant_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantRange<'s>, VerboseError<'s>> {
    (
        constant_expression_parser,
        token(Token::Colon),
        constant_expression_parser,
    )
        .map(|(a, b, c)| ConstantRange(a, b, c))
        .parse_next(input)
}

fn gen_constant_expression_parser<'s>(
    min_bp: u8,
) -> impl FnMut(
    &mut Tokens<'s>,
) -> ModalResult<ConstantExpression<'s>, VerboseError<'s>> {
    move |input| constant_expression_bp_parser(input, min_bp)
}

fn pattern_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<Pattern<'s>, VerboseError<'s>> {
    let _parentheses_parser =
        (token(Token::Paren), pattern_parser, token(Token::EParen))
            .map(|(a, b, c)| Pattern::Parentheses(Box::new((a, b, c))));
    let _variable_identifier_parser =
        (token(Token::Period), variable_identifier_parser)
            .map(|(a, b)| Pattern::VariableIdentifier(Box::new((a, b))));
    let _wildcard_parser = (token(Token::Period), token(Token::Star))
        .map(|(a, b)| Pattern::Wildcard(Box::new((a, b))));
    let _constant_expression_parser = gen_constant_expression_parser(min_bp)
        .map(|a| Pattern::ConstantExpression(Box::new(a)));
    let _tagged_member_parser = (
        token(Token::Tagged),
        member_identifier_parser,
        opt_note(pattern_parser),
    )
        .map(|(a, b, c)| Pattern::TaggedMember(Box::new((a, b, c))));
    let _multi_pattern_parser = (
        token(Token::Apost),
        token(Token::Brace),
        pattern_parser,
        repeat_note((token(Token::Apost), pattern_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| {
            Pattern::MultiPattern(Box::new((a, b, c, d, e)))
        });
    let _multi_identifier_pattern_parser = (
        token(Token::Apost),
        token(Token::Brace),
        member_identifier_parser,
        token(Token::Colon),
        pattern_parser,
        repeat_note((
            token(Token::Apost),
            member_identifier_parser,
            token(Token::Colon),
            pattern_parser,
        )),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e, f, g)| {
            Pattern::MultiIdentifierPattern(Box::new((a, b, c, d, e, f, g)))
        });
    alt((
        _parentheses_parser,
        _variable_identifier_parser,
        _wildcard_parser,
        _constant_expression_parser,
        _tagged_member_parser,
        _multi_pattern_parser,
        _multi_identifier_pattern_parser,
    ))
    .parse_next(input)
}

pub(crate) fn gen_pattern_parser<'s>(
    min_bp: u8,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<Pattern<'s>, VerboseError<'s>> {
    move |input| pattern_bp_parser(input, min_bp)
}

fn basic_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Expression<'s>, VerboseError<'s>> {
    let _primary_parser = terminated(
        primary_parser,
        peek(alt((
            token(Token::Comma),
            token(Token::Paren),
            token(Token::EParen),
            token(Token::SColon),
        ))),
    )
    .map(|a| Expression::Primary(Box::new(a)));
    let _unary_parser = (
        unary_operator_parser,
        attribute_instance_vec_parser,
        primary_parser,
    )
        .map(|(a, b, c)| Expression::Unary(Box::new((a, b, c))));
    let _inc_or_dec_expression_parser = inc_or_dec_expression_parser
        .map(|a| Expression::IncOrDecExpression(Box::new(a)));
    let _operator_assignment_parser = (
        token(Token::Paren),
        operator_assignment_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| Expression::OperatorAssignment(Box::new((a, b, c))));
    let _tagged_union_expression_parser = tagged_union_expression_parser
        .map(|a| Expression::TaggedUnionExpression(Box::new(a)));
    let _catchall_primary_parser =
        primary_parser.map(|a| Expression::Primary(Box::new(a)));
    alt((
        _primary_parser,
        _unary_parser,
        _inc_or_dec_expression_parser,
        _operator_assignment_parser,
        _tagged_union_expression_parser,
        _catchall_primary_parser,
    ))
    .parse_next(input)
}

enum ExpressionPrattOp<'a> {
    Binary(BinaryOperator<'a>),
    Ternary(Metadata<'a>),
    Matches(Metadata<'a>),
    AmpAmpAmp(Metadata<'a>),
    Inside(Metadata<'a>),
}

fn expression_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<Expression<'s>, VerboseError<'s>> {
    let mut lhs = basic_expression_parser.parse_next(input)?;
    loop {
        let Ok((op, r_bp)) = alt((
            binary_operator_parser.verify_map(|a| {
                let (l_bp, r_bp) = binary_operator_binding_power(&a);
                if l_bp < min_bp {
                    return None;
                }
                Some((ExpressionPrattOp::Binary(a), r_bp))
            }),
            token(Token::Quest).verify_map(|a| {
                let (l_bp, r_bp) = ternary_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((ExpressionPrattOp::Ternary(a), r_bp))
            }),
            token(Token::Matches).verify_map(|a| {
                let bp = matches_operator_binding_power();
                if bp < min_bp {
                    return None;
                }
                Some((ExpressionPrattOp::Matches(a), bp))
            }),
            token(Token::AmpAmpAmp).verify_map(|a| {
                let bp = amp_amp_amp_operator_binding_power();
                if bp < min_bp {
                    return None;
                }
                Some((ExpressionPrattOp::AmpAmpAmp(a), bp))
            }),
            token(Token::Inside).verify_map(|a| {
                let (l_bp, r_bp) = inside_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((ExpressionPrattOp::Inside(a), r_bp))
            }),
        ))
        .parse_next(input) else {
            return Ok(lhs);
        };
        lhs = match op {
            ExpressionPrattOp::Binary(binop) => {
                let attrs = attribute_instance_vec_parser(input)?;
                let rhs = expression_bp_parser(input, r_bp)?;
                Expression::Binary(Box::new((lhs, binop, attrs, rhs)))
            }
            ExpressionPrattOp::Ternary(quest) => {
                let attrs = attribute_instance_vec_parser(input)?;
                let mhs = expression_bp_parser(input, 0)?;
                let colon = token(Token::Colon)(input)?;
                let rhs = expression_bp_parser(input, r_bp)?;
                Expression::ConditionalExpression(Box::new(
                    ConditionalExpression(
                        CondPredicate(
                            ExpressionOrCondPattern::Expression(Box::new(lhs)),
                            vec![],
                        ),
                        quest,
                        attrs,
                        mhs,
                        colon,
                        rhs,
                    ),
                ))
            }
            ExpressionPrattOp::Matches(matches) => {
                let pattern = pattern_bp_parser(input, r_bp + 1)?;
                let cond_pattern = CondPattern(lhs, matches, pattern);
                let mut cond_predicate: Vec<(
                    Metadata<'s>,
                    ExpressionOrCondPattern<'s>,
                )> = vec![];
                match opt_note(token(Token::AmpAmpAmp)).parse_next(input)? {
                    None => (),
                    Some(mut amp_amp_amp) => loop {
                        let next_expression =
                            expression_bp_parser(input, r_bp + 1)?;
                        let expression_or_cond_patter = match opt_note((
                            token(Token::Matches),
                            gen_pattern_parser(r_bp + 1),
                        ))
                        .parse_next(input)?
                        {
                            Some((matches, pattern)) => {
                                ExpressionOrCondPattern::CondPattern(Box::new(
                                    CondPattern(
                                        next_expression,
                                        matches,
                                        pattern,
                                    ),
                                ))
                            }
                            None => ExpressionOrCondPattern::Expression(
                                Box::new(next_expression),
                            ),
                        };
                        cond_predicate
                            .push((amp_amp_amp, expression_or_cond_patter));
                        match opt_note(token(Token::AmpAmpAmp))
                            .parse_next(input)?
                        {
                            Some(new_amp_amp_amp) => {
                                amp_amp_amp = new_amp_amp_amp
                            }
                            None => {
                                break;
                            }
                        }
                    },
                }
                let quest = token(Token::Quest)(input)?;
                let attrs = attribute_instance_vec_parser(input)?;
                let mhs = expression_bp_parser(input, 0)?;
                let colon = token(Token::Colon)(input)?;
                let (_, ternary_r_bp) = ternary_operator_binding_power();
                let rhs = expression_bp_parser(input, ternary_r_bp)?;
                Expression::ConditionalExpression(Box::new(
                    ConditionalExpression(
                        CondPredicate(
                            ExpressionOrCondPattern::CondPattern(Box::new(
                                cond_pattern,
                            )),
                            cond_predicate,
                        ),
                        quest,
                        attrs,
                        mhs,
                        colon,
                        rhs,
                    ),
                ))
            }
            ExpressionPrattOp::AmpAmpAmp(mut amp_amp_amp) => {
                let mut cond_predicate: Vec<(
                    Metadata<'s>,
                    ExpressionOrCondPattern<'s>,
                )> = vec![];
                loop {
                    let next_expression = expression_bp_parser(
                        input,
                        matches_operator_binding_power() + 1,
                    )?;
                    let expression_or_cond_patter = match opt_note((
                        token(Token::Matches),
                        gen_pattern_parser(
                            matches_operator_binding_power() + 1,
                        ),
                    ))
                    .parse_next(input)?
                    {
                        Some((matches, pattern)) => {
                            ExpressionOrCondPattern::CondPattern(Box::new(
                                CondPattern(next_expression, matches, pattern),
                            ))
                        }
                        None => ExpressionOrCondPattern::Expression(Box::new(
                            next_expression,
                        )),
                    };
                    cond_predicate
                        .push((amp_amp_amp, expression_or_cond_patter));
                    match opt_note(token(Token::AmpAmpAmp)).parse_next(input)? {
                        Some(new_amp_amp_amp) => amp_amp_amp = new_amp_amp_amp,
                        None => {
                            break;
                        }
                    }
                }
                let quest = token(Token::Quest)(input)?;
                let attrs = attribute_instance_vec_parser(input)?;
                let mhs = expression_bp_parser(input, 0)?;
                let colon = token(Token::Colon)(input)?;
                let rhs = expression_bp_parser(input, r_bp)?;
                Expression::ConditionalExpression(Box::new(
                    ConditionalExpression(
                        CondPredicate(
                            ExpressionOrCondPattern::Expression(Box::new(lhs)),
                            cond_predicate,
                        ),
                        quest,
                        attrs,
                        mhs,
                        colon,
                        rhs,
                    ),
                ))
            }
            ExpressionPrattOp::Inside(inside) => {
                let brace = token(Token::Brace)(input)?;
                let range_list = range_list_parser(input)?;
                let ebrace = token(Token::EBrace)(input)?;
                Expression::InsideExpression(Box::new(InsideExpression(
                    lhs, inside, brace, range_list, ebrace,
                )))
            }
        }
    }
}

pub(crate) fn gen_expression_parser<'s>(
    min_bp: u8,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<Expression<'s>, VerboseError<'s>>
{
    move |input| expression_bp_parser(input, min_bp)
}

pub fn expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Expression<'s>, VerboseError<'s>> {
    expression_bp_parser(input, 0)
}

pub fn tagged_union_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TaggedUnionExpression<'s>, VerboseError<'s>> {
    (
        token(Token::Tagged),
        member_identifier_parser,
        opt_note(primary_parser),
    )
        .map(|(a, b, c)| TaggedUnionExpression(a, b, c))
        .parse_next(input)
}

#[allow(dead_code)] // Parsed in Pratt parsing
pub fn inside_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InsideExpression<'s>, VerboseError<'s>> {
    (
        expression_parser,
        token(Token::Inside),
        token(Token::Brace),
        range_list_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| InsideExpression(a, b, c, d, e))
        .parse_next(input)
}

pub fn mintypmax_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MintypmaxExpression<'s>, VerboseError<'s>> {
    alt((
        (
            expression_parser,
            token(Token::Colon),
            expression_parser,
            token(Token::Colon),
            expression_parser,
        )
            .map(|(a, b, c, d, e)| {
                MintypmaxExpression::Mintypmax(Box::new((a, b, c, d, e)))
            }),
        expression_parser.map(|a| MintypmaxExpression::Single(Box::new(a))),
    ))
    .parse_next(input)
}

#[allow(dead_code)] // Parsed in Pratt parsing
pub fn module_path_conditional_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathConditionalExpression<'s>, VerboseError<'s>> {
    (
        module_path_expression_parser,
        token(Token::Quest),
        attribute_instance_vec_parser,
        module_path_expression_parser,
        token(Token::Colon),
        module_path_expression_parser,
    )
        .map(|(a, b, c, d, e, f)| {
            ModulePathConditionalExpression(a, b, c, d, e, f)
        })
        .parse_next(input)
}

#[inline]
fn binary_module_path_operator_binding_power<'s>(
    binop: &BinaryModulePathOperator<'s>,
) -> (u8, u8) {
    match binop {
        BinaryModulePathOperator::EqEq(_) => left_assoc(10),
        BinaryModulePathOperator::ExclEq(_) => left_assoc(10),
        BinaryModulePathOperator::Amp(_) => left_assoc(9),
        BinaryModulePathOperator::Caret(_) => left_assoc(8),
        BinaryModulePathOperator::CaretTilde(_) => left_assoc(8),
        BinaryModulePathOperator::TildeCaret(_) => left_assoc(8),
        BinaryModulePathOperator::Pipe(_) => left_assoc(7),
        BinaryModulePathOperator::AmpAmp(_) => left_assoc(6),
        BinaryModulePathOperator::PipePipe(_) => left_assoc(5),
    }
}

enum BinaryModulePathOrTernaryOp<'a> {
    Binary(BinaryModulePathOperator<'a>),
    Ternary(Metadata<'a>),
}

fn module_path_expression_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<ModulePathExpression<'s>, VerboseError<'s>> {
    let mut lhs = alt((
        module_path_primary_parser
            .map(|a| ModulePathExpression::Primary(Box::new(a))),
        (
            unary_module_path_operator_parser,
            attribute_instance_vec_parser,
            module_path_primary_parser,
        )
            .map(|(a, b, c)| ModulePathExpression::Unary(Box::new((a, b, c)))),
    ))
    .parse_next(input)?;
    loop {
        let Ok((op, r_bp)) = alt((
            binary_module_path_operator_parser.verify_map(|a| {
                let (l_bp, r_bp) =
                    binary_module_path_operator_binding_power(&a);
                if l_bp < min_bp {
                    return None;
                }
                Some((BinaryModulePathOrTernaryOp::Binary(a), r_bp))
            }),
            token(Token::Quest).verify_map(|a| {
                let (l_bp, r_bp) = ternary_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((BinaryModulePathOrTernaryOp::Ternary(a), r_bp))
            }),
        ))
        .parse_next(input) else {
            return Ok(lhs);
        };
        lhs = match op {
            BinaryModulePathOrTernaryOp::Binary(binop) => {
                let attrs = attribute_instance_vec_parser(input)?;
                let rhs = module_path_expression_bp_parser(input, r_bp)?;
                ModulePathExpression::Binary(Box::new((lhs, binop, attrs, rhs)))
            }
            BinaryModulePathOrTernaryOp::Ternary(quest) => {
                let attrs = attribute_instance_vec_parser(input)?;
                let mhs = module_path_expression_bp_parser(input, 0)?;
                let colon = token(Token::Colon)(input)?;
                let rhs = module_path_expression_bp_parser(input, r_bp)?;
                ModulePathExpression::Conditional(Box::new(
                    ModulePathConditionalExpression(
                        lhs, quest, attrs, mhs, colon, rhs,
                    ),
                ))
            }
        }
    }
}

pub fn module_path_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathExpression<'s>, VerboseError<'s>> {
    module_path_expression_bp_parser(input, 0)
}

pub fn module_path_mintypmax_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathMintypmaxExpression<'s>, VerboseError<'s>> {
    alt((
        (
            module_path_expression_parser,
            token(Token::Colon),
            module_path_expression_parser,
            token(Token::Colon),
            module_path_expression_parser,
        )
            .map(|(a, b, c, d, e)| {
                ModulePathMintypmaxExpression::Mintypmax(Box::new((
                    a, b, c, d, e,
                )))
            }),
        module_path_expression_parser
            .map(|a| ModulePathMintypmaxExpression::Single(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn part_select_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PartSelectRange<'s>, VerboseError<'s>> {
    alt((
        constant_range_parser
            .map(|a| PartSelectRange::ConstantRange(Box::new(a))),
        indexed_range_parser
            .map(|a| PartSelectRange::IndexedRange(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn indexed_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IndexedRange<'s>, VerboseError<'s>> {
    alt((
        (
            expression_parser,
            token(Token::PlusColon),
            constant_expression_parser,
        )
            .map(|(a, b, c)| IndexedRange::Plus(Box::new((a, b, c)))),
        (
            expression_parser,
            token(Token::MinusColon),
            constant_expression_parser,
        )
            .map(|(a, b, c)| IndexedRange::Minus(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn genvar_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenvarExpression<'s>, VerboseError<'s>> {
    constant_expression_parser
        .map(|a| GenvarExpression(a))
        .parse_next(input)
}
