// =======================================================================
// interface_items.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.6
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceOrGenerateItem<'a> {
    ModuleCommon(Box<(Vec<AttributeInstance<'a>>, ModuleCommonItem<'a>)>),
    ExternTf(Box<(Vec<AttributeInstance<'a>>, ExternTfDeclaration<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExternTfDeclaration<'a> {
    Method(
        Box<(
            Metadata<'a>, // exterm
            MethodPrototype<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Task(
        Box<(
            Metadata<'a>, // extern
            Metadata<'a>, // forkjoin
            TaskPrototype<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceItem<'a> {
    Port(
        Box<(
            PortDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    NonPort(Box<NonPortInterfaceItem<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NonPortInterfaceItem<'a> {
    Generate(Box<GenerateRegion<'a>>),
    InterfaceOrGenerate(Box<InterfaceOrGenerateItem<'a>>),
    Program(Box<ProgramDeclaration<'a>>),
    Modport(Box<ModportDeclaration<'a>>),
    Interface(Box<InterfaceDeclaration<'a>>),
    Timeunits(Box<TimeunitsDeclaration<'a>>),
}
