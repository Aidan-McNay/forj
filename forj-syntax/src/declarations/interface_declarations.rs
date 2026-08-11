// =======================================================================
// interface_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.9
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ModportDeclaration<'a>(
    pub Metadata<'a>, // modport
    pub ModportItem<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        ModportItem<'a>,
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModportItem<'a>(
    pub ModportIdentifier<'a>,
    pub Metadata<'a>, // (
    pub ModportPortsDeclaration<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        ModportPortsDeclaration<'a>,
    )>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub enum ModportPortsDeclaration<'a> {
    Simple(
        Box<(
            Vec<AttributeInstance<'a>>,
            ModportSimplePortsDeclaration<'a>,
        )>,
    ),
    Tf(Box<(Vec<AttributeInstance<'a>>, ModportTfPortsDeclaration<'a>)>),
    Clocking(Box<(Vec<AttributeInstance<'a>>, ModportClockingDeclaration<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModportClockingDeclaration<'a>(
    pub Metadata<'a>, // clocking
    pub ClockingIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModportSimplePortsDeclaration<'a>(
    pub PortDirection<'a>,
    pub ModportSimplePort<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        ModportSimplePort<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ModportSimplePort<'a> {
    Name(Box<PortIdentifier<'a>>),
    Expression(
        Box<(
            Metadata<'a>, // .
            PortIdentifier<'a>,
            Metadata<'a>, // (
            Option<Expression<'a>>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModportTfPortsDeclaration<'a>(
    pub ImportExport<'a>,
    pub ModportTfPort<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        ModportTfPort<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ModportTfPort<'a> {
    Method(Box<MethodPrototype<'a>>),
    Tf(Box<TfIdentifier<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImportExport<'a> {
    Import(Metadata<'a>),
    Export(Metadata<'a>),
}
