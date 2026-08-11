// =======================================================================
// checker_items.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.8
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerPortList<'a>(
    pub CheckerPortItem<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        CheckerPortItem<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerPortItem<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<CheckerPortDirection<'a>>,
    pub PropertyFormalType<'a>,
    pub FormalPortIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub  Option<(
        Metadata<'a>, // =
        PropertyActualArg<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum CheckerPortDirection<'a> {
    Input(Metadata<'a>),
    Output(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CheckerOrGenerateItem<'a> {
    Declaration(Box<CheckerOrGenerateItemDeclaration<'a>>),
    Initial(Box<InitialConstruct<'a>>),
    Always(Box<AlwaysConstruct<'a>>),
    Final(Box<FinalConstruct<'a>>),
    Assertion(Box<AssertionItem<'a>>),
    Assign(Box<ContinuousAssign<'a>>),
    Generate(Box<CheckerGenerateItem<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CheckerOrGenerateItemDeclaration<'a> {
    Data(
        Box<(
            Option<Metadata<'a>>, // rand
            DataDeclaration<'a>,
        )>,
    ),
    Function(Box<FunctionDeclaration<'a>>),
    Checker(Box<CheckerDeclaration<'a>>),
    AssertionItem(Box<AssertionItemDeclaration<'a>>),
    Covergroup(Box<CovergroupDeclaration<'a>>),
    Genvar(Box<GenvarDeclaration<'a>>),
    Clocking(Box<ClockingDeclaration<'a>>),
    DefaultClocking(
        Box<(
            Metadata<'a>, // default
            Metadata<'a>, // clocking
            ClockingIdentifier<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    DefaultDisable(
        Box<(
            Metadata<'a>, // default
            Metadata<'a>, // disable
            Metadata<'a>, // iff
            ExpressionOrDist<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Null(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CheckerGenerateItem<'a> {
    Loop(Box<LoopGenerateConstruct<'a>>),
    Conditional(Box<ConditionalGenerateConstruct<'a>>),
    Region(Box<GenerateRegion<'a>>),
    ElaborationSeverity(Box<ElaborationSeveritySystemTask<'a>>),
}
