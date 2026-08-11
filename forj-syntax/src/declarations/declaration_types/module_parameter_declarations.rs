// =======================================================================
// module_parameter_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.1.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum LocalParameterDeclaration<'a> {
    DataParameter(
        Metadata<'a>, // localparam
        DataTypeOrImplicit<'a>,
        ListOfParamAssignments<'a>,
    ),
    TypeParameter(
        Metadata<'a>, // localparam
        TypeParameterDeclaration<'a>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterDeclaration<'a> {
    DataParameter(
        Metadata<'a>, // parameter
        DataTypeOrImplicit<'a>,
        ListOfParamAssignments<'a>,
    ),
    TypeParameter(
        Metadata<'a>, // parameter
        TypeParameterDeclaration<'a>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeParameterDeclaration<'a>(
    pub Metadata<'a>, // type
    pub Option<ForwardType<'a>>,
    pub ListOfTypeAssignments<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct SpecparamDeclaration<'a>(
    pub Metadata<'a>, // specparam
    pub Option<PackedDimension<'a>>,
    pub ListOfSpecparamAssignments<'a>,
    pub Metadata<'a>, // ;
);
