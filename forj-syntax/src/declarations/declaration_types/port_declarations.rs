// =======================================================================
// port_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.1.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InoutDeclaration<'a>(
    pub Metadata<'a>, // inout
    pub NetPortType<'a>,
    pub ListOfPortIdentifiers<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum InputDeclaration<'a> {
    NetInputDeclaration(
        Box<(Metadata<'a>, NetPortType<'a>, ListOfPortIdentifiers<'a>)>,
    ),
    VariableInputDeclaration(
        Box<(
            Metadata<'a>,
            VariablePortType<'a>,
            ListOfVariableIdentifiers<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OutputDeclaration<'a> {
    NetOutputDeclaration(
        Box<(Metadata<'a>, NetPortType<'a>, ListOfPortIdentifiers<'a>)>,
    ),
    VariableOutputDeclaration(
        Box<(
            Metadata<'a>,
            VariablePortType<'a>,
            ListOfVariablePortIdentifiers<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterfacePortDeclaration<'a> {
    Interface(Box<(InterfaceIdentifier<'a>, ListOfInterfaceIdentifiers<'a>)>),
    Modport(
        Box<(
            InterfaceIdentifier<'a>,
            Metadata<'a>, // .
            ModportIdentifier<'a>,
            ListOfInterfaceIdentifiers<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RefDeclaration<'a>(
    pub Metadata<'a>, // ref
    pub VariablePortType<'a>,
    pub ListOfVariableIdentifiers<'a>,
);
