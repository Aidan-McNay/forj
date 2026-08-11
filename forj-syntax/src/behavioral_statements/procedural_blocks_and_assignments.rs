// =======================================================================
// procedural_blocks_and_assignments.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InitialConstruct<'a>(
    pub Metadata<'a>, // initial
    pub StatementOrNull<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct AlwaysConstruct<'a>(pub AlwaysKeyword<'a>, pub Statement<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum AlwaysKeyword<'a> {
    Always(Metadata<'a>),
    AlwaysComb(Metadata<'a>),
    AlwaysLatch(Metadata<'a>),
    AlwaysFf(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalConstruct<'a>(
    pub Metadata<'a>, // final
    pub FunctionStatement<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ImplicitClassHandleOrClassScopeOrPackageScope<'a> {
    ImplicitClassHandle(
        Box<(
            ImplicitClassHandle<'a>,
            Metadata<'a>, // .
        )>,
    ),
    Class(Box<ClassScope<'a>>),
    Package(Box<PackageScope<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockingAssignment<'a> {
    Variable(
        Box<(
            VariableLvalue<'a>,
            Metadata<'a>, // =
            DelayOrEventControl<'a>,
            Expression<'a>,
        )>,
    ),
    Dynamic(
        Box<(
            NonrangeVariableLvalue<'a>,
            Metadata<'a>, // =
            DynamicArrayNew<'a>,
        )>,
    ),
    Class(
        Box<(
            Option<ImplicitClassHandleOrClassScopeOrPackageScope<'a>>,
            HierarchicalVariableIdentifier<'a>,
            Select<'a>,
            Metadata<'a>, // =
            ClassNew<'a>,
        )>,
    ),
    Operator(Box<OperatorAssignment<'a>>),
    IncOrDec(Box<IncOrDecExpression<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct OperatorAssignment<'a>(
    pub VariableLvalue<'a>,
    pub AssignmentOperator<'a>,
    pub Expression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentOperator<'a> {
    Eq(Metadata<'a>),
    PlusEq(Metadata<'a>),
    MinusEq(Metadata<'a>),
    StarEq(Metadata<'a>),
    SlashEq(Metadata<'a>),
    PercentEq(Metadata<'a>),
    AmpEq(Metadata<'a>),
    PipeEq(Metadata<'a>),
    CaretEq(Metadata<'a>),
    LtLtEq(Metadata<'a>),
    GtGtEq(Metadata<'a>),
    LtLtLtEq(Metadata<'a>),
    GtGtGtEq(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonblockingAssignment<'a>(
    pub VariableLvalue<'a>,
    pub Metadata<'a>, // <=
    pub Option<DelayOrEventControl<'a>>,
    pub Expression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ProceduralContinuousAssignment<'a> {
    Assign(
        Box<(
            Metadata<'a>, // assign
            VariableAssignment<'a>,
        )>,
    ),
    Deassign(
        Box<(
            Metadata<'a>, // deassign
            VariableLvalue<'a>,
        )>,
    ),
    ForceVar(
        Box<(
            Metadata<'a>, // force
            VariableAssignment<'a>,
        )>,
    ),
    ForceNet(
        Box<(
            Metadata<'a>, // force
            NetAssignment<'a>,
        )>,
    ),
    ReleaseVar(
        Box<(
            Metadata<'a>, // release
            VariableLvalue<'a>,
        )>,
    ),
    ReleaseNet(
        Box<(
            Metadata<'a>, // release
            NetLvalue<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableAssignment<'a>(
    pub VariableLvalue<'a>,
    pub Metadata<'a>, // =
    pub Expression<'a>,
);
