// =======================================================================
// udp_instantiation.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.5.4
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UdpInstantiation<'a>(
    pub UdpIdentifier<'a>,
    pub Option<DriveStrength<'a>>,
    pub Option<Delay2<'a>>,
    pub UdpInstance<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        UdpInstance<'a>,
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct UdpInstance<'a>(
    pub Option<NameOfInstance<'a>>,
    pub Metadata<'a>, // (
    pub OutputTerminal<'a>,
    pub Metadata<'a>, // ,
    pub InputTerminal<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        InputTerminal<'a>,
    )>,
    pub Metadata<'a>, // )
);
