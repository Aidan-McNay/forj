// =======================================================================
// udp_declaration.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.5.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UdpNonansiDeclaration<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // primitive
    pub UdpIdentifier<'a>,
    pub Metadata<'a>, // (
    pub UdpPortList<'a>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct UdpAnsiDeclaration<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // primitive
    pub UdpIdentifier<'a>,
    pub Metadata<'a>, // (
    pub UdpDeclarationPortList<'a>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum UdpDeclaration<'a> {
    Nonansi(
        Box<(
            UdpNonansiDeclaration<'a>,
            UdpPortDeclaration<'a>,
            Vec<UdpPortDeclaration<'a>>,
            UdpBody<'a>,
            Metadata<'a>, // endprimitive
            Option<(
                Metadata<'a>, // :
                UdpIdentifier<'a>,
            )>,
        )>,
    ),
    Ansi(
        Box<(
            UdpAnsiDeclaration<'a>,
            UdpBody<'a>,
            Metadata<'a>, // endprimitive
            Option<(
                Metadata<'a>, // :
                UdpIdentifier<'a>,
            )>,
        )>,
    ),
    ExternNonansi(
        Box<(
            Metadata<'a>, // extern
            UdpNonansiDeclaration<'a>,
        )>,
    ),
    ExternAnsi(
        Box<(
            Metadata<'a>, // extern
            UdpAnsiDeclaration<'a>,
        )>,
    ),
    Wildcard(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // primitive
            UdpIdentifier<'a>,
            Metadata<'a>, // (
            Metadata<'a>, // .
            Metadata<'a>, // *
            Metadata<'a>, // )
            Metadata<'a>, // ;
            Vec<UdpPortDeclaration<'a>>,
            UdpBody<'a>,
            Metadata<'a>, // endprimitive
            Option<(
                Metadata<'a>, // :
                UdpIdentifier<'a>,
            )>,
        )>,
    ),
}
