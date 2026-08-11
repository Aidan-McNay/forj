// =======================================================================
// primitive_terminals.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.3.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct EnableTerminal<'a>(pub Expression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InoutTerminal<'a>(pub NetLvalue<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InputTerminal<'a>(pub Expression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct NcontrolTerminal<'a>(pub Expression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct OutputTerminal<'a>(pub NetLvalue<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PcontrolTerminal<'a>(pub Expression<'a>);
