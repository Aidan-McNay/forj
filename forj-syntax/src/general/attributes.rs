// =======================================================================
// attributes.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.9.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeInstance<'a>(
    pub Metadata<'a>, // (
    pub Metadata<'a>, // *
    pub AttrSpec<'a>,
    pub Vec<(Metadata<'a>, AttrSpec<'a>)>,
    pub Metadata<'a>, // *
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct AttrSpec<'a>(
    pub AttrName<'a>,
    pub Option<(Metadata<'a>, ConstantExpression<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct AttrName<'a>(pub Identifier<'a>);
