// =======================================================================
// strengths.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.2.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Delay2<'a> {
    Value(
        Box<(
            Metadata<'a>, // #
            DelayValue<'a>,
        )>,
    ),
    Mintypmax(
        Box<(
            Metadata<'a>, // #
            Metadata<'a>, // (
            MintypmaxExpression<'a>,
            Option<(
                Metadata<'a>, // ,
                MintypmaxExpression<'a>,
            )>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Delay3<'a> {
    Value(
        Box<(
            Metadata<'a>, // #
            DelayValue<'a>,
        )>,
    ),
    Mintypmax(
        Box<(
            Metadata<'a>, // #
            Metadata<'a>, // (
            MintypmaxExpression<'a>,
            Option<(
                Metadata<'a>, // ,
                MintypmaxExpression<'a>,
                Option<(
                    Metadata<'a>, // ,
                    MintypmaxExpression<'a>,
                )>,
            )>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DelayValue<'a> {
    Unsigned(UnsignedNumber<'a>),
    Real(RealNumber<'a>),
    Ps(PsIdentifier<'a>),
    Time(TimeLiteral<'a>),
    OneStep(Metadata<'a>),
}
