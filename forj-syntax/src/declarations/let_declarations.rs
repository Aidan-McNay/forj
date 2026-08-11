// =======================================================================
// let_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.12
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LetDeclaration<'a>(
    pub Metadata<'a>, // let
    pub LetIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<LetPortList<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // =
    pub Expression<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct LetIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct LetPortList<'a>(
    pub LetPortItem<'a>,
    pub Vec<(Metadata<'a>, LetPortItem<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct LetPortItem<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub LetFormalType<'a>,
    pub FormalPortIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub Option<(Metadata<'a>, Expression<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum LetFormalType<'a> {
    DataTypeOrImplicit(Box<DataTypeOrImplicit<'a>>),
    Untyped(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LetExpression<'a>(
    pub Option<PackageScope<'a>>,
    pub LetIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<LetListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct LetListOfPartialIdentifierArguments<'a>(
    pub Option<LetActualArg<'a>>,
    pub Vec<(Metadata<'a>, Option<LetActualArg<'a>>)>,
    pub  Vec<(
        Metadata<'a>, // ,
        Metadata<'a>, // .
        Identifier<'a>,
        Metadata<'a>, // (
        Option<LetActualArg<'a>>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct LetListOfIdentifierArguments<'a>(
    pub Metadata<'a>, // .
    pub Identifier<'a>,
    pub Metadata<'a>, // (
    pub Option<LetActualArg<'a>>,
    pub Metadata<'a>, // )
    pub  Vec<(
        Metadata<'a>, // ,
        Metadata<'a>, // .
        Identifier<'a>,
        Metadata<'a>, // (
        Option<LetActualArg<'a>>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum LetListOfArguments<'a> {
    PartialIdentifier(Box<LetListOfPartialIdentifierArguments<'a>>),
    Identifier(Box<LetListOfIdentifierArguments<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LetActualArg<'a>(pub Expression<'a>);
