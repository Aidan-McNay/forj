// =======================================================================
// udp_ports.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.5.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UdpPortList<'a>(
    pub OutputPortIdentifier<'a>,
    pub Metadata<'a>, // ,
    pub InputPortIdentifier<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        InputPortIdentifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct UdpDeclarationPortList<'a>(
    pub UdpOutputDeclaration<'a>,
    pub Metadata<'a>, // ,
    pub UdpInputDeclaration<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        UdpInputDeclaration<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum UdpPortDeclaration<'a> {
    Output(
        Box<(
            UdpOutputDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Input(
        Box<(
            UdpInputDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Reg(
        Box<(
            UdpRegDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum UdpOutputDeclaration<'a> {
    Combinational(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // output
            PortIdentifier<'a>,
        )>,
    ),
    Sequential(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // output
            Metadata<'a>, // reg
            PortIdentifier<'a>,
            Option<(
                Metadata<'a>, // =
                ConstantExpression<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UdpInputDeclaration<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // input
    pub ListOfUdpPortIdentifiers<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct UdpRegDeclaration<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // reg
    pub VariableIdentifier<'a>,
);
