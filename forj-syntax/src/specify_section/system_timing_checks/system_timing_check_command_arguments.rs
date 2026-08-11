// =======================================================================
// specify_timing_check_command_arguments.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.7.5.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ControlledReferenceEvent<'a>(pub ControlledTimingCheckEvent<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct DataEvent<'a>(pub TimingCheckEvent<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum DelayedData<'a> {
    Identifier(Box<TerminalIdentifier<'a>>),
    Slice(
        Box<(
            TerminalIdentifier<'a>,
            Metadata<'a>, // [
            ConstantMintypmaxExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DelayedReference<'a> {
    Identifier(Box<TerminalIdentifier<'a>>),
    Slice(
        Box<(
            TerminalIdentifier<'a>,
            Metadata<'a>, // [
            ConstantMintypmaxExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndEdgeOffset<'a>(pub MintypmaxExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct EventBasedFlag<'a>(pub ConstantExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct Notifier<'a>(pub VariableIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceEvent<'a>(pub TimingCheckEvent<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct RemainActiveFlag<'a>(pub ConstantMintypmaxExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TimecheckCondition<'a>(pub MintypmaxExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TimestampCondition<'a>(pub MintypmaxExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct StartEdgeOffset<'a>(pub MintypmaxExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct Threshold<'a>(pub ConstantExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TimingCheckLimit<'a>(pub Expression<'a>);
