// =======================================================================
// task_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.7
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TaskDeclaration<'a>(
    pub Metadata<'a>, // task
    pub Option<DynamicOverrideSpecifiers<'a>>,
    pub Option<Lifetime<'a>>,
    pub TaskBodyDeclaration<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceIdentifierOrClassScope<'a> {
    InterfaceIdentifier(
        Box<(
            InterfaceIdentifier<'a>,
            Metadata<'a>, // .
        )>,
    ),
    ClassScope(Box<ClassScope<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TaskBodyDeclaration<'a> {
    Tf(
        Box<(
            Option<InterfaceIdentifierOrClassScope<'a>>,
            TaskIdentifier<'a>,
            Metadata<'a>, // ;
            Vec<TfItemDeclaration<'a>>,
            Vec<StatementOrNull<'a>>,
            Metadata<'a>, // endtask
            Option<(
                Metadata<'a>, // :
                TaskIdentifier<'a>,
            )>,
        )>,
    ),
    Block(
        Box<(
            Option<InterfaceIdentifierOrClassScope<'a>>,
            TaskIdentifier<'a>,
            Metadata<'a>, // (
            Option<TfPortList<'a>>,
            Metadata<'a>, // )
            Metadata<'a>, // ;
            Vec<BlockItemDeclaration<'a>>,
            Vec<StatementOrNull<'a>>,
            Metadata<'a>, // endtask
            Option<(
                Metadata<'a>, // :
                TaskIdentifier<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TfItemDeclaration<'a> {
    Block(Box<BlockItemDeclaration<'a>>),
    Tf(Box<TfPortDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TfPortList<'a>(
    pub TfPortItem<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        TfPortItem<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct TfPortItem<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<TfPortDirection<'a>>,
    pub Option<Metadata<'a>>, // var
    pub DataTypeOrImplicit<'a>,
    pub  Option<(
        PortIdentifier<'a>,
        Vec<VariableDimension<'a>>,
        Option<(
            Metadata<'a>, // eq
            Expression<'a>,
        )>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum TfPortDirection<'a> {
    Port(Box<PortDirection<'a>>),
    Ref(
        Box<(
            Option<Metadata<'a>>, // const
            Metadata<'a>,         // ref
            Option<Metadata<'a>>, // static
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TfPortDeclaration<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub TfPortDirection<'a>,
    pub Option<Metadata<'a>>, // var
    pub DataTypeOrImplicit<'a>,
    pub ListOfTfVariableIdentifiers<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct TaskPrototype<'a>(
    pub Metadata<'a>, // task
    pub Option<DynamicOverrideSpecifiers<'a>>,
    pub TaskIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<TfPortList<'a>>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicOverrideSpecifiers<'a>(
    pub Option<InitialOrExtendsSpecifier<'a>>,
    pub Option<FinalSpecifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum InitialOrExtendsSpecifier<'a> {
    Initial((Metadata<'a>, Metadata<'a>)),
    Extends((Metadata<'a>, Metadata<'a>)),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalSpecifier<'a>(
    pub Metadata<'a>, // :
    pub Metadata<'a>, // final
);
