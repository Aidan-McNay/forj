// =======================================================================
// class_items.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.9
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ClassItem<'a> {
    Property(Box<(Vec<AttributeInstance<'a>>, ClassProperty<'a>)>),
    Method(Box<(Vec<AttributeInstance<'a>>, ClassMethod<'a>)>),
    Constraint(Box<(Vec<AttributeInstance<'a>>, ClassConstraint<'a>)>),
    Declaration(Box<(Vec<AttributeInstance<'a>>, ClassDeclaration<'a>)>),
    InterfaceDeclaration(
        Box<(Vec<AttributeInstance<'a>>, InterfaceClassDeclaration<'a>)>,
    ),
    Covergroup(Box<(Vec<AttributeInstance<'a>>, CovergroupDeclaration<'a>)>),
    LocalParameter(
        Box<(
            LocalParameterDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Parameter(
        Box<(
            ParameterDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Null(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassProperty<'a> {
    Data(Box<(Vec<PropertyQualifier<'a>>, DataDeclaration<'a>)>),
    Const(
        Box<(
            Metadata<'a>, // const
            Vec<ClassItemQualifier<'a>>,
            DataType<'a>,
            ConstIdentifier<'a>,
            Option<(
                Metadata<'a>, // =
                ConstantExpression<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassMethod<'a> {
    Task(Box<(Vec<MethodQualifier<'a>>, TaskDeclaration<'a>)>),
    Function(Box<(Vec<MethodQualifier<'a>>, FunctionDeclaration<'a>)>),
    PureVirtualMethod(
        Box<(
            Metadata<'a>, // pure
            Metadata<'a>, // virtual
            Vec<ClassItemQualifier<'a>>,
            MethodPrototype<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    ExternMethod(
        Box<(
            Metadata<'a>, // extern
            Vec<MethodQualifier<'a>>,
            MethodPrototype<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    ConstructorDeclaration(
        Box<(Vec<MethodQualifier<'a>>, ClassConstructorDeclaration<'a>)>,
    ),
    ConstructorPrototype(
        Box<(
            Metadata<'a>, // extern
            Vec<MethodQualifier<'a>>,
            ClassConstructorPrototype<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListOfArgumentsOrDefault<'a> {
    ListOfArguments(Box<ListOfArguments<'a>>),
    Default(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassConstructorDeclaration<'a>(
    pub Metadata<'a>, // function
    pub Option<ClassScope<'a>>,
    pub Metadata<'a>, // new
    pub  Option<(
        Metadata<'a>, // (
        Option<ClassConstructorArgList<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
    pub Vec<BlockItemDeclaration<'a>>,
    pub  Option<(
        Metadata<'a>, // super
        Metadata<'a>, // .
        Metadata<'a>, // new
        Option<(
            Metadata<'a>, // (
            Option<ListOfArgumentsOrDefault<'a>>,
            Metadata<'a>, // )
        )>,
        Metadata<'a>, // ;
    )>,
    pub Vec<FunctionStatementOrNull<'a>>,
    pub Metadata<'a>, // endfunction
    pub  Option<(
        Metadata<'a>, // :
        Metadata<'a>, // new
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassConstructorPrototype<'a>(
    pub Metadata<'a>, // function
    pub Metadata<'a>, // new
    pub  Option<(
        Metadata<'a>, // (
        Option<ClassConstructorArgList<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassConstructorArgList<'a>(
    pub ClassConstructorArg<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        ClassConstructorArg<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ClassConstructorArg<'a> {
    TfPort(Box<TfPortItem<'a>>),
    Default(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceClassItem<'a> {
    Type(Box<TypeDeclaration<'a>>),
    InterfaceClass(Box<(Vec<AttributeInstance<'a>>, InterfaceClassMethod<'a>)>),
    LocalParameter(
        Box<(
            LocalParameterDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Parameter(
        Box<(
            ParameterDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Null(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceClassMethod<'a>(
    pub Metadata<'a>, // pure
    pub Metadata<'a>, // virtual
    pub MethodPrototype<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum ClassConstraint<'a> {
    Prototype(Box<ConstraintPrototype<'a>>),
    Declaration(Box<ConstraintDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassItemQualifier<'a> {
    Static(Metadata<'a>),
    Protected(Metadata<'a>),
    Local(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyQualifier<'a> {
    Random(Box<RandomQualifier<'a>>),
    ClassItem(Box<ClassItemQualifier<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RandomQualifier<'a> {
    Rand(Metadata<'a>),
    Randc(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MethodQualifier<'a> {
    PureVirtual(
        Box<(
            Option<Metadata<'a>>, // pure
            Metadata<'a>,         // virtual
        )>,
    ),
    ClassItem(Box<ClassItemQualifier<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MethodPrototype<'a> {
    Task(Box<TaskPrototype<'a>>),
    Function(Box<FunctionPrototype<'a>>),
}
