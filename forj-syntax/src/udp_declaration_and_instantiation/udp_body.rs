// =======================================================================
// udp_body.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.5.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum UdpBody<'a> {
    Combinational(Box<CombinationalBody<'a>>),
    Sequential(Box<SequentialBody<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CombinationalBody<'a>(
    pub Metadata<'a>, // table
    pub CombinationalEntry<'a>,
    pub Vec<CombinationalEntry<'a>>,
    pub Metadata<'a>, // endtable
);

#[derive(Clone, Debug, PartialEq)]
pub struct CombinationalEntry<'a>(
    pub LevelInputList<'a>,
    pub Metadata<'a>, // :
    pub OutputSymbol<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct SequentialBody<'a>(
    pub Option<UdpInitialStatement<'a>>,
    pub Metadata<'a>, // table
    pub SequentialEntry<'a>,
    pub Vec<SequentialEntry<'a>>,
    pub Metadata<'a>, // endtable
);

#[derive(Clone, Debug, PartialEq)]
pub struct UdpInitialStatement<'a>(
    pub Metadata<'a>, // initial
    pub OutputPortIdentifier<'a>,
    pub Metadata<'a>, // =
    pub InitVal<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum InitVal<'a> {
    LittleB0(Metadata<'a>),
    LittleB1(Metadata<'a>),
    LittleBx(Metadata<'a>),
    LittleBX(Metadata<'a>),
    BigB0(Metadata<'a>),
    BigB1(Metadata<'a>),
    BigBx(Metadata<'a>),
    BigBX(Metadata<'a>),
    One(Metadata<'a>),
    Zero(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequentialEntry<'a>(
    pub SeqInputList<'a>,
    pub Metadata<'a>, // :
    pub CurrentState<'a>,
    pub Metadata<'a>, // :
    pub NextState<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum SeqInputList<'a> {
    Level(Box<LevelInputList<'a>>),
    Edge(Box<EdgeInputList<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LevelInputList<'a>(pub LevelSymbol<'a>, pub Vec<LevelSymbol<'a>>);

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeInputList<'a>(
    pub Vec<LevelSymbol<'a>>,
    pub EdgeIndicator<'a>,
    pub Vec<LevelSymbol<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeIndicator<'a> {
    Explicit(
        Box<(
            Metadata<'a>, // (
            LevelSymbol<'a>,
            LevelSymbol<'a>,
            Metadata<'a>, // )
        )>,
    ),
    Shorthand(Box<EdgeSymbol<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CurrentState<'a>(pub LevelSymbol<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum NextState<'a> {
    Output(Box<OutputSymbol<'a>>),
    Minus(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OutputSymbol<'a> {
    Zero(Metadata<'a>),
    One(Metadata<'a>),
    LittleX(Metadata<'a>),
    BigX(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LevelSymbol<'a> {
    Zero(Metadata<'a>),
    One(Metadata<'a>),
    LittleX(Metadata<'a>),
    BigX(Metadata<'a>),
    Quest(Metadata<'a>),
    LittleB(Metadata<'a>),
    BigB(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeSymbol<'a> {
    LittleR(Metadata<'a>),
    BigR(Metadata<'a>),
    LittleF(Metadata<'a>),
    BigF(Metadata<'a>),
    LittleP(Metadata<'a>),
    BigP(Metadata<'a>),
    LittleN(Metadata<'a>),
    BigN(Metadata<'a>),
    Star(Metadata<'a>),
}
