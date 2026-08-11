// =======================================================================
// specify_path_delays.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.7.4
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PathDelayValue<'a> {
    NoParen(Box<ListOfPathDelayExpressions<'a>>),
    Paren(
        Box<(
            Metadata<'a>, // (
            ListOfPathDelayExpressions<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListOfPathDelayExpressions<'a> {
    Path(Box<TPathDelayExpression<'a>>),
    RiseFall(
        Box<(
            TrisePathDelayExpression<'a>,
            Metadata<'a>, // ,
            TfallPathDelayExpression<'a>,
        )>,
    ),
    RiseFallZ(
        Box<(
            TrisePathDelayExpression<'a>,
            Metadata<'a>, // ,
            TfallPathDelayExpression<'a>,
            Metadata<'a>, // ,
            TzPathDelayExpression<'a>,
        )>,
    ),
    EdgeZ(
        Box<(
            T01PathDelayExpression<'a>,
            Metadata<'a>, // ,
            T10PathDelayExpression<'a>,
            Metadata<'a>, // ,
            T0zPathDelayExpression<'a>,
            Metadata<'a>, // ,
            Tz1PathDelayExpression<'a>,
            Metadata<'a>, // ,
            T1zPathDelayExpression<'a>,
            Metadata<'a>, // ,
            Tz0PathDelayExpression<'a>,
        )>,
    ),
    // Nest statements to avoid compiler issues with long tuples
    EdgeZX(
        Box<(
            (
                T01PathDelayExpression<'a>,
                Metadata<'a>, // ,
                T10PathDelayExpression<'a>,
                Metadata<'a>, // ,
                T0zPathDelayExpression<'a>,
                Metadata<'a>, // ,
                Tz1PathDelayExpression<'a>,
                Metadata<'a>, // ,
                T1zPathDelayExpression<'a>,
                Metadata<'a>, // ,
                Tz0PathDelayExpression<'a>,
                Metadata<'a>, // ,
            ),
            (
                T0xPathDelayExpression<'a>,
                Metadata<'a>, // ,
                Tx1PathDelayExpression<'a>,
                Metadata<'a>, // ,
                T1xPathDelayExpression<'a>,
                Metadata<'a>, // ,
                Tx0PathDelayExpression<'a>,
                Metadata<'a>, // ,
                TxzPathDelayExpression<'a>,
                Metadata<'a>, // ,
                TzxPathDelayExpression<'a>,
            ),
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TrisePathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TfallPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TzPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct T01PathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct T10PathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct T0zPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct Tz1PathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct T1zPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct Tz0PathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct T0xPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct Tx1PathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct T1xPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct Tx0PathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TxzPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TzxPathDelayExpression<'a>(pub PathDelayExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PathDelayExpression<'a>(pub ConstantMintypmaxExpression<'a>);
