// =======================================================================
// assertion_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.10

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};
use winnow::stream::Stream;

pub fn concurrent_assertion_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConcurrentAssertionItem<'s>, VerboseError<'s>> {
    let _assertion_parser = (
        opt_note((block_identifier_parser, token(Token::Colon))),
        concurrent_assertion_statement_parser,
    )
        .map(|(a, b)| ConcurrentAssertionItem::Assertion(Box::new((a, b))));
    let _checker_parser = checker_instantiation_parser
        .map(|a| ConcurrentAssertionItem::Checker(Box::new(a)));
    alt((_assertion_parser, _checker_parser)).parse_next(input)
}

pub fn concurrent_assertion_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConcurrentAssertionStatement<'s>, VerboseError<'s>> {
    alt((
        assert_property_statement_parser
            .map(|a| ConcurrentAssertionStatement::AssertProp(Box::new(a))),
        assume_property_statement_parser
            .map(|a| ConcurrentAssertionStatement::AssumeProp(Box::new(a))),
        cover_property_statement_parser
            .map(|a| ConcurrentAssertionStatement::CoverProp(Box::new(a))),
        cover_sequence_statement_parser
            .map(|a| ConcurrentAssertionStatement::CoverSeq(Box::new(a))),
        restrict_property_statement_parser
            .map(|a| ConcurrentAssertionStatement::RestrictProp(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn assert_property_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssertPropertyStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Assert),
        token(Token::Property),
        token(Token::Paren),
        property_spec_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e, f)| AssertPropertyStatement(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn assume_property_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssumePropertyStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Assume),
        token(Token::Property),
        token(Token::Paren),
        property_spec_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e, f)| AssumePropertyStatement(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn cover_property_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverPropertyStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Cover),
        token(Token::Property),
        token(Token::Paren),
        property_spec_parser,
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e, f)| CoverPropertyStatement(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn expect_property_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ExpectPropertyStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Expect),
        token(Token::Paren),
        property_spec_parser,
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e)| ExpectPropertyStatement(a, b, c, d, e))
        .parse_next(input)
}

