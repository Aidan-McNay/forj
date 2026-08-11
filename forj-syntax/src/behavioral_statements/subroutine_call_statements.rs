// =======================================================================
// subroutine_call_statements.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.9
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum SubroutineCallStatement<'a> {
    Subroutine(
        Box<(
            SubroutineCall<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Void(
        Box<(
            Metadata<'a>, // void
            Metadata<'a>, // '
            Metadata<'a>, // (
            FunctionSubroutineCall<'a>,
            Metadata<'a>, // )
        )>,
    ),
}
