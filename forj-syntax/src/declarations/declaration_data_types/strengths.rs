// =======================================================================
// strengths.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.2.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum DriveStrength<'a> {
    S0S1(
        Metadata<'a>, // (
        Strength0<'a>,
        Metadata<'a>, // ,
        Strength1<'a>,
        Metadata<'a>, // )
    ),
    S1S0(
        Metadata<'a>, // (
        Strength1<'a>,
        Metadata<'a>, // ,
        Strength0<'a>,
        Metadata<'a>, // )
    ),
    S0Z1(
        Metadata<'a>, // (
        Strength0<'a>,
        Metadata<'a>, // ,
        Metadata<'a>, // highz1
        Metadata<'a>, // )
    ),
    S1Z0(
        Metadata<'a>, // (
        Strength1<'a>,
        Metadata<'a>, // ,
        Metadata<'a>, // highz0
        Metadata<'a>, // )
    ),
    Z0S1(
        Metadata<'a>, // (
        Metadata<'a>, // highz0
        Metadata<'a>, // ,
        Strength1<'a>,
        Metadata<'a>, // )
    ),
    Z1S0(
        Metadata<'a>, // (
        Metadata<'a>, // highz1
        Metadata<'a>, // ,
        Strength0<'a>,
        Metadata<'a>, // )
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Strength0<'a> {
    Supply0(Metadata<'a>),
    Strong0(Metadata<'a>),
    Pull0(Metadata<'a>),
    Weak0(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Strength1<'a> {
    Supply1(Metadata<'a>),
    Strong1(Metadata<'a>),
    Pull1(Metadata<'a>),
    Weak1(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChargeStrength<'a>(
    pub Metadata<'a>, // (
    pub ChargeStrengthSize<'a>,
    pub Metadata<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ChargeStrengthSize<'a> {
    Small(Metadata<'a>),
    Medium(Metadata<'a>),
    Large(Metadata<'a>),
}
