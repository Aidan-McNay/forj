// =======================================================================
// expression_left_side_values.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.8.5
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum NetLvalue<'a> {
    Selection(Box<SelectionNetLvalue<'a>>),
    Nested(Box<NestedNetLvalue<'a>>),
    Assignment(Box<AssignmentNetLvalue<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectionNetLvalue<'a>(
    pub PsOrHierarchicalNetIdentifier<'a>,
    pub ConstantSelect<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct NestedNetLvalue<'a>(
    pub Metadata<'a>, // {
    pub NetLvalue<'a>,
    pub Vec<(Metadata<'a>, NetLvalue<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentNetLvalue<'a>(
    pub Option<AssignmentPatternExpressionType<'a>>,
    pub AssignmentPatternNetLvalue<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum VariableLvalue<'a> {
    Selection(Box<SelectionVariableLvalue<'a>>),
    Nested(Box<NestedVariableLvalue<'a>>),
    Assignment(Box<AssignmentVariableLvalue<'a>>),
    Streaming(Box<StreamingConcatenation<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImplicitClassHandleOrPackageScope<'a> {
    ImplicitClassHandle(Box<(ImplicitClassHandle<'a>, Metadata<'a>)>),
    PackageScope(Box<PackageScope<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectionVariableLvalue<'a>(
    pub Option<ImplicitClassHandleOrPackageScope<'a>>,
    pub HierarchicalVariableIdentifier<'a>,
    pub Select<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct NestedVariableLvalue<'a>(
    pub Metadata<'a>, // {
    pub VariableLvalue<'a>,
    pub Vec<(Metadata<'a>, VariableLvalue<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentVariableLvalue<'a>(
    pub Option<AssignmentPatternExpressionType<'a>>,
    pub AssignmentPatternVariableLvalue<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct NonrangeVariableLvalue<'a>(
    pub Option<ImplicitClassHandleOrPackageScope<'a>>,
    pub HierarchicalVariableIdentifier<'a>,
    pub NonrangeSelect<'a>,
);
