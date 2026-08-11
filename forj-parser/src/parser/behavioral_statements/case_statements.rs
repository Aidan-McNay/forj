// =======================================================================
// case_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.7

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

enum CaseStatementBody<'a> {
    Regular((CaseItem<'a>, Vec<CaseItem<'a>>)),
    Matches(
        (
            Metadata<'a>, // matches
            CasePatternItem<'a>,
            Vec<CasePatternItem<'a>>,
        ),
    ),
    Inside(
        (
            Metadata<'a>, // inside
            CaseInsideItem<'a>,
            Vec<CaseInsideItem<'a>>,
        ),
    ),
}

pub fn case_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CaseStatement<'s>, VerboseError<'s>> {
    let _case_statement_header_parser = (
        opt_note(unique_priority_parser),
        case_keyword_parser,
        token(Token::Paren),
        case_expression_parser,
        token(Token::EParen),
    );
    let _regular_body_parser =
        (case_item_parser, repeat_note(case_item_parser))
            .map(|(a, b)| CaseStatementBody::Regular((a, b)));
    let _matches_body_parser = (
        token(Token::Matches),
        case_pattern_item_parser,
        repeat_note(case_pattern_item_parser),
    )
        .map(|(a, b, c)| CaseStatementBody::Matches((a, b, c)));
    let _inside_body_parser = (
        token(Token::Inside),
        case_inside_item_parser,
        repeat_note(case_inside_item_parser),
    )
        .map(|(a, b, c)| CaseStatementBody::Inside((a, b, c)));
    (
        _case_statement_header_parser,
        alt((
            _regular_body_parser,
            _matches_body_parser,
            _inside_body_parser,
        )),
        token(Token::Endcase),
    )
        .map(|((a, b, c, d, e), body, end)| match body {
            CaseStatementBody::Regular((first, rest)) => {
                CaseStatement::Regular(Box::new((
                    a, b, c, d, e, first, rest, end,
                )))
            }
            CaseStatementBody::Matches((matches, first, rest)) => {
                CaseStatement::Matches(Box::new((
                    a, b, c, d, e, matches, first, rest, end,
                )))
            }
            CaseStatementBody::Inside((inside, first, rest)) => {
                CaseStatement::Inside(Box::new((
                    a, b, c, d, e, inside, first, rest, end,
                )))
            }
        })
        .parse_next(input)
}

pub fn case_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CaseExpression<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| CaseExpression(a))
        .parse_next(input)
}

pub fn case_keyword_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CaseKeyword<'s>, VerboseError<'s>> {
    alt((
        token(Token::Case).map(|a| CaseKeyword::Case(a)),
        token(Token::Casez).map(|a| CaseKeyword::Casez(a)),
        token(Token::Casex).map(|a| CaseKeyword::Casex(a)),
    ))
    .parse_next(input)
}

pub fn case_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CaseItem<'s>, VerboseError<'s>> {
    let _expression_parser = (
        case_item_expression_parser,
        repeat_note((token(Token::Comma), case_item_expression_parser)),
        token(Token::Colon),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d)| CaseItem::Expression(Box::new((a, b, c, d))));
    let _default_parser = (
        token(Token::Default),
        opt_note(token(Token::Colon)),
        statement_or_null_parser,
    )
        .map(|(a, b, c)| CaseItem::Default(Box::new((a, b, c))));
    alt((_default_parser, _expression_parser)).parse_next(input)
}

pub fn case_pattern_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CasePatternItem<'s>, VerboseError<'s>> {
    let _expression_parser = (
        pattern_parser,
        opt_note((token(Token::AmpAmpAmp), expression_parser)),
        token(Token::Colon),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d)| {
            CasePatternItem::Expression(Box::new((a, b, c, d)))
        });
    let _default_parser = (
        token(Token::Default),
        opt_note(token(Token::Colon)),
        statement_or_null_parser,
    )
        .map(|(a, b, c)| CasePatternItem::Default(Box::new((a, b, c))));
    alt((_default_parser, _expression_parser)).parse_next(input)
}

pub fn case_inside_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CaseInsideItem<'s>, VerboseError<'s>> {
    let _expression_parser = (
        range_list_parser,
        token(Token::Colon),
        statement_or_null_parser,
    )
        .map(|(a, b, c)| CaseInsideItem::Expression(Box::new((a, b, c))));
    let _default_parser = (
        token(Token::Default),
        opt_note(token(Token::Colon)),
        statement_or_null_parser,
    )
        .map(|(a, b, c)| CaseInsideItem::Default(Box::new((a, b, c))));
    alt((_default_parser, _expression_parser)).parse_next(input)
}

pub fn case_item_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CaseItemExpression<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| CaseItemExpression(a))
        .parse_next(input)
}

pub fn randcase_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RandcaseStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Randcase),
        randcase_item_parser,
        repeat_note(randcase_item_parser),
        token(Token::Endcase),
    )
        .map(|(a, b, c, d)| RandcaseStatement(a, b, c, d))
        .parse_next(input)
}

pub fn randcase_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RandcaseItem<'s>, VerboseError<'s>> {
    (
        expression_parser,
        token(Token::Comma),
        statement_or_null_parser,
    )
        .map(|(a, b, c)| RandcaseItem(a, b, c))
        .parse_next(input)
}

pub fn range_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RangeList<'s>, VerboseError<'s>> {
    let _value_range_vec_parser =
        repeat_note((token(Token::Comma), value_range_parser));
    (value_range_parser, _value_range_vec_parser)
        .map(|(a, b)| RangeList(a, b))
        .parse_next(input)
}

pub fn value_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ValueRange<'s>, VerboseError<'s>> {
    let _expression_parser =
        expression_parser.map(|a| ValueRange::Expression(Box::new(a)));
    let _slice_parser = (
        token(Token::Bracket),
        expression_parser,
        token(Token::Colon),
        expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| ValueRange::Slice(Box::new((a, b, c, d, e))));
    let _dollar_low_parser = (
        token(Token::Bracket),
        token(Token::Dollar),
        token(Token::Colon),
        expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            ValueRange::DollarLow(Box::new((a, b, c, d, e)))
        });
    let _dollar_high_parser = (
        token(Token::Bracket),
        expression_parser.clone(),
        token(Token::Colon),
        token(Token::Dollar),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            ValueRange::DollarHigh(Box::new((a, b, c, d, e)))
        });
    let _absolute_tolerance_parser = (
        token(Token::Bracket),
        expression_parser,
        token(Token::PlusSlashMinus),
        expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            ValueRange::AbsoluteTolerance(Box::new((a, b, c, d, e)))
        });
    let _relative_tolerance_parser = (
        token(Token::Bracket),
        expression_parser,
        token(Token::PlusPercentMinus),
        expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            ValueRange::RelativeTolerance(Box::new((a, b, c, d, e)))
        });
    alt((
        _expression_parser,
        _slice_parser,
        _dollar_low_parser,
        _dollar_high_parser,
        _absolute_tolerance_parser,
        _relative_tolerance_parser,
    ))
    .parse_next(input)
}
