// =======================================================================
// statements.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.4
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum StatementOrNull<'a> {
    Statement(Box<Statement<'a>>),
    Null(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<'a>(
    pub  Option<(
        BlockIdentifier<'a>,
        Metadata<'a>, // :
    )>,
    pub Vec<AttributeInstance<'a>>,
    pub StatementItem<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum StatementItem<'a> {
    Blocking(
        Box<(
            BlockingAssignment<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Nonblocking(
        Box<(
            NonblockingAssignment<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    ProceduralContinuous(
        Box<(
            ProceduralContinuousAssignment<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Case(Box<CaseStatement<'a>>),
    Conditional(Box<ConditionalStatement<'a>>),
    SubroutineCall(Box<SubroutineCallStatement<'a>>),
    Disable(Box<DisableStatement<'a>>),
    Event(Box<EventTrigger<'a>>),
    Loop(Box<LoopStatement<'a>>),
    Jump(Box<JumpStatement<'a>>),
    Par(Box<ParBlock<'a>>),
    ProceduralTimingControl(Box<ProceduralTimingControlStatement<'a>>),
    Seq(Box<SeqBlock<'a>>),
    Wait(Box<WaitStatement<'a>>),
    ProceduralAssertion(Box<ProceduralAssertionStatement<'a>>),
    Clocking(
        Box<(
            ClockingDrive<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Randsequence(Box<RandsequenceStatement<'a>>),
    Randcase(Box<RandcaseStatement<'a>>),
    Expect(Box<ExpectPropertyStatement<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionStatement<'a>(pub Statement<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionStatementOrNull<'a> {
    FunctionStatement(Box<FunctionStatement<'a>>),
    Null(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // ;
        )>,
    ),
}
