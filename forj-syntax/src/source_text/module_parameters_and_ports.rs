// =======================================================================
// module_parameters_and_ports.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterPortList<'a> {
    Defaults(
        Metadata<'a>, // #
        Metadata<'a>, // (
        ListOfParamAssignments<'a>,
        Vec<(Metadata<'a>, ParameterPortDeclaration<'a>)>,
        Metadata<'a>, // )
    ),
    NoDefaults(
        Metadata<'a>, // #
        Metadata<'a>, // (
        ParameterPortDeclaration<'a>,
        Vec<(Metadata<'a>, ParameterPortDeclaration<'a>)>,
        Metadata<'a>, // )
    ),
    Empty(Metadata<'a>, Metadata<'a>, Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterPortDeclaration<'a> {
    ParameterDeclaration(Box<ParameterDeclaration<'a>>),
    LocalParameterDeclaration(Box<LocalParameterDeclaration<'a>>),
    DataAssignments(Box<(DataType<'a>, ListOfParamAssignments<'a>)>),
    TypeParameterDeclaration(Box<TypeParameterDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfPorts<'a>(
    pub Metadata<'a>, // (
    pub Port<'a>,
    pub Vec<(Metadata<'a>, Port<'a>)>,
    pub Metadata<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfPortDeclarations<'a>(
    pub Metadata<'a>, // (
    pub  Option<(
        Vec<AttributeInstance<'a>>,
        AnsiPortDeclaration<'a>,
        Vec<(
            Metadata<'a>, // ,
            Vec<AttributeInstance<'a>>,
            AnsiPortDeclaration<'a>,
        )>,
    )>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub enum PortDeclaration<'a> {
    InoutDeclaration(Box<(Vec<AttributeInstance<'a>>, InoutDeclaration<'a>)>),
    InputDeclaration(Box<(Vec<AttributeInstance<'a>>, InputDeclaration<'a>)>),
    OutputDeclaration(Box<(Vec<AttributeInstance<'a>>, OutputDeclaration<'a>)>),
    RefDeclaration(Box<(Vec<AttributeInstance<'a>>, RefDeclaration<'a>)>),
    InterfacePortDeclaration(
        Box<(Vec<AttributeInstance<'a>>, InterfacePortDeclaration<'a>)>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Port<'a> {
    PortExpression(Box<Option<PortExpression<'a>>>),
    PortIdentifier(
        Box<(
            Metadata<'a>, // .
            PortIdentifier<'a>,
            Metadata<'a>, // (
            Option<PortExpression<'a>>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PortExpression<'a> {
    SinglePortReference(Box<PortReference<'a>>),
    MultiPortReference(
        Box<(
            Metadata<'a>, // {
            PortReference<'a>,
            Vec<(Metadata<'a>, PortReference<'a>)>,
            Metadata<'a>, // }
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PortReference<'a>(pub PortIdentifier<'a>, pub ConstantSelect<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum PortDirection<'a> {
    Input(Metadata<'a>),
    Output(Metadata<'a>),
    Inout(Metadata<'a>),
    Ref(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetPortHeader<'a>(
    pub Option<PortDirection<'a>>,
    pub NetPortType<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct VariablePortHeader<'a>(
    pub Option<PortDirection<'a>>,
    pub VariablePortType<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum InterfacePortHeader<'a> {
    InterfaceIdentifier(
        (
            InterfaceIdentifier<'a>,
            Option<(Metadata<'a>, ModportIdentifier<'a>)>,
        ),
    ),
    Interface((Metadata<'a>, Option<(Metadata<'a>, ModportIdentifier<'a>)>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnsiPortDeclaration<'a> {
    NetPort(Box<AnsiNetPortDeclaration<'a>>),
    VariablePort(Box<AnsiVariablePortDeclaration<'a>>),
    ConstantPort(Box<AnsiConstantPortDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NetOrInterfacePortHeader<'a> {
    NetPortHeader(Box<NetPortHeader<'a>>),
    InterfacePortHeader(Box<InterfacePortHeader<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnsiNetPortDeclaration<'a>(
    pub Option<NetOrInterfacePortHeader<'a>>,
    pub PortIdentifier<'a>,
    pub Vec<UnpackedDimension<'a>>,
    pub Option<(Metadata<'a>, ConstantExpression<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct AnsiVariablePortDeclaration<'a>(
    pub Option<VariablePortHeader<'a>>,
    pub PortIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub Option<(Metadata<'a>, ConstantExpression<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct AnsiConstantPortDeclaration<'a>(
    pub Option<PortDirection<'a>>,
    pub Metadata<'a>, // .
    pub PortIdentifier<'a>,
    pub Metadata<'a>, // (
    pub Option<Expression<'a>>,
    pub Metadata<'a>, // )
);
