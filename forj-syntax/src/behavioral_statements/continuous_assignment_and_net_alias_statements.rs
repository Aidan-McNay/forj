// =======================================================================
// case_statements.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ContinuousAssign<'a> {
    NetAssignments(
        Box<(
            Metadata<'a>, // assign
            Option<DriveStrength<'a>>,
            Option<Delay3<'a>>,
            ListOfNetAssignments<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    VariableAssignments(
        Box<(
            Metadata<'a>, // assign
            Option<DelayControl<'a>>,
            ListOfVariableAssignments<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfNetAssignments<'a>(
    pub NetAssignment<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        NetAssignment<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfVariableAssignments<'a>(
    pub VariableAssignment<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        VariableAssignment<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct NetAlias<'a>(
    pub Metadata<'a>, // alias
    pub NetLvalue<'a>,
    pub Metadata<'a>, // =
    pub NetLvalue<'a>,
    pub  Vec<(
        Metadata<'a>, // =
        NetLvalue<'a>,
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct NetAssignment<'a>(
    pub NetLvalue<'a>,
    pub Metadata<'a>, // =
    pub Expression<'a>,
);
