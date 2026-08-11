// =======================================================================
// specify_block_declaration.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.7.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SpecifyBlock<'a>(
    pub Metadata<'a>, // specify
    pub Vec<SpecifyItem<'a>>,
    pub Metadata<'a>, // endspecify
);

#[derive(Clone, Debug, PartialEq)]
pub enum SpecifyItem<'a> {
    Specparam(Box<SpecparamDeclaration<'a>>),
    Pulsestyle(Box<PulsestyleDeclaration<'a>>),
    Showcancelled(Box<ShowcancelledDeclaration<'a>>),
    Path(Box<PathDeclaration<'a>>),
    SystemTiming(Box<SystemTimingCheck<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PulsestyleDeclaration<'a> {
    Onevent(
        Box<(
            Metadata<'a>, // pulsestyle_onevent
            ListOfPathOutputs<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Ondetect(
        Box<(
            Metadata<'a>, // pulsestyle_ondetect
            ListOfPathOutputs<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShowcancelledDeclaration<'a> {
    Show(
        Box<(
            Metadata<'a>, // showcancelled
            ListOfPathOutputs<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Noshow(
        Box<(
            Metadata<'a>, // noshowcancelled
            ListOfPathOutputs<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}
