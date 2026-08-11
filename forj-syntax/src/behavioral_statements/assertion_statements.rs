// =======================================================================
// assertion_statements.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.10
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AssertionItem<'a> {
    Concurrent(Box<ConcurrentAssertionItem<'a>>),
    DeferredImmediate(Box<DeferredImmediateAssertionItem<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredImmediateAssertionItem<'a>(
    pub  Option<(
        BlockIdentifier<'a>,
        Metadata<'a>, // ;
    )>,
    pub DeferredImmediateAssertionStatement<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ProceduralAssertionStatement<'a> {
    Concurrent(Box<ConcurrentAssertionStatement<'a>>),
    Immediate(Box<ImmediateAssertionStatement<'a>>),
    Checker(Box<CheckerInstantiation<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImmediateAssertionStatement<'a> {
    Simple(Box<SimpleImmediateAssertionStatement<'a>>),
    Deferred(Box<DeferredImmediateAssertionStatement<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SimpleImmediateAssertionStatement<'a> {
    Assert(Box<SimpleImmediateAssertStatement<'a>>),
    Assume(Box<SimpleImmediateAssumeStatement<'a>>),
    Cover(Box<SimpleImmediateCoverStatement<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeferredImmediateAssertionStatement<'a> {
    Assert(Box<DeferredImmediateAssertStatement<'a>>),
    Assume(Box<DeferredImmediateAssumeStatement<'a>>),
    Cover(Box<DeferredImmediateCoverStatement<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleImmediateAssertStatement<'a>(
    pub Metadata<'a>, // assert
    pub Metadata<'a>, // (
    pub Expression<'a>,
    pub Metadata<'a>, // )
    pub ActionBlock<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleImmediateAssumeStatement<'a>(
    pub Metadata<'a>, // assume
    pub Metadata<'a>, // (
    pub Expression<'a>,
    pub Metadata<'a>, // )
    pub ActionBlock<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleImmediateCoverStatement<'a>(
    pub Metadata<'a>, // cover
    pub Metadata<'a>, // (
    pub Expression<'a>,
    pub Metadata<'a>, // )
    pub StatementOrNull<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum DeferredImmediateAssertStatement<'a> {
    Now(
        Box<(
            Metadata<'a>, // assert
            Metadata<'a>, // #
            Metadata<'a>, // 0
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            ActionBlock<'a>,
        )>,
    ),
    Final(
        Box<(
            Metadata<'a>, // assert
            Metadata<'a>, // final
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            ActionBlock<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeferredImmediateAssumeStatement<'a> {
    Now(
        Box<(
            Metadata<'a>, // assume
            Metadata<'a>, // #
            Metadata<'a>, // 0
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            ActionBlock<'a>,
        )>,
    ),
    Final(
        Box<(
            Metadata<'a>, // assume
            Metadata<'a>, // final
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            ActionBlock<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeferredImmediateCoverStatement<'a> {
    Now(
        Box<(
            Metadata<'a>, // cover
            Metadata<'a>, // #
            Metadata<'a>, // 0
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            StatementOrNull<'a>,
        )>,
    ),
    Final(
        Box<(
            Metadata<'a>, // cover
            Metadata<'a>, // final
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            StatementOrNull<'a>,
        )>,
    ),
}
