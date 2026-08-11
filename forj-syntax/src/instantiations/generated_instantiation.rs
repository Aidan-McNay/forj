// =======================================================================
// generated_instantiation.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.4.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct GenerateRegion<'a>(
    pub Metadata<'a>, // generate
    pub Vec<GenerateItem<'a>>,
    pub Metadata<'a>, // endgenerate
);

#[derive(Clone, Debug, PartialEq)]
pub struct LoopGenerateConstruct<'a>(
    pub Metadata<'a>, // for
    pub Metadata<'a>, // (
    pub GenvarInitialization<'a>,
    pub Metadata<'a>, // ;
    pub GenvarExpression<'a>,
    pub Metadata<'a>, // ;
    pub GenvarIteration<'a>,
    pub Metadata<'a>, // )
    pub GenerateBlock<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct GenvarInitialization<'a>(
    pub Option<Metadata<'a>>, // genvar
    pub GenvarIdentifier<'a>,
    pub Metadata<'a>, // =
    pub ConstantExpression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum GenvarIteration<'a> {
    Assignment(
        Box<(
            GenvarIdentifier<'a>,
            AssignmentOperator<'a>,
            GenvarExpression<'a>,
        )>,
    ),
    Prefix(Box<(IncOrDecOperator<'a>, GenvarIdentifier<'a>)>),
    Postfix(Box<(GenvarIdentifier<'a>, IncOrDecOperator<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConditionalGenerateConstruct<'a> {
    If(Box<IfGenerateConstruct<'a>>),
    Case(Box<CaseGenerateConstruct<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfGenerateConstruct<'a>(
    pub Metadata<'a>, // if
    pub Metadata<'a>, // (
    pub ConstantExpression<'a>,
    pub Metadata<'a>, // )
    pub GenerateBlock<'a>,
    pub  Option<(
        Metadata<'a>, // else
        GenerateBlock<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CaseGenerateConstruct<'a>(
    pub Metadata<'a>, // case
    pub Metadata<'a>, // (
    pub ConstantExpression<'a>,
    pub Metadata<'a>, // )
    pub CaseGenerateItem<'a>,
    pub Vec<CaseGenerateItem<'a>>,
    pub Metadata<'a>, // endcase
);

#[derive(Clone, Debug, PartialEq)]
pub enum CaseGenerateItem<'a> {
    Expression(
        Box<(
            ConstantExpression<'a>,
            Vec<(
                Metadata<'a>, // ,
                ConstantExpression<'a>,
            )>,
            Metadata<'a>, // :
            GenerateBlock<'a>,
        )>,
    ),
    Default(
        Box<(
            Metadata<'a>,         // default
            Option<Metadata<'a>>, // :
            GenerateBlock<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenerateBlock<'a> {
    Item(Box<GenerateItem<'a>>),
    Block(
        Box<(
            Option<(
                GenerateBlockIdentifier<'a>,
                Metadata<'a>, // :
            )>,
            Metadata<'a>, // begin
            Option<(
                Metadata<'a>, // :
                GenerateBlockIdentifier<'a>,
            )>,
            Vec<GenerateItem<'a>>,
            Metadata<'a>, // end
            Option<(
                Metadata<'a>, // :
                GenerateBlockIdentifier<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenerateItem<'a> {
    Module(Box<ModuleOrGenerateItem<'a>>),
    Interface(Box<InterfaceOrGenerateItem<'a>>),
    Checker(Box<CheckerOrGenerateItem<'a>>),
}
