// =======================================================================
// looping_statements.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.8
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum LoopStatement<'a> {
    Forever(
        Box<(
            Metadata<'a>, // forever
            StatementOrNull<'a>,
        )>,
    ),
    Repeat(
        Box<(
            Metadata<'a>, // repeat
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            StatementOrNull<'a>,
        )>,
    ),
    While(
        Box<(
            Metadata<'a>, // while
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            StatementOrNull<'a>,
        )>,
    ),
    For(
        Box<(
            Metadata<'a>, // for
            Metadata<'a>, // (
            Option<ForInitialization<'a>>,
            Metadata<'a>, // ;
            Option<Expression<'a>>,
            Metadata<'a>, // ;
            Option<ForStep<'a>>,
            Metadata<'a>, // )
            StatementOrNull<'a>,
        )>,
    ),
    DoWhile(
        Box<(
            Metadata<'a>, // do
            StatementOrNull<'a>,
            Metadata<'a>, // while
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            Metadata<'a>, // ;
        )>,
    ),
    Foreach(
        Box<(
            Metadata<'a>, // foreach
            Metadata<'a>, // (
            PsOrHierarchicalArrayIdentifier<'a>,
            Metadata<'a>, // [
            LoopVariables<'a>,
            Metadata<'a>, // ]
            Metadata<'a>, // )
            Statement<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForInitialization<'a> {
    VariableAssignment(Box<ListOfVariableAssignments<'a>>),
    VariableDeclarations(
        Box<(
            ForVariableDeclaration<'a>,
            Vec<(
                Metadata<'a>, // ,
                ForVariableDeclaration<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForVariableDeclaration<'a>(
    pub Option<Metadata<'a>>, // var
    pub DataType<'a>,
    pub VariableIdentifier<'a>,
    pub Metadata<'a>, // =
    pub Expression<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        VariableIdentifier<'a>,
        Metadata<'a>, // =
        Expression<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ForStep<'a>(
    pub ForStepAssignment<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        ForStepAssignment<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ForStepAssignment<'a> {
    Operator(Box<OperatorAssignment<'a>>),
    IncOrDec(Box<IncOrDecExpression<'a>>),
    Function(Box<FunctionSubroutineCall<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoopVariables<'a>(
    pub Option<IndexVariableIdentifier<'a>>,
    pub  Vec<(
        Metadata<'a>, // ,
        Option<IndexVariableIdentifier<'a>>,
    )>,
);
