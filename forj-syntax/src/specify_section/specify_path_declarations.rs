// =======================================================================
// specify_path_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.7.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PathDeclaration<'a> {
    Simple(
        Box<(
            SimplePathDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    EdgeSensitive(
        Box<(
            EdgeSensitivePathDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    StateDependent(
        Box<(
            StateDependentPathDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SimplePathDeclaration<'a> {
    Parallel(
        Box<(
            ParallelPathDescription<'a>,
            Metadata<'a>, // =
            PathDelayValue<'a>,
        )>,
    ),
    Full(
        Box<(
            FullPathDescription<'a>,
            Metadata<'a>, // =
            PathDelayValue<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParallelPathDescription<'a>(
    pub Metadata<'a>, // (
    pub SpecifyInputTerminalDescriptor<'a>,
    pub Option<PolarityOperator<'a>>,
    pub Metadata<'a>, // =>
    pub SpecifyOutputTerminalDescriptor<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct FullPathDescription<'a>(
    pub Metadata<'a>, // (
    pub ListOfPathInputs<'a>,
    pub Option<PolarityOperator<'a>>,
    pub Metadata<'a>, // *>
    pub ListOfPathOutputs<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeSensitivePathDeclaration<'a> {
    Parallel(
        Box<(
            ParallelEdgeSensitivePathDescription<'a>,
            Metadata<'a>, // =
            PathDelayValue<'a>,
        )>,
    ),
    Full(
        Box<(
            FullEdgeSensitivePathDescription<'a>,
            Metadata<'a>, // =
            PathDelayValue<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParallelEdgeSensitivePathDescription<'a> {
    DataSource(
        Box<(
            Metadata<'a>, // (
            Option<EdgeIdentifier<'a>>,
            SpecifyInputTerminalDescriptor<'a>,
            Option<PolarityOperator<'a>>,
            Metadata<'a>, // =>
            Metadata<'a>, // (
            SpecifyOutputTerminalDescriptor<'a>,
            Option<PolarityOperator<'a>>,
            Metadata<'a>, // :
            DataSourceExpression<'a>,
            Metadata<'a>, // )
            Metadata<'a>, // )
        )>,
    ),
    NoDataSource(
        Box<(
            Metadata<'a>, // (
            Option<EdgeIdentifier<'a>>,
            SpecifyInputTerminalDescriptor<'a>,
            Option<PolarityOperator<'a>>,
            Metadata<'a>, // =>
            SpecifyOutputTerminalDescriptor<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FullEdgeSensitivePathDescription<'a> {
    DataSource(
        Box<(
            Metadata<'a>, // (
            Option<EdgeIdentifier<'a>>,
            ListOfPathInputs<'a>,
            Option<PolarityOperator<'a>>,
            Metadata<'a>, // *>
            Metadata<'a>, // (
            ListOfPathOutputs<'a>,
            Option<PolarityOperator<'a>>,
            Metadata<'a>, // :
            DataSourceExpression<'a>,
            Metadata<'a>, // )
            Metadata<'a>, // )
        )>,
    ),
    NoDataSource(
        Box<(
            Metadata<'a>, // (
            Option<EdgeIdentifier<'a>>,
            ListOfPathInputs<'a>,
            Option<PolarityOperator<'a>>,
            Metadata<'a>, // *>
            ListOfPathOutputs<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StateDependentPathDeclaration<'a> {
    Simple(
        Box<(
            Metadata<'a>, // if
            Metadata<'a>, // (
            ModulePathExpression<'a>,
            Metadata<'a>, // )
            SimplePathDeclaration<'a>,
        )>,
    ),
    EdgeSensitive(
        Box<(
            Metadata<'a>, // if
            Metadata<'a>, // (
            ModulePathExpression<'a>,
            Metadata<'a>, // )
            EdgeSensitivePathDeclaration<'a>,
        )>,
    ),
    NoCondition(
        Box<(
            Metadata<'a>, // ifnone
            SimplePathDeclaration<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataSourceExpression<'a>(pub Expression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeIdentifier<'a> {
    Posedge(Metadata<'a>),
    Negedge(Metadata<'a>),
    Edge(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PolarityOperator<'a> {
    Plus(Metadata<'a>),
    Minus(Metadata<'a>),
}
