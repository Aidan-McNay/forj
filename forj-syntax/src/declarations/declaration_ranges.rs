// =======================================================================
// declaration_ranges.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.5
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum UnpackedDimension<'a> {
    UnpackedRange(
        Box<(
            Metadata<'a>, // [
            ConstantRange<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    UnpackedExpression(
        Box<(
            Metadata<'a>, // [
            ConstantExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PackedDimension<'a> {
    PackedRange(
        Box<(
            Metadata<'a>, // [
            ConstantRange<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    UnsizedDimension(Box<UnsizedDimension<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssociativeDimension<'a> {
    Data(
        Box<(
            Metadata<'a>, // [
            DataType<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    Star(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // *
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum VariableDimension<'a> {
    UnsizedDimension(Box<UnsizedDimension<'a>>),
    UnpackedDimension(Box<UnpackedDimension<'a>>),
    AssociativeDimension(Box<AssociativeDimension<'a>>),
    QueueDimension(Box<QueueDimension<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct QueueDimension<'a>(
    pub Metadata<'a>, // [
    pub Metadata<'a>, // $
    pub Option<(Metadata<'a>, ConstantExpression<'a>)>,
    pub Metadata<'a>, // ]
);

#[derive(Clone, Debug, PartialEq)]
pub struct UnsizedDimension<'a>(
    pub Metadata<'a>, // [
    pub Metadata<'a>, // ]
);
