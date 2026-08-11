// =======================================================================
// module_instantiation.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.4.1.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleInstantiation<'a>(
    pub ModuleIdentifier<'a>,
    pub Option<ParameterValueAssignment<'a>>,
    pub HierarchicalInstance<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        HierarchicalInstance<'a>,
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterValueAssignment<'a>(
    pub Metadata<'a>, // #
    pub Metadata<'a>, // (
    pub ListOfParameterValueAssignments<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub enum ListOfParameterValueAssignments<'a> {
    Ordered(
        Box<(
            OrderedParameterAssignment<'a>,
            Vec<(
                Metadata<'a>, // ,
                OrderedParameterAssignment<'a>,
            )>,
        )>,
    ),
    Named(
        Box<(
            NamedParameterAssignment<'a>,
            Vec<(
                Metadata<'a>, // ,
                NamedParameterAssignment<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderedParameterAssignment<'a>(pub ParamExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct NamedParameterAssignment<'a>(
    pub Metadata<'a>, // .
    pub ParameterIdentifier<'a>,
    pub Metadata<'a>, // (
    pub Option<ParamExpression<'a>>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalInstance<'a>(
    pub NameOfInstance<'a>,
    pub Metadata<'a>, // (
    pub Option<ListOfPortConnections<'a>>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct NameOfInstance<'a>(
    pub InstanceIdentifier<'a>,
    pub Vec<UnpackedDimension<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ListOfPortConnections<'a> {
    Ordered(
        Box<(
            OrderedPortConnection<'a>,
            Vec<(
                Metadata<'a>, // ,
                OrderedPortConnection<'a>,
            )>,
        )>,
    ),
    Named(
        Box<(
            NamedPortConnection<'a>,
            Vec<(
                Metadata<'a>, // ,
                NamedPortConnection<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderedPortConnection<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<Expression<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum NamedPortConnection<'a> {
    Named(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // .
            PortIdentifier<'a>,
            Option<(
                Metadata<'a>, // (
                Option<Expression<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Wildcard(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // .
            Metadata<'a>, // *
        )>,
    ),
}
