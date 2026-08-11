// =======================================================================
// package_items.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.11
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PackageItem<'a> {
    PackageOrGenerate(Box<PackageOrGenerateItemDeclaration<'a>>),
    AnonymousProgram(Box<AnonymousProgram<'a>>),
    PackageExport(Box<PackageExportDeclaration<'a>>),
    Timeunits(Box<TimeunitsDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PackageOrGenerateItemDeclaration<'a> {
    Net(Box<NetDeclaration<'a>>),
    Data(Box<DataDeclaration<'a>>),
    Task(Box<TaskDeclaration<'a>>),
    Function(Box<FunctionDeclaration<'a>>),
    Checker(Box<CheckerDeclaration<'a>>),
    DpiImportExport(Box<DpiImportExport<'a>>),
    ExternConstraint(Box<ExternConstraintDeclaration<'a>>),
    Class(Box<ClassDeclaration<'a>>),
    InterfaceClass(Box<InterfaceClassDeclaration<'a>>),
    ClassConstructor(Box<ClassConstructorDeclaration<'a>>),
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
    Covergroup(Box<CovergroupDeclaration<'a>>),
    AssertionItem(Box<AssertionItemDeclaration<'a>>),
    Null(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousProgram<'a>(
    pub Metadata<'a>, // program
    pub Metadata<'a>, // ;
    pub Vec<AnonymousProgramItem<'a>>,
    pub Metadata<'a>, // endprogram
);

#[derive(Clone, Debug, PartialEq)]
pub enum AnonymousProgramItem<'a> {
    Task(Box<TaskDeclaration<'a>>),
    Function(Box<FunctionDeclaration<'a>>),
    Class(Box<ClassDeclaration<'a>>),
    InterfaceClass(Box<InterfaceClassDeclaration<'a>>),
    Covergroup(Box<CovergroupDeclaration<'a>>),
    ClassConstructor(Box<ClassConstructorDeclaration<'a>>),
    Null(Box<Metadata<'a>>),
}
