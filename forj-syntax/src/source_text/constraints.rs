// =======================================================================
// constraints.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.10
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintDeclaration<'a>(
    pub Option<Metadata<'a>>, // static
    pub Metadata<'a>,         // constraint
    pub Option<DynamicOverrideSpecifiers<'a>>,
    pub ConstraintIdentifier<'a>,
    pub ConstraintBlock<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintBlock<'a>(
    pub Metadata<'a>, // {
    pub Vec<ConstraintBlockItem<'a>>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintBlockItem<'a> {
    Ordering(
        Box<(
            Metadata<'a>, // solve
            SolveBeforeList<'a>,
            Metadata<'a>, // before
            SolveBeforeList<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Expression(Box<ConstraintExpression<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SolveBeforeList<'a>(
    pub ConstraintPrimary<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        ConstraintPrimary<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintPrimary<'a>(
    pub ImplicitClassHandleOrClassScope<'a>,
    pub HierarchicalIdentifier<'a>,
    pub Select<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintExpression<'a> {
    Expression(
        Box<(
            Option<Metadata<'a>>, // soft
            ExpressionOrDist<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Uniqueness(
        Box<(
            UniquenessConstraint<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Implication(
        Box<(
            Expression<'a>,
            Metadata<'a>, // ->
            ConstraintSet<'a>,
        )>,
    ),
    Conditional(
        Box<(
            Metadata<'a>, // if
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
            ConstraintSet<'a>,
            Option<(
                Metadata<'a>, // else
                ConstraintSet<'a>,
            )>,
        )>,
    ),
    Foreach(
        Box<(
            Metadata<'a>, // foreach
            Metadata<'a>, // (
            PsOrHierarchicalArrayIdentifier<'a>,
            Metadata<'a>, // [
            LoopVariables<'a>,
            Metadata<'a>, // ]
            Metadata<'a>, // )
            ConstraintSet<'a>,
        )>,
    ),
    Disable(
        Box<(
            Metadata<'a>, // disable
            Metadata<'a>, // soft
            ConstraintPrimary<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UniquenessConstraint<'a>(
    pub Metadata<'a>, // unique
    pub Metadata<'a>, // {
    pub RangeList<'a>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintSet<'a> {
    Single(Box<ConstraintExpression<'a>>),
    Multi(
        Box<(
            Metadata<'a>, // {
            Vec<ConstraintExpression<'a>>,
            Metadata<'a>, // }
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionOrDist<'a>(
    pub Expression<'a>,
    pub  Option<(
        Metadata<'a>, // dist
        Metadata<'a>, // {
        DistList<'a>,
        Metadata<'a>, // }
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct DistList<'a>(
    pub DistItem<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        DistItem<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum DistItem<'a> {
    Value(Box<(ValueRange<'a>, Option<DistWeight<'a>>)>),
    Default(
        Box<(
            Metadata<'a>, // default
            Metadata<'a>, // :/
            Expression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DistWeight<'a> {
    IndvWeight(
        Metadata<'a>, // :=
        Expression<'a>,
    ),
    RangeWeight(
        Metadata<'a>, // :/
        Expression<'a>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintPrototype<'a>(
    pub Option<ConstraintPrototypeQualifier<'a>>,
    pub Option<Metadata<'a>>, // static
    pub Metadata<'a>,         // constraint
    pub Option<DynamicOverrideSpecifiers<'a>>,
    pub ConstraintIdentifier<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintPrototypeQualifier<'a> {
    Extern(Metadata<'a>),
    Pure(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExternConstraintDeclaration<'a>(
    pub Option<Metadata<'a>>, // static
    pub Metadata<'a>,         // constraint
    pub Option<DynamicOverrideSpecifiers<'a>>,
    pub ClassScope<'a>,
    pub ConstraintIdentifier<'a>,
    pub ConstraintBlock<'a>,
);
