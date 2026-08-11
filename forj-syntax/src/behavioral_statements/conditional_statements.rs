// =======================================================================
// conditional_statements.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.6
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConditionalStatement<'a>(
    pub Option<UniquePriority<'a>>,
    pub Metadata<'a>, // if
    pub Metadata<'a>, // (
    pub CondPredicate<'a>,
    pub Metadata<'a>, // )
    pub StatementOrNull<'a>,
    pub  Vec<(
        Metadata<'a>, // else
        Metadata<'a>, // if
        Metadata<'a>, // (
        CondPredicate<'a>,
        Metadata<'a>, // )
        StatementOrNull<'a>,
    )>,
    pub  Option<(
        Metadata<'a>, // else
        StatementOrNull<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum UniquePriority<'a> {
    Unique(Metadata<'a>),
    Unique0(Metadata<'a>),
    Priority(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CondPredicate<'a>(
    pub ExpressionOrCondPattern<'a>,
    pub  Vec<(
        Metadata<'a>, // &&&
        ExpressionOrCondPattern<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionOrCondPattern<'a> {
    Expression(Box<Expression<'a>>),
    CondPattern(Box<CondPattern<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CondPattern<'a>(
    pub Expression<'a>,
    pub Metadata<'a>, // matches
    pub Pattern<'a>,
);