pub fn cover_sequence_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverSequenceStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Cover),
        token(Token::Sequence),
        token(Token::Paren),
        opt_note(clocking_event_parser),
        opt_note((
            token(Token::Disable),
            token(Token::Iff),
            token(Token::Paren),
            expression_or_dist_parser,
            token(Token::EParen),
        )),
        sequence_expr_parser,
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            CoverSequenceStatement(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn restrict_property_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RestrictPropertyStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Restrict),
        token(Token::Property),
        token(Token::Paren),
        property_spec_parser,
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| RestrictPropertyStatement(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn property_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyInstance<'s>, VerboseError<'s>> {
    (
        ps_or_hierarchical_property_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(property_list_of_arguments_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b)| PropertyInstance(a, b))
        .parse_next(input)
}

pub fn property_list_of_arguments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyListOfArguments<'s>, VerboseError<'s>> {
    let _partial_identifier_parser = (
        opt_note(property_actual_arg_parser),
        repeat_note((
            token(Token::Comma),
            opt_note(property_actual_arg_parser),
        )),
        repeat_note((
            token(Token::Comma),
            token(Token::Period),
            identifier_parser,
            token(Token::Paren),
            opt_note(property_actual_arg_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c)| {
            PropertyListOfArguments::PartialIdentifier(Box::new((a, b, c)))
        });
    let _identifier_parser = (
        token(Token::Period),
        identifier_parser,
        token(Token::Paren),
        opt_note(property_actual_arg_parser),
        token(Token::EParen),
        repeat_note((
            token(Token::Comma),
            token(Token::Period),
            identifier_parser,
            token(Token::Paren),
            opt_note(property_actual_arg_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d, e, f)| {
            PropertyListOfArguments::Identifier(Box::new((a, b, c, d, e, f)))
        });
    alt((_partial_identifier_parser, _identifier_parser)).parse_next(input)
}

pub fn property_actual_arg_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyActualArg<'s>, VerboseError<'s>> {
    alt((
        property_expr_parser.map(|a| PropertyActualArg::Property(Box::new(a))),
        sequence_actual_arg_parser
            .map(|a| PropertyActualArg::Sequence(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn assertion_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssertionItemDeclaration<'s>, VerboseError<'s>> {
    alt((
        property_declaration_parser
            .map(|a| AssertionItemDeclaration::Property(Box::new(a))),
        sequence_declaration_parser
            .map(|a| AssertionItemDeclaration::Sequence(Box::new(a))),
        let_declaration_parser
            .map(|a| AssertionItemDeclaration::Let(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn property_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Property),
        property_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(property_port_list_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
        repeat_note(assertion_variable_declaration_parser),
        property_spec_parser,
        opt_note(token(Token::SColon)),
        token(Token::Endproperty),
        opt_note((token(Token::Colon), property_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i)| {
            PropertyDeclaration(a, b, c, d, e, f, g, h, i)
        })
        .parse_next(input)
}

pub fn property_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyPortList<'s>, VerboseError<'s>> {
    (
        property_port_item_parser,
        repeat_note((token(Token::Comma), property_port_item_parser)),
    )
        .map(|(a, b)| PropertyPortList(a, b))
        .parse_next(input)
}

pub fn property_port_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyPortItem<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        opt_note((
            token(Token::Local),
            opt_note(property_lvar_port_direction_parser),
        )),
        property_formal_type_parser,
        formal_port_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), property_actual_arg_parser)),
    )
        .map(|(a, b, c, d, e, f)| PropertyPortItem(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn property_lvar_port_direction_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyLvarPortDirection<'s>, VerboseError<'s>> {
    token(Token::Input)
        .map(|a| PropertyLvarPortDirection(a))
        .parse_next(input)
}

pub fn property_formal_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyFormalType<'s>, VerboseError<'s>> {
    alt((
        token(Token::Property)
            .map(|a| PropertyFormalType::Property(Box::new(a))),
        sequence_formal_type_parser
            .map(|a| PropertyFormalType::Sequence(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn property_spec_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertySpec<'s>, VerboseError<'s>> {
    (
        opt_note(clocking_event_parser),
        opt_note((
            token(Token::Disable),
            token(Token::Iff),
            token(Token::Paren),
            expression_or_dist_parser,
            token(Token::EParen),
        )),
        property_expr_parser,
    )
        .map(|(a, b, c)| PropertySpec(a, b, c))
        .parse_next(input)
}

#[inline(always)]
fn not_nexttime_binding_power<'s>() -> u8 {
    no_assoc(7)
}

#[inline(always)]
fn and_binding_power<'s>() -> (u8, u8) {
    left_assoc(6)
}

#[inline(always)]
fn or_binding_power<'s>() -> (u8, u8) {
    left_assoc(5)
}

#[inline(always)]
fn iff_binding_power<'s>() -> (u8, u8) {
    right_assoc(4)
}

#[inline(always)]
fn until_binding_power<'s>() -> (u8, u8) {
    right_assoc(3)
}

#[inline(always)]
fn impl_followed_by_binding_power<'s>() -> (u8, u8) {
    right_assoc(2)
}

#[inline(always)]
fn property_mod_binding_power<'s>() -> u8 {
    no_assoc(1)
}

enum PropertyExprSeqOp<'a> {
    OverlapImpl(Metadata<'a>),
    NonoverlapImpl(Metadata<'a>),
    OverlapFollowedBy(Metadata<'a>),
    NonoverlapFollowedBy(Metadata<'a>),
}

fn basic_property_expr_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<PropertyExpr<'s>, VerboseError<'s>> {
    let mut _sequence_parser = (
        gen_sequence_expr_parser(min_bp, false),
        opt_note((
            alt((
                token(Token::PipeMinusGt)
                    .map(|a| PropertyExprSeqOp::OverlapImpl(a)),
                token(Token::PipeEqGt)
                    .map(|a| PropertyExprSeqOp::NonoverlapImpl(a)),
                token(Token::PoundMinusPound)
                    .map(|a| PropertyExprSeqOp::OverlapFollowedBy(a)),
                token(Token::PoundEqPound)
                    .map(|a| PropertyExprSeqOp::NonoverlapFollowedBy(a)),
            ))
            .verify_map(|a| {
                let (l_bp, _) = impl_followed_by_binding_power();
                if l_bp < min_bp { None } else { Some(a) }
            }),
            gen_property_expr_parser(impl_followed_by_binding_power().1),
        )),
    )
        .map(|(a, b)| match b {
            None => PropertyExpr::SeqExpr(Box::new(a)),
            Some((PropertyExprSeqOp::OverlapImpl(metadata), rhs)) => {
                PropertyExpr::OverlapImpl(Box::new((a, metadata, rhs)))
            }
            Some((PropertyExprSeqOp::NonoverlapImpl(metadata), rhs)) => {
                PropertyExpr::NonoverlapImpl(Box::new((a, metadata, rhs)))
            }
            Some((PropertyExprSeqOp::OverlapFollowedBy(metadata), rhs)) => {
                PropertyExpr::OverlapFollowedBy(Box::new((a, metadata, rhs)))
            }
            Some((PropertyExprSeqOp::NonoverlapFollowedBy(metadata), rhs)) => {
                PropertyExpr::NonoverlapFollowedBy(Box::new((a, metadata, rhs)))
            }
        });
    let _strong_parser = (
        token(Token::Strong),
        token(Token::Paren),
        sequence_expr_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| PropertyExpr::Strong(Box::new((a, b, c, d))));
    let _weak_parser = (
        token(Token::Weak),
        token(Token::Paren),
        sequence_expr_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| PropertyExpr::Weak(Box::new((a, b, c, d))));
    let _paren_parser = (
        token(Token::Paren),
        property_expr_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| PropertyExpr::Paren(Box::new((a, b, c))));
    let _not_parser = (
        token(Token::Not),
        gen_property_expr_parser(not_nexttime_binding_power()),
    )
        .map(|(a, b)| PropertyExpr::Not(Box::new((a, b))));
    let _conditional_parser = (
        token(Token::If),
        token(Token::Paren),
        expression_or_dist_parser,
        token(Token::EParen),
        gen_property_expr_parser(property_mod_binding_power()),
        opt_note((
            token(Token::Else),
            gen_property_expr_parser(property_mod_binding_power()),
        )),
    )
        .map(|(a, b, c, d, e, f)| {
            PropertyExpr::Conditional(Box::new((a, b, c, d, e, f)))
        });
    let _case_parser = (
        token(Token::Case),
        token(Token::Paren),
        expression_or_dist_parser,
        token(Token::EParen),
        property_case_item_parser,
        repeat_note(property_case_item_parser),
        token(Token::Endcase),
    )
        .map(|(a, b, c, d, e, f, g)| {
            PropertyExpr::Case(Box::new((a, b, c, d, e, f, g)))
        });
    let _nexttime_parser = (
        token(Token::Nexttime),
        opt_note((
            token(Token::Bracket),
            constant_expression_parser,
            token(Token::EBracket),
        )),
        gen_property_expr_parser(not_nexttime_binding_power()),
    )
        .map(|(a, b, c)| match b {
            None => PropertyExpr::Nexttime(Box::new((a, c))),
            Some((begin, const_expr, end)) => PropertyExpr::NexttimeExpr(
                Box::new((a, begin, const_expr, end, c)),
            ),
        });
    let _s_nexttime_parser = (
        token(Token::SNexttime),
        opt_note((
            token(Token::Bracket),
            constant_expression_parser,
            token(Token::EBracket),
        )),
        gen_property_expr_parser(not_nexttime_binding_power()),
    )
        .map(|(a, b, c)| match b {
            None => PropertyExpr::SNexttime(Box::new((a, c))),
            Some((begin, const_expr, end)) => PropertyExpr::SNexttimeExpr(
                Box::new((a, begin, const_expr, end, c)),
            ),
        });
    let _always_parser = (
        token(Token::Always),
        opt_note((
            token(Token::Bracket),
            cycle_delay_const_range_expression_parser,
            token(Token::EBracket),
        )),
        gen_property_expr_parser(property_mod_binding_power()),
    )
        .map(|(a, b, c)| match b {
            None => PropertyExpr::Always(Box::new((a, c))),
            Some((begin, range, end)) => {
                PropertyExpr::AlwaysRange(Box::new((a, begin, range, end, c)))
            }
        });
    let _s_always_parser = (
        token(Token::SAlways),
        token(Token::Bracket),
        constant_range_parser,
        token(Token::EBracket),
        gen_property_expr_parser(property_mod_binding_power()),
    )
        .map(|(a, b, c, d, e)| {
            PropertyExpr::SAlways(Box::new((a, b, c, d, e)))
        });
    let _eventually_parser = (
        token(Token::Eventually),
        token(Token::Bracket),
        constant_range_parser,
        token(Token::EBracket),
        gen_property_expr_parser(property_mod_binding_power()),
    )
        .map(|(a, b, c, d, e)| {
            PropertyExpr::Eventually(Box::new((a, b, c, d, e)))
        });
    let _s_eventually_parser = (
        token(Token::SEventually),
        opt_note((
            token(Token::Bracket),
            cycle_delay_const_range_expression_parser,
            token(Token::EBracket),
        )),
        gen_property_expr_parser(property_mod_binding_power()),
    )
        .map(|(a, b, c)| match b {
            None => PropertyExpr::SEventually(Box::new((a, c))),
            Some((begin, range, end)) => PropertyExpr::SEventuallyRange(
                Box::new((a, begin, range, end, c)),
            ),
        });
    let _accept_on_parser = (
        token(Token::AcceptOn),
        token(Token::Paren),
        expression_or_dist_parser,
        token(Token::EParen),
        gen_property_expr_parser(property_mod_binding_power()),
    )
        .map(|(a, b, c, d, e)| {
            PropertyExpr::AcceptOn(Box::new((a, b, c, d, e)))
        });
    let _reject_on_parser = (
        token(Token::RejectOn),
        token(Token::Paren),
        expression_or_dist_parser,
        token(Token::EParen),
        gen_property_expr_parser(property_mod_binding_power()),
    )
        .map(|(a, b, c, d, e)| {
            PropertyExpr::RejectOn(Box::new((a, b, c, d, e)))
        });
    let _sync_accept_on_parser = (
        token(Token::SyncAcceptOn),
        token(Token::Paren),
        expression_or_dist_parser,
        token(Token::EParen),
        gen_property_expr_parser(property_mod_binding_power()),
    )
        .map(|(a, b, c, d, e)| {
            PropertyExpr::SyncAcceptOn(Box::new((a, b, c, d, e)))
        });
    let _sync_reject_on_parser = (
        token(Token::SyncRejectOn),
        token(Token::Paren),
        expression_or_dist_parser,
        token(Token::EParen),
        gen_property_expr_parser(property_mod_binding_power()),
    )
        .map(|(a, b, c, d, e)| {
            PropertyExpr::SyncRejectOn(Box::new((a, b, c, d, e)))
        });
    let _property_instance_parser =
        property_instance_parser.map(|a| PropertyExpr::Instance(Box::new(a)));
    let _clocking_parser = (clocking_event_parser, property_expr_parser)
        .map(|(a, b)| PropertyExpr::Clocking(Box::new((a, b))));
    alt((
        _sequence_parser,
        _strong_parser,
        _weak_parser,
        _paren_parser,
        _not_parser,
        _conditional_parser,
        _case_parser,
        _nexttime_parser,
        _s_nexttime_parser,
        _always_parser,
        _s_always_parser,
        _eventually_parser,
        _s_eventually_parser,
        _accept_on_parser,
        _reject_on_parser,
        _sync_accept_on_parser,
        _sync_reject_on_parser,
        _property_instance_parser,
        _clocking_parser,
    ))
    .parse_next(input)
}

enum PropertyPrattOp<'a> {
    Or(Metadata<'a>),
    And(Metadata<'a>),
    Until(Metadata<'a>),
    SUntil(Metadata<'a>),
    UntilWith(Metadata<'a>),
    SUntilWith(Metadata<'a>),
    Implies(Metadata<'a>),
    Iff(Metadata<'a>),
}

fn property_expr_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<PropertyExpr<'s>, VerboseError<'s>> {
    let mut lhs = basic_property_expr_parser(input, min_bp)?;
    loop {
        let Ok((op, r_bp)) = alt((
            token(Token::Or).verify_map(|a| {
                let (l_bp, r_bp) = or_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((PropertyPrattOp::Or(a), r_bp))
            }),
            token(Token::And).verify_map(|a| {
                let (l_bp, r_bp) = and_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((PropertyPrattOp::And(a), r_bp))
            }),
            token(Token::Until).verify_map(|a| {
                let (l_bp, r_bp) = until_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((PropertyPrattOp::Until(a), r_bp))
            }),
            token(Token::SUntil).verify_map(|a| {
                let (l_bp, r_bp) = until_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((PropertyPrattOp::SUntil(a), r_bp))
            }),
            token(Token::UntilWith).verify_map(|a| {
                let (l_bp, r_bp) = until_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((PropertyPrattOp::UntilWith(a), r_bp))
            }),
            token(Token::SUntilWith).verify_map(|a| {
                let (l_bp, r_bp) = until_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((PropertyPrattOp::SUntilWith(a), r_bp))
            }),
            token(Token::Implies).verify_map(|a| {
                let (l_bp, r_bp) = until_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((PropertyPrattOp::Implies(a), r_bp))
            }),
            token(Token::Iff).verify_map(|a| {
                let (l_bp, r_bp) = iff_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((PropertyPrattOp::Iff(a), r_bp))
            }),
        ))
        .parse_next(input) else {
            return Ok(lhs);
        };
        lhs = match op {
            PropertyPrattOp::And(metadata) => {
                let rhs = property_expr_bp_parser(input, r_bp)?;
                PropertyExpr::And(Box::new((lhs, metadata, rhs)))
            }
            PropertyPrattOp::Or(metadata) => {
                let rhs = property_expr_bp_parser(input, r_bp)?;
                PropertyExpr::Or(Box::new((lhs, metadata, rhs)))
            }
            PropertyPrattOp::Until(metadata) => {
                let rhs = property_expr_bp_parser(input, r_bp)?;
                PropertyExpr::Until(Box::new((lhs, metadata, rhs)))
            }
            PropertyPrattOp::SUntil(metadata) => {
                let rhs = property_expr_bp_parser(input, r_bp)?;
                PropertyExpr::SUntil(Box::new((lhs, metadata, rhs)))
            }
            PropertyPrattOp::UntilWith(metadata) => {
                let rhs = property_expr_bp_parser(input, r_bp)?;
                PropertyExpr::UntilWith(Box::new((lhs, metadata, rhs)))
            }
            PropertyPrattOp::SUntilWith(metadata) => {
                let rhs = property_expr_bp_parser(input, r_bp)?;
                PropertyExpr::SUntilWith(Box::new((lhs, metadata, rhs)))
            }
            PropertyPrattOp::Implies(metadata) => {
                let rhs = property_expr_bp_parser(input, r_bp)?;
                PropertyExpr::Implies(Box::new((lhs, metadata, rhs)))
            }
            PropertyPrattOp::Iff(metadata) => {
                let rhs = property_expr_bp_parser(input, r_bp)?;
                PropertyExpr::Iff(Box::new((lhs, metadata, rhs)))
            }
        }
    }
}

fn gen_property_expr_parser<'s>(
    min_bp: u8,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<PropertyExpr<'s>, VerboseError<'s>>
{
    move |input| property_expr_bp_parser(input, min_bp)
}

pub fn property_expr_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyExpr<'s>, VerboseError<'s>> {
    property_expr_bp_parser(input, 0)
}

pub fn property_case_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyCaseItem<'s>, VerboseError<'s>> {
    let _expr_parser = (
        expression_or_dist_parser,
        repeat_note((token(Token::Comma), expression_or_dist_parser)),
        token(Token::Colon),
        property_expr_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            PropertyCaseItem::Expr(Box::new((a, b, c, d, e)))
        });
    let _default_parser = (
        token(Token::Default),
        opt_note(token(Token::Colon)),
        property_expr_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| PropertyCaseItem::Default(Box::new((a, b, c, d))));
    alt((_default_parser, _expr_parser)).parse_next(input)
}

pub fn sequence_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Sequence),
        sequence_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(sequence_port_list_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
        repeat_note(assertion_variable_declaration_parser),
        sequence_expr_parser,
        opt_note(token(Token::SColon)),
        token(Token::Endsequence),
        opt_note((token(Token::Colon), sequence_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i)| {
            SequenceDeclaration(a, b, c, d, e, f, g, h, i)
        })
        .parse_next(input)
}

pub fn sequence_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequencePortList<'s>, VerboseError<'s>> {
    (
        sequence_port_item_parser,
        repeat_note((token(Token::Comma), sequence_port_item_parser)),
    )
        .map(|(a, b)| SequencePortList(a, b))
        .parse_next(input)
}

pub fn sequence_port_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequencePortItem<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        opt_note((
            token(Token::Local),
            opt_note(sequence_lvar_port_direction_parser),
        )),
        sequence_formal_type_parser,
        formal_port_identifier_parser,
        repeat_note(variable_dimension_parser),
        opt_note((token(Token::Eq), sequence_actual_arg_parser)),
    )
        .map(|(a, b, c, d, e, f)| SequencePortItem(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn sequence_lvar_port_direction_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceLvarPortDirection<'s>, VerboseError<'s>> {
    alt((
        token(Token::Input).map(|a| SequenceLvarPortDirection::Input(a)),
        token(Token::Inout).map(|a| SequenceLvarPortDirection::Inout(a)),
        token(Token::Output).map(|a| SequenceLvarPortDirection::Output(a)),
    ))
    .parse_next(input)
}

pub fn sequence_formal_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceFormalType<'s>, VerboseError<'s>> {
    alt((
        data_type_or_implicit_parser_sequence_formal_type
            .map(|a| SequenceFormalType::DataTypeOrImplicit(Box::new(a))),
        token(Token::Sequence)
            .map(|a| SequenceFormalType::Sequence(Box::new(a))),
        token(Token::Untyped).map(|a| SequenceFormalType::Untyped(Box::new(a))),
    ))
    .parse_next(input)
}

fn data_type_or_implicit_parser_sequence_formal_type<'s>(
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

#[inline(always)]
fn throughout_binding_power<'s>() -> u8 {
    no_assoc(10)
}

#[inline(always)]
fn within_binding_power<'s>() -> (u8, u8) {
    left_assoc(9)
}

#[inline(always)]
fn intersect_binding_power<'s>() -> (u8, u8) {
    left_assoc(8)
}

fn basic_sequence_expr_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceExpr<'s>, VerboseError<'s>> {
    let _start_delay_parser = (
        cycle_delay_range_parser,
        sequence_expr_parser,
        repeat_note((cycle_delay_range_parser, sequence_expr_parser)),
    )
        .map(|(a, b, c)| SequenceExpr::StartDelay(Box::new((a, b, c))));
    let _expr_parser =
        (expression_or_dist_parser, opt_note(boolean_abbrev_parser))
            .map(|(a, b)| SequenceExpr::Expr(Box::new((a, b))));
    let _paren_parser = (
        token(Token::Paren),
        sequence_expr_parser,
        repeat_note((token(Token::Comma), sequence_match_item_parser)),
        token(Token::EParen),
        opt_note(sequence_abbrev_parser),
    )
        .map(|(a, b, c, d, e)| SequenceExpr::Paren(Box::new((a, b, c, d, e))));
    let _first_match_parser = (
        token(Token::FirstMatch),
        token(Token::Paren),
        sequence_expr_parser,
        repeat_note((token(Token::Comma), sequence_match_item_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| {
            SequenceExpr::FirstMatch(Box::new((a, b, c, d, e)))
        });
    let _clocking_parser = (clocking_event_parser, sequence_expr_parser)
        .map(|(a, b)| SequenceExpr::Clocking(Box::new((a, b))));
    alt((
        _start_delay_parser,
        _expr_parser,
        _paren_parser,
        _first_match_parser,
        _clocking_parser,
    ))
    .parse_next(input)
}

enum SequencePrattOp<'a> {
    CycleDelayRange(CycleDelayRange<'a>),
    Within(Metadata<'a>),
    Intersect(Metadata<'a>),
    And(Metadata<'a>),
    Or(Metadata<'a>),
}

fn sequence_expr_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
    must_consume_and_or: bool,
) -> ModalResult<SequenceExpr<'s>, VerboseError<'s>> {
    let mut lhs = alt((
        (
            expression_or_dist_parser,
            token(Token::Throughout),
            |input: &mut Tokens<'s>| {
                sequence_expr_bp_parser(
                    input,
                    throughout_binding_power(),
                    must_consume_and_or,
                )
            },
        )
            .map(|(a, b, c)| SequenceExpr::Throughout(Box::new((a, b, c)))),
        basic_sequence_expr_parser,
    ))
    .parse_next(input)?;
    let mut checkpoint = input.checkpoint();
    loop {
        let Ok((op, r_bp)) = alt((
            cycle_delay_range_parser.map(|a| {
                (SequencePrattOp::CycleDelayRange(a), 200) // bp not used
            }),
            token(Token::Within).verify_map(|a| {
                let (l_bp, r_bp) = within_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SequencePrattOp::Within(a), r_bp))
            }),
            token(Token::Intersect).verify_map(|a| {
                let (l_bp, r_bp) = intersect_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SequencePrattOp::Intersect(a), r_bp))
            }),
            token(Token::And).verify_map(|a| {
                let (l_bp, r_bp) = and_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SequencePrattOp::And(a), r_bp))
            }),
            token(Token::Or).verify_map(|a| {
                let (l_bp, r_bp) = or_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SequencePrattOp::Or(a), r_bp))
            }),
        ))
        .parse_next(input) else {
            return Ok(lhs);
        };
        lhs = match op {
            SequencePrattOp::CycleDelayRange(cycle_delay_range) => {
                let next_sequence_expr = sequence_expr_parser(input)?;
                let later_delays = repeat_note((
                    cycle_delay_range_parser,
                    sequence_expr_parser,
                ))
                .parse_next(input)?;
                SequenceExpr::Delay(Box::new((
                    lhs,
                    cycle_delay_range,
                    next_sequence_expr,
                    later_delays,
                )))
            }
            SequencePrattOp::Within(within) => {
                let rhs =
                    sequence_expr_bp_parser(input, r_bp, must_consume_and_or)?;
                SequenceExpr::Within(Box::new((lhs, within, rhs)))
            }
            SequencePrattOp::Intersect(intersect) => {
                let rhs =
                    sequence_expr_bp_parser(input, r_bp, must_consume_and_or)?;
                SequenceExpr::Intersect(Box::new((lhs, intersect, rhs)))
            }
            SequencePrattOp::And(and) => {
                match sequence_expr_bp_parser(input, r_bp, must_consume_and_or)
                {
                    Ok(rhs) => SequenceExpr::And(Box::new((lhs, and, rhs))),
                    Err(err) => {
                        if must_consume_and_or {
                            return Err(err);
                        } else {
                            input.reset(&checkpoint);
                            lhs
                        }
                    }
                }
            }
            SequencePrattOp::Or(or) => {
                match sequence_expr_bp_parser(input, r_bp, must_consume_and_or)
                {
                    Ok(rhs) => SequenceExpr::Or(Box::new((lhs, or, rhs))),
                    Err(err) => {
                        if must_consume_and_or {
                            return Err(err);
                        } else {
                            input.reset(&checkpoint);
                            lhs
                        }
                    }
                }
            }
        };
        checkpoint = input.checkpoint();
    }
}

fn gen_sequence_expr_parser<'s>(
    min_bp: u8,
    must_consume_and_or: bool,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<SequenceExpr<'s>, VerboseError<'s>>
{
    move |input| sequence_expr_bp_parser(input, min_bp, must_consume_and_or)
}

pub fn sequence_expr_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceExpr<'s>, VerboseError<'s>> {
    sequence_expr_bp_parser(input, 0, true)
}

pub fn cycle_delay_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CycleDelayRange<'s>, VerboseError<'s>> {
    let _primary_parser = (token(Token::PoundPound), constant_primary_parser)
        .map(|(a, b)| CycleDelayRange::Primary(Box::new((a, b))));
    let _range_parser = (
        token(Token::PoundPound),
        token(Token::Bracket),
        cycle_delay_const_range_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| CycleDelayRange::Range(Box::new((a, b, c, d))));
    let _star_parser = (
        token(Token::PoundPound),
        token(Token::Bracket),
        token(Token::Star),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| CycleDelayRange::Star(Box::new((a, b, c, d))));
    let _plus_parser = (
        token(Token::PoundPound),
        token(Token::Bracket),
        token(Token::Plus),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| CycleDelayRange::Plus(Box::new((a, b, c, d))));
    alt((_primary_parser, _range_parser, _star_parser, _plus_parser))
        .parse_next(input)
}

pub fn sequence_method_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceMethodCall<'s>, VerboseError<'s>> {
    (
        sequence_instance_parser,
        token(Token::Period),
        method_identifier_parser,
    )
        .map(|(a, b, c)| SequenceMethodCall(a, b, c))
        .parse_next(input)
}

pub fn sequence_match_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceMatchItem<'s>, VerboseError<'s>> {
    alt((
        operator_assignment_parser
            .map(|a| SequenceMatchItem::Operator(Box::new(a))),
        inc_or_dec_expression_parser
            .map(|a| SequenceMatchItem::IncOrDec(Box::new(a))),
        subroutine_call_parser
            .map(|a| SequenceMatchItem::Subroutine(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn sequence_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceInstance<'s>, VerboseError<'s>> {
    (
        ps_or_hierarchical_sequence_identifier_parser,
        opt_note((
            token(Token::Paren),
            opt_note(sequence_list_of_arguments_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b)| SequenceInstance(a, b))
        .parse_next(input)
}

pub fn sequence_list_of_arguments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceListOfArguments<'s>, VerboseError<'s>> {
    let _partial_identifier_parser = (
        opt_note(sequence_actual_arg_parser),
        repeat_note((
            token(Token::Comma),
            opt_note(sequence_actual_arg_parser),
        )),
        repeat_note((
            token(Token::Comma),
            token(Token::Period),
            identifier_parser,
            token(Token::Paren),
            opt_note(sequence_actual_arg_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c)| {
            SequenceListOfArguments::PartialIdentifier(Box::new((a, b, c)))
        });
    let _identifier_parser = (
        token(Token::Period),
        identifier_parser,
        token(Token::Paren),
        opt_note(sequence_actual_arg_parser),
        token(Token::EParen),
        repeat_note((
            token(Token::Comma),
            token(Token::Period),
            identifier_parser,
            token(Token::Paren),
            opt_note(sequence_actual_arg_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d, e, f)| {
            SequenceListOfArguments::Identifier(Box::new((a, b, c, d, e, f)))
        });
    alt((_partial_identifier_parser, _identifier_parser)).parse_next(input)
}

pub fn sequence_actual_arg_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceActualArg<'s>, VerboseError<'s>> {
    alt((
        event_expression_parser.map(|a| SequenceActualArg::Event(Box::new(a))),
        sequence_expr_parser.map(|a| SequenceActualArg::Sequence(Box::new(a))),
        token(Token::Dollar).map(|a| SequenceActualArg::Dollar(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn boolean_abbrev_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BooleanAbbrev<'s>, VerboseError<'s>> {
    alt((
        consecutive_repetition_parser
            .map(|a| BooleanAbbrev::Consecutive(Box::new(a))),
        nonconsecutive_repetition_parser
            .map(|a| BooleanAbbrev::Nonconsecutive(Box::new(a))),
        goto_repetition_parser.map(|a| BooleanAbbrev::Goto(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn sequence_abbrev_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceAbbrev<'s>, VerboseError<'s>> {
    consecutive_repetition_parser
        .map(|a| SequenceAbbrev(a))
        .parse_next(input)
}

pub fn consecutive_repetition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConsecutiveRepetition<'s>, VerboseError<'s>> {
    let _expr_parser = (
        token(Token::Bracket),
        token(Token::Star),
        const_or_range_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| {
            ConsecutiveRepetition::Expr(Box::new((a, b, c, d)))
        });
    let _star_parser = (
        token(Token::Bracket),
        token(Token::Star),
        token(Token::EBracket),
    )
        .map(|(a, b, c)| ConsecutiveRepetition::Star(Box::new((a, b, c))));
    let _plus_parser = (
        token(Token::Bracket),
        token(Token::Plus),
        token(Token::EBracket),
    )
        .map(|(a, b, c)| ConsecutiveRepetition::Plus(Box::new((a, b, c))));
    alt((_expr_parser, _star_parser, _plus_parser)).parse_next(input)
}

pub fn nonconsecutive_repetition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonconsecutiveRepetition<'s>, VerboseError<'s>> {
    (
        token(Token::Bracket),
        token(Token::Eq),
        const_or_range_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| NonconsecutiveRepetition(a, b, c, d))
        .parse_next(input)
}

pub fn goto_repetition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GotoRepetition<'s>, VerboseError<'s>> {
    (
        token(Token::Bracket),
        token(Token::MinusGt),
        const_or_range_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| GotoRepetition(a, b, c, d))
        .parse_next(input)
}

pub fn const_or_range_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstOrRangeExpression<'s>, VerboseError<'s>> {
    alt((
        constant_expression_parser
            .map(|a| ConstOrRangeExpression::Expr(Box::new(a))),
        cycle_delay_const_range_expression_parser
            .map(|a| ConstOrRangeExpression::Range(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn cycle_delay_const_range_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CycleDelayConstRangeExpression<'s>, VerboseError<'s>> {
    let _bounded_parser = (
        constant_expression_parser,
        token(Token::Colon),
        constant_expression_parser,
    )
        .map(|(a, b, c)| {
            CycleDelayConstRangeExpression::Bounded(Box::new((a, b, c)))
        });
    let _unbounded_parser = (
        constant_expression_parser,
        token(Token::Colon),
        token(Token::Dollar),
    )
        .map(|(a, b, c)| {
            CycleDelayConstRangeExpression::Unbounded(Box::new((a, b, c)))
        });
    alt((_bounded_parser, _unbounded_parser)).parse_next(input)
}

pub fn assertion_variable_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssertionVariableDeclaration<'s>, VerboseError<'s>> {
    (
        var_data_type_parser,
        list_of_variable_decl_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| AssertionVariableDeclaration(a, b, c))
        .parse_next(input)
}
