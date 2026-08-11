// =======================================================================
// randsequence.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.12
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RandsequenceStatement<'a>(
    pub Metadata<'a>, // randsequence
    pub Metadata<'a>, // (
    pub Option<RsProductionIdentifier<'a>>,
    pub Metadata<'a>, // )
    pub RsProduction<'a>,
    pub Vec<RsProduction<'a>>,
    pub Metadata<'a>, // endsequence
);

#[derive(Clone, Debug, PartialEq)]
pub struct RsProduction<'a>(
    pub Option<DataTypeOrVoid<'a>>,
    pub RsProductionIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        TfPortList<'a>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // :
    pub RsRule<'a>,
    pub  Vec<(
        Metadata<'a>, // |
        RsRule<'a>,
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct RsRule<'a>(
    pub RsProductionList<'a>,
    pub  Option<(
        Metadata<'a>, // :=
        RsWeightSpecification<'a>,
        Option<RsCodeBlock<'a>>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum RsProductionList<'a> {
    Producers(Box<(RsProd<'a>, Vec<RsProd<'a>>)>),
    Join(
        Box<(
            Metadata<'a>, // rand
            Metadata<'a>, // join
            Option<(
                Metadata<'a>, // (
                Expression<'a>,
                Metadata<'a>, // )
            )>,
            RsProductionItem<'a>,
            RsProductionItem<'a>,
            Vec<RsProductionItem<'a>>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RsWeightSpecification<'a> {
    Integral(Box<IntegralNumber<'a>>),
    Ps(Box<PsIdentifier<'a>>),
    Expression(
        Box<(
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RsCodeBlock<'a>(
    pub Metadata<'a>, // {
    pub Vec<DataDeclaration<'a>>,
    pub Vec<StatementOrNull<'a>>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub enum RsProd<'a> {
    Item(Box<RsProductionItem<'a>>),
    CodeBlock(Box<RsCodeBlock<'a>>),
    IfElse(Box<RsIfElse<'a>>),
    Repeat(Box<RsRepeat<'a>>),
    Case(Box<RsCase<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RsProductionItem<'a>(
    pub RsProductionIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        ListOfArguments<'a>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct RsIfElse<'a>(
    pub Metadata<'a>, // if
    pub Metadata<'a>, // (
    pub Expression<'a>,
    pub Metadata<'a>, // )
    pub RsProductionItem<'a>,
    pub  Option<(
        Metadata<'a>, // else
        RsProductionItem<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct RsRepeat<'a>(
    pub Metadata<'a>, // repeat
    pub Metadata<'a>, // (
    pub Expression<'a>,
    pub Metadata<'a>, // )
    pub RsProductionItem<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct RsCase<'a>(
    pub Metadata<'a>, // case
    pub Metadata<'a>, // (
    pub CaseExpression<'a>,
    pub Metadata<'a>, // )
    pub RsCaseItem<'a>,
    pub Vec<RsCaseItem<'a>>,
    pub Metadata<'a>, // endcase
);

#[derive(Clone, Debug, PartialEq)]
pub enum RsCaseItem<'a> {
    Expression(
        Box<(
            CaseItemExpression<'a>,
            Vec<(
                Metadata<'a>, // ,
                CaseItemExpression<'a>,
            )>,
            Metadata<'a>, // :
            RsProductionItem<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Default(
        Box<(
            Metadata<'a>,         // default
            Option<Metadata<'a>>, // :
            RsProductionItem<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}
