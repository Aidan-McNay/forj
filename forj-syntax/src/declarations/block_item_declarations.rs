// =======================================================================
// block_item_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.8
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockItemDeclaration<'a> {
    Data(Box<(Vec<AttributeInstance<'a>>, DataDeclaration<'a>)>),
    LocalParameter(
        Box<(Vec<AttributeInstance<'a>>, LocalParameterDeclaration<'a>)>,
    ),
    Parameter(Box<(Vec<AttributeInstance<'a>>, ParameterDeclaration<'a>)>),
    Let(Box<(Vec<AttributeInstance<'a>>, LetDeclaration<'a>)>),
}
