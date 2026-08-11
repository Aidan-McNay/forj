// =======================================================================
// patterns.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.6.7.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern<'a> {
    Parentheses(Box<(Metadata<'a>, Pattern<'a>, Metadata<'a>)>),
    VariableIdentifier(Box<(Metadata<'a>, VariableIdentifier<'a>)>),
    Wildcard(Box<(Metadata<'a>, Metadata<'a>)>),
    ConstantExpression(Box<ConstantExpression<'a>>),
    TaggedMember(
        Box<(Metadata<'a>, MemberIdentifier<'a>, Option<Pattern<'a>>)>,
    ),
    MultiPattern(
        Box<(
            Metadata<'a>, // '
            Metadata<'a>, // {
            Pattern<'a>,
            Vec<(
                Metadata<'a>, // ,
                Pattern<'a>,
            )>,
            Metadata<'a>, // }
        )>,
    ),
    MultiIdentifierPattern(
        Box<(
            Metadata<'a>, // '
            Metadata<'a>, // {
            MemberIdentifier<'a>,
            Metadata<'a>, // :
            Pattern<'a>,
            Vec<(
                Metadata<'a>, // ,
                MemberIdentifier<'a>,
                Metadata<'a>, // :
                Pattern<'a>,
            )>,
            Metadata<'a>, // }
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentPattern<'a> {
    Expression(
        Box<(
            Metadata<'a>, // '
            Metadata<'a>, // {
            Expression<'a>,
            Vec<(
                Metadata<'a>, // ,
                Expression<'a>,
            )>,
            Metadata<'a>, // }
        )>,
    ),
    Structure(
        Box<(
            Metadata<'a>, // '
            Metadata<'a>, // {
            StructurePatternKey<'a>,
            Metadata<'a>, // :
            Expression<'a>,
            Vec<(
                Metadata<'a>, // ,
                StructurePatternKey<'a>,
                Metadata<'a>, // :
                Expression<'a>,
            )>,
            Metadata<'a>, // }
        )>,
    ),
    Array(
        Box<(
            Metadata<'a>, // '
            Metadata<'a>, // {
            ArrayPatternKey<'a>,
            Metadata<'a>, // :
            Expression<'a>,
            Vec<(
                Metadata<'a>, // ,
                ArrayPatternKey<'a>,
                Metadata<'a>, // :
                Expression<'a>,
            )>,
            Metadata<'a>, // }
        )>,
    ),
    Constant(
        Box<(
            Metadata<'a>, // '
            Metadata<'a>, // {
            ConstantExpression<'a>,
            Metadata<'a>, // {
            Expression<'a>,
            Vec<(
                Metadata<'a>, // ,
                Expression<'a>,
            )>,
            Metadata<'a>, // }
            Metadata<'a>, // }
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructurePatternKey<'a> {
    MemberIdentifier(Box<MemberIdentifier<'a>>),
    AssignmentPatternKey(Box<AssignmentPatternKey<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArrayPatternKey<'a> {
    ConstantExpression(Box<ConstantExpression<'a>>),
    AssignmentPatternKey(Box<AssignmentPatternKey<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentPatternKey<'a> {
    SimpleType(Box<SimpleType<'a>>),
    Default(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentPatternExpression<'a>(
    pub Option<AssignmentPatternExpressionType<'a>>,
    pub AssignmentPattern<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentPatternExpressionType<'a> {
    PsType(Box<PsTypeIdentifier<'a>>),
    PsParameter(Box<PsParameterIdentifier<'a>>),
    Integer(Box<IntegerAtomType<'a>>),
    Type(Box<TypeReference<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantAssignmentPatternExpression<'a>(
    pub AssignmentPatternExpression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentPatternNetLvalue<'a>(
    pub Metadata<'a>, // '
    pub Metadata<'a>, // {
    pub NetLvalue<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        NetLvalue<'a>,
    )>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentPatternVariableLvalue<'a>(
    pub Metadata<'a>, // '
    pub Metadata<'a>, // {
    pub VariableLvalue<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        VariableLvalue<'a>,
    )>,
    pub Metadata<'a>, // }
);
