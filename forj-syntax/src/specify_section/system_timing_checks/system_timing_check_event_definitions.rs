// =======================================================================
// specify_timing_check_event_definitions.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.7.5.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TimingCheckEvent<'a>(
    pub Option<TimingCheckEventControl<'a>>,
    pub SpecifyTerminalDescriptor<'a>,
    pub  Option<(
        Metadata<'a>, // &&&
        TimingCheckCondition<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ControlledTimingCheckEvent<'a>(
    pub TimingCheckEventControl<'a>,
    pub SpecifyTerminalDescriptor<'a>,
    pub  Option<(
        Metadata<'a>, // &&&
        TimingCheckCondition<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum TimingCheckEventControl<'a> {
    Posedge(Box<Metadata<'a>>),
    Negedge(Box<Metadata<'a>>),
    Edge(Box<Metadata<'a>>),
    EdgeControl(Box<EdgeControlSpecifier<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpecifyTerminalDescriptor<'a> {
    Input(Box<SpecifyInputTerminalDescriptor<'a>>),
    Output(Box<SpecifyOutputTerminalDescriptor<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeControlSpecifier<'a>(
    pub Metadata<'a>, // edge
    pub Metadata<'a>, // [
    pub EdgeDescriptor<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        EdgeDescriptor<'a>,
    )>,
    pub Metadata<'a>, // ]
);

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeDescriptor<'a> {
    ZeroOne(Box<Metadata<'a>>),
    OneZero(Box<Metadata<'a>>),
    ZeroX(Box<(Metadata<'a>, Metadata<'a>)>),
    OneX(Box<(Metadata<'a>, Metadata<'a>)>),
    XZero(Box<Metadata<'a>>),
    XOne(Box<Metadata<'a>>),
    ZeroZ(Box<(Metadata<'a>, Metadata<'a>)>),
    OneZ(Box<(Metadata<'a>, Metadata<'a>)>),
    ZZero(Box<Metadata<'a>>),
    ZOne(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimingCheckCondition<'a> {
    NoParen(Box<ScalarTimingCheckCondition<'a>>),
    Paren(
        Box<(
            Metadata<'a>, // (
            ScalarTimingCheckCondition<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScalarTimingCheckCondition<'a> {
    Base(Box<Expression<'a>>),
    Invert(Box<(Metadata<'a>, Expression<'a>)>),
    EqEq(
        Box<(
            Expression<'a>,
            Metadata<'a>, // ==
            ScalarConstant<'a>,
        )>,
    ),
    EqEqEq(
        Box<(
            Expression<'a>,
            Metadata<'a>, // ===
            ScalarConstant<'a>,
        )>,
    ),
    ExclEq(
        Box<(
            Expression<'a>,
            Metadata<'a>, // !=
            ScalarConstant<'a>,
        )>,
    ),
    ExclEqEq(
        Box<(
            Expression<'a>,
            Metadata<'a>, // !==
            ScalarConstant<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScalarConstant<'a> {
    Sizedb0(Metadata<'a>),
    Sizedb1(Metadata<'a>),
    SizedB0(Metadata<'a>),
    SizedB1(Metadata<'a>),
    Unsizedb0(Metadata<'a>),
    Unsizedb1(Metadata<'a>),
    UnsizedB0(Metadata<'a>),
    UnsizedB1(Metadata<'a>),
    One(Metadata<'a>),
    Zero(Metadata<'a>),
}
