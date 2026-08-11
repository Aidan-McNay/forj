// =======================================================================
// clocking_block.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.11
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ClockingDeclaration<'a> {
    Local(
        Box<(
            Option<Metadata<'a>>, // default
            Metadata<'a>,         // clocking
            Option<ClockingIdentifier<'a>>,
            ClockingEvent<'a>,
            Metadata<'a>, // ;
            Vec<ClockingItem<'a>>,
            Metadata<'a>, // endclocking
            Option<(
                Metadata<'a>, // :
                ClockingIdentifier<'a>,
            )>,
        )>,
    ),
    Global(
        Box<(
            Metadata<'a>, // global
            Metadata<'a>, // clocking
            Option<ClockingIdentifier<'a>>,
            ClockingEvent<'a>,
            Metadata<'a>, // ;
            Vec<ClockingItem<'a>>,
            Metadata<'a>, // endclocking
            Option<(
                Metadata<'a>, // :
                ClockingIdentifier<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClockingItem<'a> {
    Default(
        Box<(
            Metadata<'a>, // default
            DefaultSkew<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Decl(
        Box<(
            ClockingDirection<'a>,
            ListOfClockingDeclAssign<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Assert(Box<(Vec<AttributeInstance<'a>>, AssertionItemDeclaration<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultSkew<'a> {
    Input(
        Box<(
            Metadata<'a>, // input
            ClockingSkew<'a>,
        )>,
    ),
    Output(
        Box<(
            Metadata<'a>, // output
            ClockingSkew<'a>,
        )>,
    ),
    InputOutput(
        Box<(
            Metadata<'a>, // input
            ClockingSkew<'a>,
            Metadata<'a>, // output
            ClockingSkew<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClockingDirection<'a> {
    Input(
        Box<(
            Metadata<'a>, // input
            Option<ClockingSkew<'a>>,
        )>,
    ),
    Output(
        Box<(
            Metadata<'a>, // output
            Option<ClockingSkew<'a>>,
        )>,
    ),
    InputOutput(
        Box<(
            Metadata<'a>, // input
            Option<ClockingSkew<'a>>,
            Metadata<'a>, // output
            Option<ClockingSkew<'a>>,
        )>,
    ),
    Inout(
        Box<
            Metadata<'a>, // inout
        >,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfClockingDeclAssign<'a>(
    pub ClockingDeclAssign<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        ClockingDeclAssign<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClockingDeclAssign<'a>(
    pub SignalIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // =
        Expression<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ClockingSkew<'a> {
    Edge(Box<(EdgeIdentifier<'a>, Option<DelayControl<'a>>)>),
    Delay(Box<DelayControl<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClockingDrive<'a>(
    pub ClockvarExpression<'a>,
    pub Metadata<'a>, // <=
    pub Option<CycleDelay<'a>>,
    pub Expression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum CycleDelay<'a> {
    Integral(
        Box<(
            Metadata<'a>, // ##
            IntegralNumber<'a>,
        )>,
    ),
    Identifier(
        Box<(
            Metadata<'a>, // ##
            Identifier<'a>,
        )>,
    ),
    Expression(
        Box<(
            Metadata<'a>, // ##
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Clockvar<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ClockvarExpression<'a>(pub Clockvar<'a>, pub Select<'a>);
