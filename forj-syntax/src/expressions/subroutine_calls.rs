// =======================================================================
// subroutine_calls.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.8.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantFunctionCall<'a>(pub FunctionSubroutineCall<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TfCall<'a>(
    pub PsOrHierarchicalTfIdentifier<'a>,
    pub Vec<AttributeInstance<'a>>,
    pub  Option<(
        Metadata<'a>, // (
        ListOfArguments<'a>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum SystemTfCall<'a> {
    Args(
        Box<(
            SystemTfIdentifier<'a>,
            Option<(
                Metadata<'a>, // (
                ListOfArguments<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Data(
        Box<(
            SystemTfIdentifier<'a>,
            Metadata<'a>, // (
            DataType<'a>,
            Option<(
                Metadata<'a>, // ,
                Expression<'a>,
            )>,
            Metadata<'a>, // )
        )>,
    ),
    Expressions(
        Box<(
            SystemTfIdentifier<'a>,
            Metadata<'a>, // (
            Expression<'a>,
            Vec<(
                Metadata<'a>, // ,
                Option<Expression<'a>>,
            )>,
            Option<(
                Metadata<'a>, // ,
                Option<ClockingEvent<'a>>,
            )>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SubroutineCall<'a> {
    Tf(Box<TfCall<'a>>),
    SystemTf(Box<SystemTfCall<'a>>),
    Method(Box<MethodCall<'a>>),
    Randomize(
        Box<(
            Option<(
                Metadata<'a>, // std
                Metadata<'a>, // ::
            )>,
            RandomizeCall<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSubroutineCall<'a>(pub SubroutineCall<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum ListOfArguments<'a> {
    Expressions(
        Box<(
            Option<Expression<'a>>,
            Vec<(
                Metadata<'a>, // ,
                Option<Expression<'a>>,
            )>,
            Vec<(
                Metadata<'a>, // ,
                Metadata<'a>, // .
                Identifier<'a>,
                Metadata<'a>, // (
                Option<Expression<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    NoExpressions(
        Box<(
            Metadata<'a>, // .
            Identifier<'a>,
            Metadata<'a>, // (
            Option<Expression<'a>>,
            Metadata<'a>, // )
            Vec<(
                Metadata<'a>, // ,
                Metadata<'a>, // .
                Identifier<'a>,
                Metadata<'a>, // (
                Option<Expression<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodCall<'a>(
    pub MethodCallRoot<'a>,
    pub Metadata<'a>, // .
    pub MethodCallBody<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum MethodCallBody<'a> {
    Custom(
        Box<(
            MethodIdentifier<'a>,
            Vec<AttributeInstance<'a>>,
            Option<(
                Metadata<'a>, // (
                ListOfArguments<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    BuiltIn(Box<BuiltInMethodCall<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuiltInMethodCall<'a> {
    ArrayManip(Box<ArrayManipulationCall<'a>>),
    Randomize(Box<RandomizeCall<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayManipulationCall<'a>(
    pub ArrayMethodName<'a>,
    pub Vec<AttributeInstance<'a>>,
    pub  Option<(
        Metadata<'a>, // (
        ListOfArguments<'a>,
        Metadata<'a>, // )
    )>,
    pub  Option<(
        Metadata<'a>, // with
        Metadata<'a>, // (
        Expression<'a>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum VariableIdentifierListOrNull<'a> {
    VariableIdentifierList(Box<VariableIdentifierList<'a>>),
    Null(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RandomizeCall<'a>(
    pub Metadata<'a>, // randomize
    pub Vec<AttributeInstance<'a>>,
    pub  Option<(
        Metadata<'a>, // (
        Option<VariableIdentifierListOrNull<'a>>,
        Metadata<'a>, // )
    )>,
    pub  Option<(
        Metadata<'a>, // with
        Option<(
            Metadata<'a>, // (
            Option<IdentifierList<'a>>,
            Metadata<'a>, // )
        )>,
        ConstraintBlock<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct VariableIdentifierList<'a>(
    pub VariableIdentifier<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        VariableIdentifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierList<'a>(
    pub Identifier<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        Identifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum MethodCallRoot<'a> {
    Primary(Box<Primary<'a>>),
    ImplicitClassHandle(Box<ImplicitClassHandle<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArrayMethodName<'a> {
    Method(Box<MethodIdentifier<'a>>),
    Unique(Box<Metadata<'a>>),
    And(Box<Metadata<'a>>),
    Or(Box<Metadata<'a>>),
    Xor(Box<Metadata<'a>>),
}
