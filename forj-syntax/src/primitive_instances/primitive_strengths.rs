// =======================================================================
// primitive_strengths.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.3.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PulldownStrength<'a> {
    S0S1(
        Box<(
            Metadata<'a>,
            Strength0<'a>,
            Metadata<'a>,
            Strength1<'a>,
            Metadata<'a>,
        )>,
    ),
    S1S0(
        Box<(
            Metadata<'a>,
            Strength1<'a>,
            Metadata<'a>,
            Strength0<'a>,
            Metadata<'a>,
        )>,
    ),
    S0(Box<(Metadata<'a>, Strength0<'a>, Metadata<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PullupStrength<'a> {
    S0S1(
        Box<(
            Metadata<'a>,
            Strength0<'a>,
            Metadata<'a>,
            Strength1<'a>,
            Metadata<'a>,
        )>,
    ),
    S1S0(
        Box<(
            Metadata<'a>,
            Strength1<'a>,
            Metadata<'a>,
            Strength0<'a>,
            Metadata<'a>,
        )>,
    ),
    S1(Box<(Metadata<'a>, Strength1<'a>, Metadata<'a>)>),
}
