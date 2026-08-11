// =======================================================================
// primitive_gate_and_switch_types.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.3.4
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CmosSwitchtype<'a> {
    Cmos(Metadata<'a>),
    Rcmos(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnableGatetype<'a> {
    Bufif0(Metadata<'a>),
    Bufif1(Metadata<'a>),
    Notif0(Metadata<'a>),
    Notif1(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MosSwitchtype<'a> {
    Nmos(Metadata<'a>),
    Pmos(Metadata<'a>),
    Rnmos(Metadata<'a>),
    Rpmos(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NInputGatetype<'a> {
    And(Metadata<'a>),
    Nand(Metadata<'a>),
    Or(Metadata<'a>),
    Nor(Metadata<'a>),
    Xor(Metadata<'a>),
    Xnor(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NOutputGatetype<'a> {
    Buf(Metadata<'a>),
    Not(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PassEnSwitchtype<'a> {
    Tranif0(Metadata<'a>),
    Tranif1(Metadata<'a>),
    Rtranif0(Metadata<'a>),
    Rtranif1(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PassSwitchtype<'a> {
    Tran(Metadata<'a>),
    Rtran(Metadata<'a>),
}
