// =======================================================================
// constraints.rs
// =======================================================================
// Parsing for 1800-2023 A.1.10

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn constraint_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintDeclaration<'s>, VerboseError<'s>> {
    (
        opt_note(token(Token::Static)),
        token(Token::Constraint),
        opt_dynamic_override_specifiers_parser,
        constraint_identifier_parser,
        constraint_block_parser,
    )
        .map(|(a, b, c, d, e)| ConstraintDeclaration(a, b, c, d, e))
        .parse_next(input)
}

pub fn constraint_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintBlock<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        repeat_note(constraint_block_item_parser),
        token(Token::EBrace),
    )
        .map(|(a, b, c)| ConstraintBlock(a, b, c))
        .parse_next(input)
}

pub fn constraint_block_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintBlockItem<'s>, VerboseError<'s>> {
    let _ordering_parser = (
        token(Token::Solve),
        solve_before_list_parser,
        token(Token::Before),
        solve_before_list_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            ConstraintBlockItem::Ordering(Box::new((a, b, c, d, e)))
        });
    let _expression_parser = constraint_expression_parser
        .map(|a| ConstraintBlockItem::Expression(Box::new(a)));
    alt((_ordering_parser, _expression_parser)).parse_next(input)
}

pub fn solve_before_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SolveBeforeList<'s>, VerboseError<'s>> {
    (
        constraint_primary_parser,
        repeat_note((token(Token::Comma), constraint_primary_parser)),
    )
        .map(|(a, b)| SolveBeforeList(a, b))
        .parse_next(input)
}

pub fn constraint_primary_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintPrimary<'s>, VerboseError<'s>> {
    (
        implicit_class_handle_or_class_scope_parser,
        hierarchical_identifier_parser,
        select_parser,
        opt_note((token(Token::Paren), token(Token::EParen))),
    )
        .map(|(a, b, c, d)| ConstraintPrimary(a, b, c, d))
        .parse_next(input)
}

pub fn constraint_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintExpression<'s>, VerboseError<'s>> {
    let _expression_parser = (
        opt_note(token(Token::Soft)),
        expression_or_dist_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| ConstraintExpression::Expression(Box::new((a, b, c))));
    let _uniqueness_parser =
        (uniqueness_constraint_parser, token(Token::SColon))
            .map(|(a, b)| ConstraintExpression::Uniqueness(Box::new((a, b))));
    let _implication_parser = (
        expression_parser,
        token(Token::MinusGt),
        constraint_set_parser,
    )
        .map(|(a, b, c)| {
            ConstraintExpression::Implication(Box::new((a, b, c)))
        });
    let _conditional_parser = (
        token(Token::If),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        constraint_set_parser,
        opt_note((token(Token::Else), constraint_set_parser)),
    )
        .map(|(a, b, c, d, e, f)| {
            ConstraintExpression::Conditional(Box::new((a, b, c, d, e, f)))
        });
    let _foreach_parser = (
        token(Token::Foreach),
        token(Token::Paren),
        ps_or_hierarchical_array_identifier_parser,
        token(Token::Bracket),
        loop_variables_parser,
        token(Token::EBracket),
        token(Token::EParen),
        constraint_set_parser,
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            ConstraintExpression::Foreach(Box::new((a, b, c, d, e, f, g, h)))
        });
    let _disable_parser = (
        token(Token::Disable),
        token(Token::Soft),
        constraint_primary_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| {
            ConstraintExpression::Disable(Box::new((a, b, c, d)))
        });
    alt((
        _uniqueness_parser,
        _conditional_parser,
        _foreach_parser,
        _disable_parser,
        _expression_parser,
        _implication_parser,
    ))
    .parse_next(input)
}

pub fn uniqueness_constraint_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UniquenessConstraint<'s>, VerboseError<'s>> {
    (
        token(Token::Unique),
        token(Token::Brace),
        range_list_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| UniquenessConstraint(a, b, c, d))
        .parse_next(input)
}

pub fn constraint_set_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintSet<'s>, VerboseError<'s>> {
    let _single_parser = constraint_expression_parser
        .map(|a| ConstraintSet::Single(Box::new(a)));
    let _multi_parser = (
        token(Token::Brace),
        repeat_note(constraint_expression_parser),
        token(Token::EBrace),
    )
        .map(|(a, b, c)| ConstraintSet::Multi(Box::new((a, b, c))));
    alt((_single_parser, _multi_parser)).parse_next(input)
}

pub fn expression_or_dist_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ExpressionOrDist<'s>, VerboseError<'s>> {
    (
        expression_parser,
        opt_note((
            token(Token::Dist),
            token(Token::Brace),
            dist_list_parser,
            token(Token::EBrace),
        )),
    )
        .map(|(a, b)| ExpressionOrDist(a, b))
        .parse_next(input)
}

pub fn dist_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DistList<'s>, VerboseError<'s>> {
    (
        dist_item_parser,
        repeat_note((token(Token::Comma), dist_item_parser)),
    )
        .map(|(a, b)| DistList(a, b))
        .parse_next(input)
}

pub fn dist_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DistItem<'s>, VerboseError<'s>> {
    let _value_parser = (value_range_parser, opt_note(dist_weight_parser))
        .map(|(a, b)| DistItem::Value(Box::new((a, b))));
    let _default_parser = (
        token(Token::Default),
        token(Token::ColonSlash),
        expression_parser,
    )
        .map(|(a, b, c)| DistItem::Default(Box::new((a, b, c))));
    alt((_value_parser, _default_parser)).parse_next(input)
}

pub fn dist_weight_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DistWeight<'s>, VerboseError<'s>> {
    let _indv_weight_parser = (token(Token::ColonEq), expression_parser)
        .map(|(a, b)| DistWeight::IndvWeight(a, b));
    let _range_weight_parser = (token(Token::ColonSlash), expression_parser)
        .map(|(a, b)| DistWeight::RangeWeight(a, b));
    alt((_indv_weight_parser, _range_weight_parser)).parse_next(input)
}

pub fn constraint_prototype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintPrototype<'s>, VerboseError<'s>> {
    (
        opt_note(constraint_prototype_qualifier_parser),
        opt_note(token(Token::Static)),
        token(Token::Constraint),
        opt_dynamic_override_specifiers_parser,
        constraint_identifier_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| ConstraintPrototype(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn constraint_prototype_qualifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintPrototypeQualifier<'s>, VerboseError<'s>> {
    alt((
        token(Token::Extern).map(|a| ConstraintPrototypeQualifier::Extern(a)),
        token(Token::Pure).map(|a| ConstraintPrototypeQualifier::Pure(a)),
    ))
    .parse_next(input)
}

pub fn extern_constraint_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ExternConstraintDeclaration<'s>, VerboseError<'s>> {
    (
        opt_note(token(Token::Static)),
        token(Token::Constraint),
        opt_dynamic_override_specifiers_parser,
        class_scope_parser,
        constraint_identifier_parser,
        constraint_block_parser,
    )
        .map(|(a, b, c, d, e, f)| ExternConstraintDeclaration(a, b, c, d, e, f))
        .parse_next(input)
}
