// =======================================================================
// numbers.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.8.7
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Number<'a> {
    Integral(Box<IntegralNumber<'a>>),
    Real(Box<RealNumber<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IntegralNumber<'a> {
    Decimal(Box<DecimalNumber<'a>>),
    Octal(Box<OctalNumber<'a>>),
    Binary(Box<BinaryNumber<'a>>),
    Hex(Box<HexNumber<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DecimalNumber<'a> {
    Sized(Box<(&'a str, Metadata<'a>)>),
    Unsized(Box<UnsignedNumber<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryNumber<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct OctalNumber<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HexNumber<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum RealNumber<'a> {
    FixedPoint(Box<FixedPointNumber<'a>>),
    Scientific(Box<ScientificNumber<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FixedPointNumber<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ScientificNumber<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct UnsignedNumber<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct UnbasedUnsizedLiteral<'a>(pub &'a str, pub Metadata<'a>);
