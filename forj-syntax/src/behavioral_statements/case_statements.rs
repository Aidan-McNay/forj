// =======================================================================
// case_statements.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.7
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CaseStatement<'a> {
    Regular(
        Box<(
            Option<UniquePriority<'a>>,
            CaseKeyword<'a>,
            Metadata<'a>, // (
            CaseExpression<'a>,
            Metadata<'a>, // )
            CaseItem<'a>,
            Vec<CaseItem<'a>>,
            Metadata<'a>, // endcase
        )>,
    ),
    Matches(
        Box<(
            Option<UniquePriority<'a>>,
            CaseKeyword<'a>,
            Metadata<'a>, // (
            CaseExpression<'a>,
            Metadata<'a>, // )
            Metadata<'a>, // matches
            CasePatternItem<'a>,
            Vec<CasePatternItem<'a>>,
            Metadata<'a>, // endcase
        )>,
    ),
    Inside(
        Box<(
            Option<UniquePriority<'a>>,
            CaseKeyword<'a>,
            Metadata<'a>, // (
            CaseExpression<'a>,
            Metadata<'a>, // )
            Metadata<'a>, // inside
            CaseInsideItem<'a>,
            Vec<CaseInsideItem<'a>>,
            Metadata<'a>, // endcase
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CaseKeyword<'a> {
    Case(Metadata<'a>),
    Casez(Metadata<'a>),
    Casex(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaseExpression<'a>(pub Expression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum CaseItem<'a> {
    Expression(
        Box<(
            CaseItemExpression<'a>,
            Vec<(
                Metadata<'a>, // ,
                CaseItemExpression<'a>,
            )>,
            Metadata<'a>, // :
            StatementOrNull<'a>,
        )>,
    ),
    Default(
        Box<(
            Metadata<'a>,         // default
            Option<Metadata<'a>>, // :
            StatementOrNull<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CasePatternItem<'a> {
    Expression(
        Box<(
            Pattern<'a>,
            Option<(
                Metadata<'a>, // &&&
                Expression<'a>,
            )>,
            Metadata<'a>, // :
            StatementOrNull<'a>,
        )>,
    ),
    Default(
        Box<(
            Metadata<'a>,         // default
            Option<Metadata<'a>>, // :
            StatementOrNull<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CaseInsideItem<'a> {
    Expression(
        Box<(
            RangeList<'a>,
            Metadata<'a>, // :
            StatementOrNull<'a>,
        )>,
    ),
    Default(
        Box<(
            Metadata<'a>,         // default
            Option<Metadata<'a>>, // :
            StatementOrNull<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaseItemExpression<'a>(pub Expression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct RandcaseStatement<'a>(
    pub Metadata<'a>, // randcase
    pub RandcaseItem<'a>,
    pub Vec<RandcaseItem<'a>>,
    pub Metadata<'a>, // endcase
);

#[derive(Clone, Debug, PartialEq)]
pub struct RandcaseItem<'a>(
    pub Expression<'a>,
    pub Metadata<'a>, // ,
    pub StatementOrNull<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct RangeList<'a>(
    pub ValueRange<'a>,
    pub Vec<(Metadata<'a>, ValueRange<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ValueRange<'a> {
    Expression(Box<Expression<'a>>),
    Slice(
        Box<(
            Metadata<'a>, // [
            Expression<'a>,
            Metadata<'a>, // :
            Expression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    DollarLow(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // $
            Metadata<'a>, // :
            Expression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    DollarHigh(
        Box<(
            Metadata<'a>, // [
            Expression<'a>,
            Metadata<'a>, // :
            Metadata<'a>, // $
            Metadata<'a>, // ]
        )>,
    ),
    AbsoluteTolerance(
        Box<(
            Metadata<'a>, // [
            Expression<'a>,
            Metadata<'a>, // +/-
            Expression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    RelativeTolerance(
        Box<(
            Metadata<'a>, // [
            Expression<'a>,
            Metadata<'a>, // +%-
            Expression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
}
