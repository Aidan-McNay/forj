// =======================================================================
// primaries.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.8.4
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantPrimary<'a> {
    PrimaryLiteral(Box<PrimaryLiteral<'a>>),
    PsParameter(Box<(PsParameterIdentifier<'a>, ConstantSelect<'a>)>),
    Specparam(
        Box<(
            SpecparamIdentifier<'a>,
            Option<(Metadata<'a>, ConstantRangeExpression<'a>, Metadata<'a>)>,
        )>,
    ),
    Genvar(Box<GenvarIdentifier<'a>>),
    FormalPort(Box<(FormalPortIdentifier<'a>, ConstantSelect<'a>)>),
    Enum(Box<(Option<PackageOrClassScope<'a>>, EnumIdentifier<'a>)>),
    EmptyUnpackedArrayConcatenation(Box<EmptyUnpackedArrayConcatenation<'a>>),
    Concatenation(
        Box<(
            ConstantConcatenation<'a>,
            Option<(Metadata<'a>, ConstantRangeExpression<'a>, Metadata<'a>)>,
        )>,
    ),
    MultipleConcatenation(
        Box<(
            ConstantMultipleConcatenation<'a>,
            Option<(Metadata<'a>, ConstantRangeExpression<'a>, Metadata<'a>)>,
        )>,
    ),
    FunctionCall(
        Box<(
            ConstantFunctionCall<'a>,
            Option<(Metadata<'a>, ConstantRangeExpression<'a>, Metadata<'a>)>,
        )>,
    ),
    LetExpression(Box<ConstantLetExpression<'a>>),
    MintypmaxExpression(
        Box<(Metadata<'a>, ConstantMintypmaxExpression<'a>, Metadata<'a>)>,
    ),
    Cast(Box<ConstantCast<'a>>),
    AssignmentPatternExpression(Box<ConstantAssignmentPatternExpression<'a>>),
    TypeReference(Box<TypeReference<'a>>),
    Null(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModulePathPrimary<'a> {
    Number(Box<Number<'a>>),
    Identifier(Box<Identifier<'a>>),
    Concatenation(Box<ModulePathConcatenation<'a>>),
    MultipleConcatenation(Box<ModulePathMultipleConcatenation<'a>>),
    FunctionSubroutineCall(Box<FunctionSubroutineCall<'a>>),
    MintypmaxExpression(
        Box<(
            Metadata<'a>,
            ModulePathMintypmaxExpression<'a>,
            Metadata<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassQualifierOrPackageScope<'a> {
    ClassQualifier(Box<ClassQualifier<'a>>),
    PackageScope(Box<PackageScope<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Primary<'a> {
    PrimaryLiteral(Box<PrimaryLiteral<'a>>),
    HierarchicalIdentifier(
        Box<(
            Option<ClassQualifierOrPackageScope<'a>>,
            HierarchicalIdentifier<'a>,
            Select<'a>,
        )>,
    ),
    EmptyUnpackedArrayConcatenation(Box<EmptyUnpackedArrayConcatenation<'a>>),
    Concatenation(
        Box<(
            Concatenation<'a>,
            Option<(Metadata<'a>, RangeExpression<'a>, Metadata<'a>)>,
        )>,
    ),
    MultipleConcatenation(
        Box<(
            MultipleConcatenation<'a>,
            Option<(Metadata<'a>, RangeExpression<'a>, Metadata<'a>)>,
        )>,
    ),
    FunctionSubroutineCall(
        Box<(
            FunctionSubroutineCall<'a>,
            Option<(Metadata<'a>, RangeExpression<'a>, Metadata<'a>)>,
        )>,
    ),
    LetExpression(Box<LetExpression<'a>>),
    MintypmaxExpression(
        Box<(Metadata<'a>, MintypmaxExpression<'a>, Metadata<'a>)>,
    ),
    Cast(Box<Cast<'a>>),
    AssignmentPatternExpression(Box<AssignmentPatternExpression<'a>>),
    StreamingConcatenation(Box<StreamingConcatenation<'a>>),
    SequenceMethodCall(Box<SequenceMethodCall<'a>>),
    This(Box<Metadata<'a>>),
    Dollar(Box<Metadata<'a>>),
    Null(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImplicitClassHandleOrClassScope<'a> {
    ImplicitClassHandle(Box<(ImplicitClassHandle<'a>, Metadata<'a>)>),
    ClassScope(Box<ClassScope<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassQualifier<'a>(
    pub  Option<(
        Metadata<'a>, // local
        Metadata<'a>, // ::
    )>,
    pub Option<ImplicitClassHandleOrClassScope<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum RangeExpression<'a> {
    Expression(Box<Expression<'a>>),
    PartSelectRange(Box<PartSelectRange<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimaryLiteral<'a> {
    Number(Box<Number<'a>>),
    TimeLiteral(Box<TimeLiteral<'a>>),
    UnbasedUnsizedLiteral(Box<UnbasedUnsizedLiteral<'a>>),
    StringLiteral(Box<StringLiteral<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeLiteral<'a> {
    TimeLiteralUnsigned(Box<(UnsignedNumber<'a>, TimeUnit<'a>)>),
    TimeLiteralFixedPoint(Box<(FixedPointNumber<'a>, TimeUnit<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeUnit<'a> {
    S(Metadata<'a>),
    MS(Metadata<'a>),
    US(Metadata<'a>),
    NS(Metadata<'a>),
    PS(Metadata<'a>),
    FS(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImplicitClassHandle<'a> {
    This(Metadata<'a>),
    Super(Metadata<'a>),
    ThisSuper(Metadata<'a>, Metadata<'a>, Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BitSelect<'a>(
    pub  Vec<(
        Metadata<'a>, // [
        Expression<'a>,
        Metadata<'a>, // ]
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct Select<'a>(
    pub  Option<(
        Vec<(
            Metadata<'a>, // .
            MemberIdentifier<'a>,
            BitSelect<'a>,
        )>,
        Metadata<'a>, // .
        MemberIdentifier<'a>,
    )>,
    pub BitSelect<'a>,
    pub  Option<(
        Metadata<'a>, // [
        PartSelectRange<'a>,
        Metadata<'a>, // ]
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct NonrangeSelect<'a>(
    pub  Option<(
        Vec<(
            Metadata<'a>, // .
            MemberIdentifier<'a>,
            BitSelect<'a>,
        )>,
        Metadata<'a>, // .
        MemberIdentifier<'a>,
    )>,
    pub BitSelect<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantBitSelect<'a>(
    pub  Vec<(
        Metadata<'a>, // [
        ConstantExpression<'a>,
        Metadata<'a>, // ]
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantSelect<'a>(
    pub  Option<(
        Vec<(
            Metadata<'a>, // .
            MemberIdentifier<'a>,
            ConstantBitSelect<'a>,
        )>,
        Metadata<'a>, // .
        MemberIdentifier<'a>,
    )>,
    pub ConstantBitSelect<'a>,
    pub  Option<(
        Metadata<'a>, // [
        ConstantPartSelectRange<'a>,
        Metadata<'a>, // ]
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct Cast<'a>(
    pub CastingType<'a>,
    pub Metadata<'a>, // '
    pub Metadata<'a>, // (
    pub Expression<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantCast<'a>(
    pub CastingType<'a>,
    pub Metadata<'a>, // '
    pub Metadata<'a>, // (
    pub ConstantExpression<'a>,
    pub Metadata<'a>, // )
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantLetExpression<'a>(pub LetExpression<'a>);
