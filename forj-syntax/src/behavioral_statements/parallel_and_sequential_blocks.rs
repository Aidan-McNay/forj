// =======================================================================
// parallel_and_sequential_blocks.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionBlock<'a> {
    Basic(Box<StatementOrNull<'a>>),
    Conditional(
        Box<(
            Option<Statement<'a>>,
            Metadata<'a>, // else
            StatementOrNull<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SeqBlock<'a>(
    pub Metadata<'a>, // begin
    pub  Option<(
        Metadata<'a>, // :
        BlockIdentifier<'a>,
    )>,
    pub Vec<BlockItemDeclaration<'a>>,
    pub Vec<StatementOrNull<'a>>,
    pub Metadata<'a>, // end
    pub  Option<(
        Metadata<'a>, // :
        BlockIdentifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ParBlock<'a>(
    pub Metadata<'a>, // fork
    pub  Option<(
        Metadata<'a>, // :
        BlockIdentifier<'a>,
    )>,
    pub Vec<BlockItemDeclaration<'a>>,
    pub Vec<StatementOrNull<'a>>,
    pub JoinKeyword<'a>,
    pub  Option<(
        Metadata<'a>, // :
        BlockIdentifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum JoinKeyword<'a> {
    Join(Metadata<'a>),
    JoinAny(Metadata<'a>),
    JoinNone(Metadata<'a>),
}
