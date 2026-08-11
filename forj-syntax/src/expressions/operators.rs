// =======================================================================
// operators.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.8.6
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator<'a> {
    Plus(Metadata<'a>),
    Minus(Metadata<'a>),
    Exclamation(Metadata<'a>),
    Tilde(Metadata<'a>),
    Amp(Metadata<'a>),
    TildeAmp(Metadata<'a>),
    Pipe(Metadata<'a>),
    TildePipe(Metadata<'a>),
    Caret(Metadata<'a>),
    TildeCaret(Metadata<'a>),
    CaretTilde(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator<'a> {
    Plus(Metadata<'a>),
    Minus(Metadata<'a>),
    Star(Metadata<'a>),
    Slash(Metadata<'a>),
    Percent(Metadata<'a>),
    EqEq(Metadata<'a>),
    ExclEq(Metadata<'a>),
    EqEqEq(Metadata<'a>),
    ExclEqEq(Metadata<'a>),
    EqEqQuest(Metadata<'a>),
    ExclEqQuest(Metadata<'a>),
    AmpAmp(Metadata<'a>),
    PipePipe(Metadata<'a>),
    StarStar(Metadata<'a>),
    Lt(Metadata<'a>),
    LtEq(Metadata<'a>),
    Gt(Metadata<'a>),
    GtEq(Metadata<'a>),
    Amp(Metadata<'a>),
    Pipe(Metadata<'a>),
    Caret(Metadata<'a>),
    CaretTilde(Metadata<'a>),
    TildeCaret(Metadata<'a>),
    GtGt(Metadata<'a>),
    LtLt(Metadata<'a>),
    GtGtGt(Metadata<'a>),
    LtLtLt(Metadata<'a>),
    MinusGt(Metadata<'a>),
    LtMinusGt(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IncOrDecOperator<'a> {
    PlusPlus(Metadata<'a>),
    MinusMinus(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryModulePathOperator<'a> {
    Exclamation(Metadata<'a>),
    Tilde(Metadata<'a>),
    Amp(Metadata<'a>),
    TildeAmp(Metadata<'a>),
    Pipe(Metadata<'a>),
    TildePipe(Metadata<'a>),
    Caret(Metadata<'a>),
    TildeCaret(Metadata<'a>),
    CaretTilde(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryModulePathOperator<'a> {
    EqEq(Metadata<'a>),
    ExclEq(Metadata<'a>),
    AmpAmp(Metadata<'a>),
    PipePipe(Metadata<'a>),
    Amp(Metadata<'a>),
    Pipe(Metadata<'a>),
    Caret(Metadata<'a>),
    CaretTilde(Metadata<'a>),
    TildeCaret(Metadata<'a>),
}
