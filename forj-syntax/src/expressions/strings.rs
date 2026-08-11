// =======================================================================
// strings.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.8.8
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum StringLiteral<'a> {
    QuotedString(Box<QuotedString<'a>>),
    TripleQuotedString(Box<TripleQuotedString<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuotedString<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TripleQuotedString<'a>(pub &'a str, pub Metadata<'a>);
