// =======================================================================
// timing_control_statements.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.5
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ProceduralTimingControlStatement<'a>(
    pub ProceduralTimingControl<'a>,
    pub StatementOrNull<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum DelayOrEventControl<'a> {
    Delay(Box<DelayControl<'a>>),
    Event(Box<EventControl<'a>>),
    Repeat(
        Box<(
            Metadata<'a>, // repeat
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            EventControl<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DelayControl<'a> {
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
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventControl<'a> {
    Clocking(Box<ClockingEvent<'a>>),
    Wildcard(
        Box<(
            Metadata<'a>, // @
            Metadata<'a>, // *
        )>,
    ),
    ParenWildcard(
        Box<(
            Metadata<'a>, // @
            Metadata<'a>, // (
            Metadata<'a>, // *
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClockingEvent<'a> {
    Ps(
        Box<(
            Metadata<'a>, // @
            PsIdentifier<'a>,
        )>,
    ),
    Hierarchical(
        Box<(
            Metadata<'a>, // @
            HierarchicalIdentifier<'a>,
        )>,
    ),
    Expression(
        Box<(
            Metadata<'a>, // @
            Metadata<'a>, // (
            EventExpression<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventExpression<'a> {
    Trigger(
        Box<(
            Option<EdgeIdentifier<'a>>,
            Expression<'a>,
            Option<(
                Metadata<'a>, // iff
                Expression<'a>,
            )>,
        )>,
    ),
    Sequence(
        Box<(
            SequenceInstance<'a>,
            Option<(
                Metadata<'a>, // iff
                Expression<'a>,
            )>,
        )>,
    ),
    Or(
        Box<(
            EventExpression<'a>,
            Metadata<'a>, // or
            EventExpression<'a>,
        )>,
    ),
    Comma(
        Box<(
            EventExpression<'a>,
            Metadata<'a>, // ,
            EventExpression<'a>,
        )>,
    ),
    Paren(
        Box<(
            Metadata<'a>, // (
            EventExpression<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProceduralTimingControl<'a> {
    Delay(Box<DelayControl<'a>>),
    Event(Box<EventControl<'a>>),
    Cycle(Box<CycleDelay<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JumpStatement<'a> {
    Return(
        Box<(
            Metadata<'a>, // return
            Option<Expression<'a>>,
            Metadata<'a>, // ;
        )>,
    ),
    Break(
        Box<(
            Metadata<'a>, // break
            Metadata<'a>, // ;
        )>,
    ),
    Continue(
        Box<(
            Metadata<'a>, // continue
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum WaitStatement<'a> {
    Expression(
        Box<(
            Metadata<'a>, // wait
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            StatementOrNull<'a>,
        )>,
    ),
    Fork(
        Box<(
            Metadata<'a>, // wait
            Metadata<'a>, // fork
            Metadata<'a>, // ;
        )>,
    ),
    Order(
        Box<(
            Metadata<'a>, // wait_order
            Metadata<'a>, // (
            HierarchicalIdentifier<'a>,
            Vec<(
                Metadata<'a>, // ,
                HierarchicalIdentifier<'a>,
            )>,
            Metadata<'a>, // )
            ActionBlock<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventTrigger<'a> {
    Blocking(
        Box<(
            Metadata<'a>, // ->
            HierarchicalEventIdentifier<'a>,
            NonrangeSelect<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Nonblocking(
        Box<(
            Metadata<'a>, // ->>
            Option<DelayOrEventControl<'a>>,
            HierarchicalEventIdentifier<'a>,
            NonrangeSelect<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DisableStatement<'a> {
    Task(
        Box<(
            Metadata<'a>, // disable
            HierarchicalTaskIdentifier<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Block(
        Box<(
            Metadata<'a>, // disable
            HierarchicalBlockIdentifier<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Fork(
        Box<(
            Metadata<'a>, // disable
            Metadata<'a>, // fork
            Metadata<'a>, // ;
        )>,
    ),
}
