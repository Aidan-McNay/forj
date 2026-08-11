// =======================================================================
// covergroup_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.11

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};

pub fn covergroup_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupDeclaration<'s>, VerboseError<'s>> {
    let _initial_parser = (
        token(Token::Covergroup),
        covergroup_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(tf_port_list_parser),
            token(Token::EParen),
        )),
        opt_note(coverage_event_parser),
        token(Token::SColon),
        repeat_note(coverage_spec_or_option_parser),
        token(Token::Endgroup),
        opt_note((token(Token::Colon), covergroup_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            CovergroupDeclaration::Initial(Box::new((a, b, c, d, e, f, g, h)))
        });
    let _extends_parser = (
        token(Token::Covergroup),
        token(Token::Extends),
        covergroup_identifier_parser,
        token(Token::SColon),
        repeat_note(coverage_spec_or_option_parser),
        token(Token::Endgroup),
        opt_note((token(Token::Colon), covergroup_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g)| {
            CovergroupDeclaration::Extends(Box::new((a, b, c, d, e, f, g)))
        });
    alt((_initial_parser, _extends_parser)).parse_next(input)
}

pub fn coverage_spec_or_option_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverageSpecOrOption<'s>, VerboseError<'s>> {
    alt((
        (attribute_instance_vec_parser, coverage_spec_parser)
            .map(|(a, b)| CoverageSpecOrOption::Spec(Box::new((a, b)))),
        (
            attribute_instance_vec_parser,
            coverage_option_parser,
            token(Token::SColon),
        )
            .map(|(a, b, c)| CoverageSpecOrOption::Option(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn coverage_option_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverageOption<'s>, VerboseError<'s>> {
    let _option_parser = (
        token(Token::Option),
        token(Token::Period),
        member_identifier_parser,
        token(Token::Eq),
        expression_parser,
    )
        .map(|(a, b, c, d, e)| {
            CoverageOption::Option(Box::new((a, b, c, d, e)))
        });
    let _type_option_parser = (
        token(Token::TypeOption),
        token(Token::Period),
        member_identifier_parser,
        token(Token::Eq),
        constant_expression_parser,
    )
        .map(|(a, b, c, d, e)| {
            CoverageOption::TypeOption(Box::new((a, b, c, d, e)))
        });
    alt((_option_parser, _type_option_parser)).parse_next(input)
}

pub fn coverage_spec_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverageSpec<'s>, VerboseError<'s>> {
    alt((
        cover_point_parser.map(|a| CoverageSpec::Point(Box::new(a))),
        cover_cross_parser.map(|a| CoverageSpec::Cross(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn coverage_event_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverageEvent<'s>, VerboseError<'s>> {
    let _clocking_parser =
        clocking_event_parser.map(|a| CoverageEvent::Clocking(Box::new(a)));
    let _function_parser = (
        token(Token::With),
        token(Token::Function),
        token(Token::Sample),
        token(Token::Paren),
        opt_note(tf_port_list_parser),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| {
            CoverageEvent::Function(Box::new((a, b, c, d, e, f)))
        });
    let _block_parser = (
        token(Token::AtAt),
        token(Token::Paren),
        block_event_expression_parser,
        token(Token::Paren),
    )
        .map(|(a, b, c, d)| CoverageEvent::Block(Box::new((a, b, c, d))));
    alt((_function_parser, _block_parser, _clocking_parser)).parse_next(input)
}

fn basic_block_event_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BlockEventExpression<'s>, VerboseError<'s>> {
    let _begin_parser =
        (token(Token::Begin), hierarchical_btf_identifier_parser)
            .map(|(a, b)| BlockEventExpression::Begin(Box::new((a, b))));
    let _end_parser = (token(Token::End), hierarchical_btf_identifier_parser)
        .map(|(a, b)| BlockEventExpression::End(Box::new((a, b))));
    alt((_begin_parser, _end_parser)).parse_next(input)
}

pub fn block_event_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BlockEventExpression<'s>, VerboseError<'s>> {
    let mut lhs = basic_block_event_expression_parser(input)?;
    loop {
        let Ok(op) = token(Token::Or)(input) else {
            return Ok(lhs);
        };
        lhs = {
            let rhs = basic_block_event_expression_parser(input)?;
            BlockEventExpression::Or(Box::new((lhs, op, rhs)))
        }
    }
}

pub fn hierarchical_identifier_or_class_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalIdentifierOrClassScope<'s>, VerboseError<'s>> {
    alt((
        (hierarchical_identifier_parser, token(Token::Period)).map(|(a, b)| {
            HierarchicalIdentifierOrClassScope::Identifier(Box::new((a, b)))
        }),
        class_scope_parser
            .map(|a| HierarchicalIdentifierOrClassScope::Scope(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn hierarchical_btf_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalBtfIdentifier<'s>, VerboseError<'s>> {
    alt((
        (
            opt_note(hierarchical_identifier_or_class_scope_parser),
            method_identifier_parser,
        )
            .map(|(a, b)| HierarchicalBtfIdentifier::Method(Box::new((a, b)))),
        hierarchical_tf_identifier_parser
            .map(|a| HierarchicalBtfIdentifier::Tf(Box::new(a))),
        hierarchical_block_identifier_parser
            .map(|a| HierarchicalBtfIdentifier::Block(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn cover_point_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverPoint<'s>, VerboseError<'s>> {
    (
        opt_note((
            opt_note(data_type_or_implicit_parser_cover_point),
            cover_point_identifier_parser,
            token(Token::Colon),
        )),
        token(Token::Coverpoint),
        expression_parser,
        opt_note((
            token(Token::Iff),
            token(Token::Paren),
            expression_parser,
            token(Token::EParen),
        )),
        bins_or_empty_parser,
    )
        .map(|(a, b, c, d, e)| CoverPoint(a, b, c, d, e))
        .parse_next(input)
}

fn data_type_or_implicit_parser_cover_point<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataTypeOrImplicit<'s>, VerboseError<'s>> {
    alt((
        terminated(data_type_parser, peek(cover_point_identifier_parser))
            .map(|a| DataTypeOrImplicit::DataType(a)),
        terminated(
            implicit_data_type_parser,
            peek(cover_point_identifier_parser),
        )
        .map(|a| DataTypeOrImplicit::ImplicitDataType(a)),
    ))
    .parse_next(input)
}

pub fn bins_or_empty_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinsOrEmpty<'s>, VerboseError<'s>> {
    let _bins_parser = (
        token(Token::Brace),
        attribute_instance_vec_parser,
        repeat_note((bins_or_options_parser, token(Token::SColon))),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| BinsOrEmpty::Bins(Box::new((a, b, c, d))));
    alt((
        _bins_parser,
        token(Token::SColon).map(|a| BinsOrEmpty::Empty(Box::new(a))),
    ))
    .parse_next(input)
}

enum BinsOrOptionsOp<'a> {
    Range(
        (
            Metadata<'a>, // {
            CovergroupRangeList<'a>,
            Metadata<'a>, // }
        ),
    ),
    Point(CoverPointIdentifier<'a>),
    Set(SetCovergroupExpression<'a>),
    Trans(TransList<'a>),
    Default(Metadata<'a>),
    DefaultSequence(
        (
            Metadata<'a>, // default
            Metadata<'a>, // sequence
        ),
    ),
}

pub fn bins_or_options_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinsOrOptions<'s>, VerboseError<'s>> {
    let _bins_parser = (
        opt_note(token(Token::Wildcard)),
        bins_keyword_parser,
        bin_identifier_parser,
        opt_note((
            token(Token::Bracket),
            opt_note(covergroup_expression_parser),
            token(Token::EBrace),
        )),
        token(Token::Eq),
        alt((
            (
                token(Token::Brace),
                covergroup_range_list_parser,
                token(Token::EBrace),
            )
                .map(|(a, b, c)| BinsOrOptionsOp::Range((a, b, c))),
            cover_point_identifier_parser.map(|a| BinsOrOptionsOp::Point(a)),
            set_covergroup_expression_parser.map(|a| BinsOrOptionsOp::Set(a)),
            trans_list_parser.map(|a| BinsOrOptionsOp::Trans(a)),
            (token(Token::Default), token(Token::Sequence))
                .map(|(a, b)| BinsOrOptionsOp::DefaultSequence((a, b))),
            token(Token::Default).map(|a| BinsOrOptionsOp::Default(a)),
        )),
        opt_note((
            token(Token::With),
            token(Token::Paren),
            with_covergroup_expression_parser,
            token(Token::EParen),
        )),
        opt_note((
            token(Token::Iff),
            token(Token::Paren),
            expression_parser,
            token(Token::EParen),
        )),
    )
        .verify_map(
            |(
                wildcard,
                bins_keyword,
                bin_identifier,
                covergroup,
                eq,
                op,
                with,
                iff,
            )| match (wildcard, covergroup, op, with) {
                (
                    wildcard,
                    covergroup,
                    BinsOrOptionsOp::Range((brace, range, ebrace)),
                    with,
                ) => Some(BinsOrOptions::Range(Box::new((
                    wildcard,
                    bins_keyword,
                    bin_identifier,
                    covergroup,
                    eq,
                    brace,
                    range,
                    ebrace,
                    with,
                    iff,
                )))),
                (
                    wildcard,
                    covergroup,
                    BinsOrOptionsOp::Point(coverpoint),
                    Some((with, paren, with_expr, eparen)),
                ) => Some(BinsOrOptions::Point(Box::new((
                    wildcard,
                    bins_keyword,
                    bin_identifier,
                    covergroup,
                    eq,
                    coverpoint,
                    with,
                    paren,
                    with_expr,
                    eparen,
                    iff,
                )))),
                (wildcard, covergroup, BinsOrOptionsOp::Set(set), None) => {
                    Some(BinsOrOptions::Set(Box::new((
                        wildcard,
                        bins_keyword,
                        bin_identifier,
                        covergroup,
                        eq,
                        set,
                        iff,
                    ))))
                }
                (
                    wildcard,
                    Some((bracket, None, ebracket)),
                    BinsOrOptionsOp::Trans(trans_list),
                    None,
                ) => Some(BinsOrOptions::Trans(Box::new((
                    wildcard,
                    bins_keyword,
                    bin_identifier,
                    Some((bracket, ebracket)),
                    eq,
                    trans_list,
                    iff,
                )))),
                (wildcard, None, BinsOrOptionsOp::Trans(trans_list), None) => {
                    Some(BinsOrOptions::Trans(Box::new((
                        wildcard,
                        bins_keyword,
                        bin_identifier,
                        None,
                        eq,
                        trans_list,
                        iff,
                    ))))
                }
                (None, covergroup, BinsOrOptionsOp::Default(default), None) => {
                    Some(BinsOrOptions::Default(Box::new((
                        bins_keyword,
                        bin_identifier,
                        covergroup,
                        eq,
                        default,
                        iff,
                    ))))
                }
                (
                    None,
                    None,
                    BinsOrOptionsOp::DefaultSequence((default, sequence)),
                    None,
                ) => Some(BinsOrOptions::DefaultSequence(Box::new((
                    bins_keyword,
                    bin_identifier,
                    eq,
                    default,
                    sequence,
                    iff,
                )))),
                _ => None,
            },
        );
    alt((
        _bins_parser,
        coverage_option_parser.map(|a| BinsOrOptions::Coverage(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn bins_keyword_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinsKeyword<'s>, VerboseError<'s>> {
    alt((
        token(Token::Bins).map(|a| BinsKeyword::Bins(a)),
        token(Token::IllegalBins).map(|a| BinsKeyword::IllegalBins(a)),
        token(Token::IgnoreBins).map(|a| BinsKeyword::IgnoreBins(a)),
    ))
    .parse_next(input)
}

pub fn trans_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TransList<'s>, VerboseError<'s>> {
    (
        token(Token::Paren),
        trans_set_parser,
        token(Token::EParen),
        repeat_note((
            token(Token::Comma),
            token(Token::Paren),
            trans_set_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d)| TransList(a, b, c, d))
        .parse_next(input)
}

pub fn trans_set_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TransSet<'s>, VerboseError<'s>> {
    (
        trans_range_list_parser,
        repeat_note((token(Token::EqGt), trans_range_list_parser)),
    )
        .map(|(a, b)| TransSet(a, b))
        .parse_next(input)
}

enum TransRangeListOp<'a> {
    Repeat(Metadata<'a>),
    GotoRepeat(Metadata<'a>),
    NonconsecutiveRepeat(Metadata<'a>),
}

pub fn trans_range_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TransRangeList<'s>, VerboseError<'s>> {
    (
        trans_item_parser,
        opt_note((
            token(Token::Bracket),
            alt((
                token(Token::Star).map(|a| TransRangeListOp::Repeat(a)),
                token(Token::MinusGt).map(|a| TransRangeListOp::GotoRepeat(a)),
                token(Token::Eq)
                    .map(|a| TransRangeListOp::NonconsecutiveRepeat(a)),
            )),
            repeat_range_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| match b {
            None => TransRangeList::NoRepeat(Box::new(a)),
            Some((
                bracket,
                TransRangeListOp::Repeat(metadata),
                range,
                ebracket,
            )) => TransRangeList::Repeat(Box::new((
                a, bracket, metadata, range, ebracket,
            ))),
            Some((
                bracket,
                TransRangeListOp::GotoRepeat(metadata),
                range,
                ebracket,
            )) => TransRangeList::GotoRepeat(Box::new((
                a, bracket, metadata, range, ebracket,
            ))),
            Some((
                bracket,
                TransRangeListOp::NonconsecutiveRepeat(metadata),
                range,
                ebracket,
            )) => TransRangeList::NonconsecutiveRepeat(Box::new((
                a, bracket, metadata, range, ebracket,
            ))),
        })
        .parse_next(input)
}

pub fn trans_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TransItem<'s>, VerboseError<'s>> {
    covergroup_range_list_parser
        .map(|a| TransItem(a))
        .parse_next(input)
}

pub fn repeat_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RepeatRange<'s>, VerboseError<'s>> {
    (
        covergroup_expression_parser,
        opt_note((token(Token::Colon), covergroup_expression_parser)),
    )
        .map(|(a, b)| match b {
            None => RepeatRange::Expr(Box::new(a)),
            Some((b, c)) => RepeatRange::Range(Box::new((a, b, c))),
        })
        .parse_next(input)
}

pub fn cover_cross_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverCross<'s>, VerboseError<'s>> {
    (
        opt_note((cross_identifier_parser, token(Token::Colon))),
        token(Token::Cross),
        list_of_cross_items_parser,
        opt_note((
            token(Token::Iff),
            token(Token::Paren),
            expression_parser,
            token(Token::EParen),
        )),
        cross_body_parser,
    )
        .map(|(a, b, c, d, e)| CoverCross(a, b, c, d, e))
        .parse_next(input)
}

pub fn list_of_cross_items_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfCrossItems<'s>, VerboseError<'s>> {
    (
        cross_item_parser,
        token(Token::Comma),
        cross_item_parser,
        repeat_note((token(Token::Comma), cross_item_parser)),
    )
        .map(|(a, b, c, d)| ListOfCrossItems(a, b, c, d))
        .parse_next(input)
}

pub fn cross_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CrossItem<'s>, VerboseError<'s>> {
    alt((
        cover_point_identifier_parser
            .map(|a| CrossItem::CoverPoint(Box::new(a))),
        variable_identifier_parser.map(|a| CrossItem::Variable(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn cross_body_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CrossBody<'s>, VerboseError<'s>> {
    let _items_parser = (
        token(Token::Brace),
        repeat_note(cross_body_item_parser),
        token(Token::EBrace),
    )
        .map(|(a, b, c)| CrossBody::Items(Box::new((a, b, c))));
    let _null_parser =
        token(Token::SColon).map(|a| CrossBody::Null(Box::new(a)));
    alt((_items_parser, _null_parser)).parse_next(input)
}

pub fn cross_body_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CrossBodyItem<'s>, VerboseError<'s>> {
    alt((
        function_declaration_parser
            .map(|a| CrossBodyItem::Function(Box::new(a))),
        (bins_selection_or_option_parser, token(Token::SColon))
            .map(|(a, b)| CrossBodyItem::BinsSelection(Box::new((a, b)))),
    ))
    .parse_next(input)
}

pub fn bins_selection_or_option_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinsSelectionOrOption<'s>, VerboseError<'s>> {
    let _coverage_parser =
        (attribute_instance_vec_parser, coverage_option_parser)
            .map(|(a, b)| BinsSelectionOrOption::Coverage(Box::new((a, b))));
    let _bins_parser = (attribute_instance_vec_parser, bins_selection_parser)
        .map(|(a, b)| BinsSelectionOrOption::Bins(Box::new((a, b))));
    alt((_coverage_parser, _bins_parser)).parse_next(input)
}

pub fn bins_selection_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinsSelection<'s>, VerboseError<'s>> {
    (
        bins_keyword_parser,
        bin_identifier_parser,
        token(Token::Eq),
        select_expression_parser,
        opt_note((
            token(Token::Iff),
            token(Token::Paren),
            expression_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d, e)| BinsSelection(a, b, c, d, e))
        .parse_next(input)
}

#[inline(always)]
fn covergroup_matches_operator_binding_power<'s>() -> u8 {
    no_assoc(3)
}

#[inline(always)]
fn amp_amp_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(2)
}

#[inline(always)]
fn pipe_pipe_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(1)
}

enum SelectExpressionOp<'a> {
    AmpAmp(Metadata<'a>),
    PipePipe(Metadata<'a>),
    With(Metadata<'a>),
}

fn basic_select_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SelectExpression<'s>, VerboseError<'s>> {
    let _condition_parser = select_condition_parser
        .map(|a| SelectExpression::Condition(Box::new(a)));
    let _not_parser = (token(Token::Exclamation), select_condition_parser)
        .map(|(a, b)| SelectExpression::Not(Box::new((a, b))));
    let _paren_parser = (
        token(Token::Paren),
        select_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| SelectExpression::Paren(Box::new((a, b, c))));
    let _cross_identifier_parser = cross_identifier_parser
        .map(|a| SelectExpression::CrossIdentifier(Box::new(a)));
    let _cross_set_parser = (
        cross_set_expression_parser,
        opt_note((token(Token::Matches), integer_covergroup_expression_parser)),
    )
        .map(|(a, b)| SelectExpression::CrossSet(Box::new((a, b))));
    alt((
        _condition_parser,
        _not_parser,
        _paren_parser,
        _cross_set_parser,
        _cross_identifier_parser,
    ))
    .parse_next(input)
}

fn select_expression_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<SelectExpression<'s>, VerboseError<'s>> {
    let mut lhs = basic_select_expression_parser(input)?;
    loop {
        let Ok((op, r_bp)) = alt((
            token(Token::AmpAmp).verify_map(|a| {
                let (l_bp, r_bp) = amp_amp_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SelectExpressionOp::AmpAmp(a), r_bp))
            }),
            token(Token::PipePipe).verify_map(|a| {
                let (l_bp, r_bp) = pipe_pipe_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SelectExpressionOp::PipePipe(a), r_bp))
            }),
            token(Token::With).verify_map(|a| {
                let bp = covergroup_matches_operator_binding_power();
                if bp < min_bp {
                    return None;
                }
                Some((SelectExpressionOp::With(a), 0))
            }),
        ))
        .parse_next(input) else {
            return Ok(lhs);
        };
        lhs = match op {
            SelectExpressionOp::AmpAmp(metadata) => {
                let rhs = select_expression_bp_parser(input, r_bp)?;
                SelectExpression::And(Box::new((lhs, metadata, rhs)))
            }
            SelectExpressionOp::PipePipe(metadata) => {
                let rhs = select_expression_bp_parser(input, r_bp)?;
                SelectExpression::Or(Box::new((lhs, metadata, rhs)))
            }
            SelectExpressionOp::With(metadata) => {
                let paren = token(Token::Paren)(input)?;
                let with_covergroup_expression =
                    with_covergroup_expression_parser(input)?;
                let eparen = token(Token::EParen)(input)?;
                let matches_section = opt_note((
                    token(Token::Matches),
                    integer_covergroup_expression_parser,
                ))
                .parse_next(input)?;
                SelectExpression::With(Box::new((
                    lhs,
                    metadata,
                    paren,
                    with_covergroup_expression,
                    eparen,
                    matches_section,
                )))
            }
        }
    }
}

pub fn select_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SelectExpression<'s>, VerboseError<'s>> {
    select_expression_bp_parser(input, 0)
}

pub fn select_condition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SelectCondition<'s>, VerboseError<'s>> {
    (
        token(Token::Binsof),
        token(Token::Paren),
        bins_expression_parser,
        token(Token::EParen),
        opt_note((
            token(Token::Intersect),
            token(Token::Brace),
            covergroup_range_list_parser,
            token(Token::EBrace),
        )),
    )
        .map(|(a, b, c, d, e)| SelectCondition(a, b, c, d, e))
        .parse_next(input)
}

pub fn bins_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinsExpression<'s>, VerboseError<'s>> {
    alt((
        (
            cover_point_identifier_parser,
            opt_note((token(Token::Period), bin_identifier_parser)),
        )
            .map(|(a, b)| BinsExpression::CoverPoint(Box::new((a, b)))),
        variable_identifier_parser
            .map(|a| BinsExpression::Variable(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn covergroup_range_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupRangeList<'s>, VerboseError<'s>> {
    (
        covergroup_value_range_parser,
        repeat_note((token(Token::Comma), covergroup_value_range_parser)),
    )
        .map(|(a, b)| CovergroupRangeList(a, b))
        .parse_next(input)
}

enum CovergroupExpressionRangeOp<'a> {
    Colon(Metadata<'a>),
    AbsoluteTolerance(Metadata<'a>),
    RelativeTolerance(Metadata<'a>),
}

pub fn covergroup_value_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupValueRange<'s>, VerboseError<'s>> {
    let _two_expression_range_parser = (
        token(Token::Bracket),
        covergroup_expression_parser,
        alt((
            token(Token::Colon).map(|a| CovergroupExpressionRangeOp::Colon(a)),
            token(Token::PlusSlashMinus)
                .map(|a| CovergroupExpressionRangeOp::AbsoluteTolerance(a)),
            token(Token::PlusPercentMinus)
                .map(|a| CovergroupExpressionRangeOp::RelativeTolerance(a)),
        )),
        covergroup_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| match c {
            CovergroupExpressionRangeOp::Colon(metadata) => {
                CovergroupValueRange::ExprRange(Box::new((
                    a, b, metadata, d, e,
                )))
            }
            CovergroupExpressionRangeOp::AbsoluteTolerance(metadata) => {
                CovergroupValueRange::AbsoluteTolerance(Box::new((
                    a, b, metadata, d, e,
                )))
            }
            CovergroupExpressionRangeOp::RelativeTolerance(metadata) => {
                CovergroupValueRange::RelativeTolerance(Box::new((
                    a, b, metadata, d, e,
                )))
            }
        });
    let _dollar_low_parser = (
        token(Token::Bracket),
        token(Token::Dollar),
        token(Token::Colon),
        covergroup_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            CovergroupValueRange::DollarLow(Box::new((a, b, c, d, e)))
        });
    let _dollar_high_parser = (
        token(Token::Bracket),
        covergroup_expression_parser,
        token(Token::Colon),
        token(Token::Dollar),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            CovergroupValueRange::DollarHigh(Box::new((a, b, c, d, e)))
        });
    let _expr_parser = covergroup_expression_parser
        .map(|a| CovergroupValueRange::Expr(Box::new(a)));
    alt((
        _dollar_low_parser,
        _two_expression_range_parser,
        _dollar_high_parser,
        _expr_parser,
    ))
    .parse_next(input)
}

pub fn with_covergroup_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<WithCovergroupExpression<'s>, VerboseError<'s>> {
    covergroup_expression_parser
        .map(|a| WithCovergroupExpression(a))
        .parse_next(input)
}

pub fn integer_covergroup_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IntegerCovergroupExpression<'s>, VerboseError<'s>> {
    alt((
        covergroup_expression_parser
            .map(|a| IntegerCovergroupExpression::Expression(Box::new(a))),
        token(Token::Dollar)
            .map(|a| IntegerCovergroupExpression::Dollar(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn set_covergroup_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SetCovergroupExpression<'s>, VerboseError<'s>> {
    covergroup_expression_parser
        .map(|a| SetCovergroupExpression(a))
        .parse_next(input)
}

pub fn cross_set_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CrossSetExpression<'s>, VerboseError<'s>> {
    covergroup_expression_parser
        .map(|a| CrossSetExpression(a))
        .parse_next(input)
}

pub fn covergroup_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupExpression<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| CovergroupExpression(a))
        .parse_next(input)
}
