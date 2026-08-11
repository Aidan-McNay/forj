// =======================================================================
// specify_block_terminals.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.7.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfPathInputs<'a>(
    pub SpecifyInputTerminalDescriptor<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        SpecifyInputTerminalDescriptor<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfPathOutputs<'a>(
    pub SpecifyOutputTerminalDescriptor<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        SpecifyOutputTerminalDescriptor<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct SpecifyInputTerminalDescriptor<'a>(
    pub InputIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // [
        ConstantRangeExpression<'a>,
        Metadata<'a>, // ]
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct SpecifyOutputTerminalDescriptor<'a>(
    pub OutputIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // [
        ConstantRangeExpression<'a>,
        Metadata<'a>, // ]
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum InputIdentifier<'a> {
    Input(Box<InputPortIdentifier<'a>>),
    Inout(Box<InoutPortIdentifier<'a>>),
    Interface(
        Box<(
            InterfaceIdentifier<'a>,
            Metadata<'a>, // .
            PortIdentifier<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OutputIdentifier<'a> {
    Output(Box<OutputPortIdentifier<'a>>),
    Inout(Box<InoutPortIdentifier<'a>>),
    Interface(
        Box<(
            InterfaceIdentifier<'a>,
            Metadata<'a>, // .
            PortIdentifier<'a>,
        )>,
    ),
}
