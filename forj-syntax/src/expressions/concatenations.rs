// =======================================================================
// concatenations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.8.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Concatenation<'a>(
    pub Metadata<'a>, // {
    pub Expression<'a>,
    pub Vec<(Metadata<'a>, Expression<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantConcatenation<'a>(
    pub Metadata<'a>, // {
    pub ConstantExpression<'a>,
    pub Vec<(Metadata<'a>, ConstantExpression<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantMultipleConcatenation<'a>(
    pub Metadata<'a>, // {
    pub ConstantExpression<'a>,
    pub ConstantConcatenation<'a>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModulePathConcatenation<'a>(
    pub Metadata<'a>, // {
    pub ModulePathExpression<'a>,
    pub Vec<(Metadata<'a>, ModulePathExpression<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModulePathMultipleConcatenation<'a>(
    pub Metadata<'a>, // {
    pub ConstantExpression<'a>,
    pub ModulePathConcatenation<'a>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct MultipleConcatenation<'a>(
    pub Metadata<'a>, // {
    pub Expression<'a>,
    pub Concatenation<'a>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct StreamingConcatenation<'a>(
    pub Metadata<'a>, // {
    pub StreamOperator<'a>,
    pub Option<SliceSize<'a>>,
    pub StreamConcatenation<'a>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub enum StreamOperator<'a> {
    Right(Metadata<'a>),
    Left(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SliceSize<'a> {
    Simple(Box<SimpleType<'a>>),
    ConstExpr(Box<ConstantExpression<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StreamConcatenation<'a>(
    pub Metadata<'a>, // {
    pub StreamExpression<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        StreamExpression<'a>,
    )>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct StreamExpression<'a>(
    pub Expression<'a>,
    pub  Option<(
        Metadata<'a>, // with
        Metadata<'a>, // [
        ArrayRangeExpression<'a>,
        Metadata<'a>, // ]
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ArrayRangeExpression<'a> {
    Select(Box<Expression<'a>>),
    Range(
        Box<(
            Expression<'a>,
            Metadata<'a>, // :
            Expression<'a>,
        )>,
    ),
    PlusRange(
        Box<(
            Expression<'a>,
            Metadata<'a>, // +:
            Expression<'a>,
        )>,
    ),
    MinusRange(
        Box<(
            Expression<'a>,
            Metadata<'a>, // -:
            Expression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyUnpackedArrayConcatenation<'a>(
    pub Metadata<'a>, // {
    pub Metadata<'a>, // }
);
