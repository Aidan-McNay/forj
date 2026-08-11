// =======================================================================
// interface_instantiation.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.4.1.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceInstantiation<'a>(
    pub InterfaceIdentifier<'a>,
    pub Option<ParameterValueAssignment<'a>>,
    pub HierarchicalInstance<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        HierarchicalInstance<'a>,
    )>,
    pub Metadata<'a>, // ;
);
