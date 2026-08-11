// =======================================================================
// checker_instantiation.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.4.1.4
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerInstantiation<'a>(
    pub PsCheckerIdentifier<'a>,
    pub NameOfInstance<'a>,
    pub Metadata<'a>, // (
    pub Option<ListOfCheckerPortConnections<'a>>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum ListOfCheckerPortConnections<'a> {
    Ordered(
        Box<(
            OrderedCheckerPortConnection<'a>,
            Vec<(
                Metadata<'a>, // ,
                OrderedCheckerPortConnection<'a>,
            )>,
        )>,
    ),
    Named(
        Box<(
            NamedCheckerPortConnection<'a>,
            Vec<(
                Metadata<'a>, // ,
                NamedCheckerPortConnection<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderedCheckerPortConnection<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<PropertyActualArg<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum NamedCheckerPortConnection<'a> {
    Identifier(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // .
            FormalPortIdentifier<'a>,
            Option<(
                Metadata<'a>, // (
                Option<PropertyActualArg<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Wildcard(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // .
            Metadata<'a>, // *
        )>,
    ),
}
