// =======================================================================
// function_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.6
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionDataTypeOrImplicit<'a> {
    DataType(Box<DataTypeOrVoid<'a>>),
    Implicit(Box<ImplicitDataType<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclaration<'a>(
    pub Metadata<'a>, // function
    pub Option<DynamicOverrideSpecifiers<'a>>,
    pub Option<Lifetime<'a>>,
    pub FunctionBodyDeclaration<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionBodyDeclaration<'a> {
    Tf(
        Box<(
            FunctionDataTypeOrImplicit<'a>,
            Option<InterfaceIdentifierOrClassScope<'a>>,
            FunctionIdentifier<'a>,
            Metadata<'a>, // ;
            Vec<TfItemDeclaration<'a>>,
            Vec<FunctionStatementOrNull<'a>>,
            Metadata<'a>, // endfunction
            Option<(
                Metadata<'a>, // :
                FunctionIdentifier<'a>,
            )>,
        )>,
    ),
    Block(
        Box<(
            FunctionDataTypeOrImplicit<'a>,
            Option<InterfaceIdentifierOrClassScope<'a>>,
            FunctionIdentifier<'a>,
            Metadata<'a>, // (
            Option<TfPortList<'a>>,
            Metadata<'a>, // )
            Metadata<'a>, // ;
            Vec<BlockItemDeclaration<'a>>,
            Vec<FunctionStatementOrNull<'a>>,
            Metadata<'a>, // endfunction
            Option<(
                Metadata<'a>, // :
                FunctionIdentifier<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionPrototype<'a>(
    pub Metadata<'a>, // function
    pub Option<DynamicOverrideSpecifiers<'a>>,
    pub DataTypeOrVoid<'a>,
    pub FunctionIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<TfPortList<'a>>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum DpiImportExport<'a> {
    FunctionImport(
        Box<(
            Metadata<'a>, // import
            DpiSpecString<'a>,
            Option<DpiFunctionImportProperty<'a>>,
            Option<(
                CIdentifier<'a>,
                Metadata<'a>, // =
            )>,
            DpiFunctionProto<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    TaskImport(
        Box<(
            Metadata<'a>, // import
            DpiSpecString<'a>,
            Option<DpiTaskImportProperty<'a>>,
            Option<(
                CIdentifier<'a>,
                Metadata<'a>, // =
            )>,
            DpiTaskProto<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    FunctionExport(
        Box<(
            Metadata<'a>, // export
            DpiSpecString<'a>,
            Option<(
                CIdentifier<'a>,
                Metadata<'a>, // =
            )>,
            Metadata<'a>, // function
            FunctionIdentifier<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    TaskExport(
        Box<(
            Metadata<'a>, // export
            DpiSpecString<'a>,
            Option<(
                CIdentifier<'a>,
                Metadata<'a>, // =
            )>,
            Metadata<'a>, // task
            TaskIdentifier<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DpiSpecString<'a> {
    DpiC(Metadata<'a>),
    Dpi(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DpiFunctionImportProperty<'a> {
    Context(Metadata<'a>),
    Pure(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DpiTaskImportProperty<'a>(
    pub Metadata<'a>, // context
);

#[derive(Clone, Debug, PartialEq)]
pub struct DpiFunctionProto<'a>(pub FunctionPrototype<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct DpiTaskProto<'a>(pub TaskPrototype<'a>);
