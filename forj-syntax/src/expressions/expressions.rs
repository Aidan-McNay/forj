// =======================================================================
// expressions.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.8.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum IncOrDecExpression<'a> {
    Preop(
        Box<(
            IncOrDecOperator<'a>,
            Vec<AttributeInstance<'a>>,
            VariableLvalue<'a>,
        )>,
    ),
    Postop(
        Box<(
            VariableLvalue<'a>,
            Vec<AttributeInstance<'a>>,
            IncOrDecOperator<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConditionalExpression<'a>(
    pub CondPredicate<'a>,
    pub Metadata<'a>, // ?
    pub Vec<AttributeInstance<'a>>,
    pub Expression<'a>,
    pub Metadata<'a>, // :
    pub Expression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantExpression<'a> {
    Primary(Box<ConstantPrimary<'a>>),
    Unary(
        Box<(
            UnaryOperator<'a>,
            Vec<AttributeInstance<'a>>,
            ConstantPrimary<'a>,
        )>,
    ),
    Binary(
        Box<(
            ConstantExpression<'a>,
            BinaryOperator<'a>,
            Vec<AttributeInstance<'a>>,
            ConstantExpression<'a>,
        )>,
    ),
    Ternary(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>, // ?
            Vec<AttributeInstance<'a>>,
            ConstantExpression<'a>,
            Metadata<'a>, // :
            ConstantExpression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantMintypmaxExpression<'a> {
    Single(Box<ConstantExpression<'a>>),
    Mintypmax(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>,
            ConstantExpression<'a>,
            Metadata<'a>,
            ConstantExpression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantParamExpression<'a> {
    Mintypmax(Box<ConstantMintypmaxExpression<'a>>),
    Data(Box<DataType<'a>>),
    Dollar(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParamExpression<'a> {
    Mintypmax(Box<MintypmaxExpression<'a>>),
    Data(Box<DataType<'a>>),
    Dollar(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantRangeExpression<'a> {
    Expression(Box<ConstantExpression<'a>>),
    PartSelectRange(Box<ConstantPartSelectRange<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantPartSelectRange<'a> {
    Range(Box<ConstantRange<'a>>),
    IndexedRange(Box<ConstantIndexedRange<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantRange<'a>(
    pub ConstantExpression<'a>,
    pub Metadata<'a>, // :
    pub ConstantExpression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantIndexedRange<'a> {
    Plus(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>, // +:
            ConstantExpression<'a>,
        )>,
    ),
    Minus(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>, // -:
            ConstantExpression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'a> {
    Primary(Box<Primary<'a>>),
    Unary(Box<(UnaryOperator<'a>, Vec<AttributeInstance<'a>>, Primary<'a>)>),
    IncOrDecExpression(Box<IncOrDecExpression<'a>>),
    OperatorAssignment(
        Box<(Metadata<'a>, OperatorAssignment<'a>, Metadata<'a>)>,
    ),
    Binary(
        Box<(
            Expression<'a>,
            BinaryOperator<'a>,
            Vec<AttributeInstance<'a>>,
            Expression<'a>,
        )>,
    ),
    ConditionalExpression(Box<ConditionalExpression<'a>>),
    InsideExpression(Box<InsideExpression<'a>>),
    TaggedUnionExpression(Box<TaggedUnionExpression<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaggedUnionExpression<'a>(
    pub Metadata<'a>, // tagged
    pub MemberIdentifier<'a>,
    pub Option<Primary<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InsideExpression<'a>(
    pub Expression<'a>,
    pub Metadata<'a>, // inside
    pub Metadata<'a>, // {
    pub RangeList<'a>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub enum MintypmaxExpression<'a> {
    Single(Box<Expression<'a>>),
    Mintypmax(
        Box<(
            Expression<'a>,
            Metadata<'a>,
            Expression<'a>,
            Metadata<'a>,
            Expression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModulePathConditionalExpression<'a>(
    pub ModulePathExpression<'a>,
    pub Metadata<'a>, // ?
    pub Vec<AttributeInstance<'a>>,
    pub ModulePathExpression<'a>,
    pub Metadata<'a>, // :
    pub ModulePathExpression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ModulePathExpression<'a> {
    Primary(Box<ModulePathPrimary<'a>>),
    Unary(
        Box<(
            UnaryModulePathOperator<'a>,
            Vec<AttributeInstance<'a>>,
            ModulePathPrimary<'a>,
        )>,
    ),
    Binary(
        Box<(
            ModulePathExpression<'a>,
            BinaryModulePathOperator<'a>,
            Vec<AttributeInstance<'a>>,
            ModulePathExpression<'a>,
        )>,
    ),
    Conditional(Box<ModulePathConditionalExpression<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModulePathMintypmaxExpression<'a> {
    Single(Box<ModulePathExpression<'a>>),
    Mintypmax(
        Box<(
            ModulePathExpression<'a>,
            Metadata<'a>,
            ModulePathExpression<'a>,
            Metadata<'a>,
            ModulePathExpression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PartSelectRange<'a> {
    ConstantRange(Box<ConstantRange<'a>>),
    IndexedRange(Box<IndexedRange<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IndexedRange<'a> {
    Plus(
        Box<(
            Expression<'a>,
            Metadata<'a>, // +:
            ConstantExpression<'a>,
        )>,
    ),
    Minus(
        Box<(
            Expression<'a>,
            Metadata<'a>, // -:
            ConstantExpression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenvarExpression<'a>(pub ConstantExpression<'a>);
