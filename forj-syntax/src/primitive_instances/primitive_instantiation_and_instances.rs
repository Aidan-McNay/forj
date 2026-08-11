// =======================================================================
// primitive_strengths.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.3.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum GateInstantiation<'a> {
    Cmos(
        Box<(
            CmosSwitchtype<'a>,
            Option<Delay3<'a>>,
            CmosSwitchInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                CmosSwitchInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    Mos(
        Box<(
            MosSwitchtype<'a>,
            Option<Delay3<'a>>,
            MosSwitchInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                MosSwitchInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    Enable(
        Box<(
            EnableGatetype<'a>,
            Option<DriveStrength<'a>>,
            Option<Delay3<'a>>,
            EnableGateInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                EnableGateInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    NInput(
        Box<(
            NInputGatetype<'a>,
            Option<DriveStrength<'a>>,
            Option<Delay2<'a>>,
            NInputGateInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                NInputGateInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    NOutput(
        Box<(
            NOutputGatetype<'a>,
            Option<DriveStrength<'a>>,
            Option<Delay2<'a>>,
            NOutputGateInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                NOutputGateInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    PassEn(
        Box<(
            PassEnSwitchtype<'a>,
            Option<Delay2<'a>>,
            PassEnableSwitchInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                PassEnableSwitchInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    Pass(
        Box<(
            PassSwitchtype<'a>,
            PassSwitchInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                PassSwitchInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    Pulldown(
        Box<(
            Metadata<'a>, // pulldown
            Option<PulldownStrength<'a>>,
            PullGateInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                PullGateInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    Pullup(
        Box<(
            Metadata<'a>, // pullup
            Option<PullupStrength<'a>>,
            PullGateInstance<'a>,
            Vec<(
                Metadata<'a>, // ,
                PullGateInstance<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CmosSwitchInstance<'a>(
    pub Option<NameOfInstance<'a>>,
    pub Metadata<'a>, // (
    pub OutputTerminal<'a>,
    pub Metadata<'a>, // ,
    pub InputTerminal<'a>,
    pub Metadata<'a>, // ,
    pub NcontrolTerminal<'a>,
    pub Metadata<'a>, // ,
    pub PcontrolTerminal<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct EnableGateInstance<'a>(
    pub Option<NameOfInstance<'a>>,
    pub Metadata<'a>, // (
    pub OutputTerminal<'a>,
    pub Metadata<'a>, // ,
    pub InputTerminal<'a>,
    pub Metadata<'a>, // ,
    pub EnableTerminal<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct MosSwitchInstance<'a>(
    pub Option<NameOfInstance<'a>>,
    pub Metadata<'a>, // (
    pub OutputTerminal<'a>,
    pub Metadata<'a>, // ,
    pub InputTerminal<'a>,
    pub Metadata<'a>, // ,
    pub EnableTerminal<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct NInputGateInstance<'a>(
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

#[derive(Clone, Debug, PartialEq)]
pub struct NOutputGateInstance<'a>(
    pub Option<NameOfInstance<'a>>,
    pub Metadata<'a>, // (
    pub OutputTerminal<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        OutputTerminal<'a>,
    )>,
    pub Metadata<'a>, // ,
    pub InputTerminal<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct PassSwitchInstance<'a>(
    pub Option<NameOfInstance<'a>>,
    pub Metadata<'a>, // (
    pub InoutTerminal<'a>,
    pub Metadata<'a>, // ,
    pub InoutTerminal<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct PassEnableSwitchInstance<'a>(
    pub Option<NameOfInstance<'a>>,
    pub Metadata<'a>, // (
    pub InoutTerminal<'a>,
    pub Metadata<'a>, // ,
    pub InoutTerminal<'a>,
    pub Metadata<'a>, // ,
    pub EnableTerminal<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct PullGateInstance<'a>(
    pub Option<NameOfInstance<'a>>,
    pub Metadata<'a>, // (
    pub OutputTerminal<'a>,
    pub Metadata<'a>, // )
);
