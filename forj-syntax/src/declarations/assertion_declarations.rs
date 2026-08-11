// =======================================================================
// assertion_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.10
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ConcurrentAssertionItem<'a> {
    Assertion(
        Box<(
            Option<(
                BlockIdentifier<'a>,
                Metadata<'a>, // :
            )>,
            ConcurrentAssertionStatement<'a>,
        )>,
    ),
    Checker(Box<CheckerInstantiation<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConcurrentAssertionStatement<'a> {
    AssertProp(Box<AssertPropertyStatement<'a>>),
    AssumeProp(Box<AssumePropertyStatement<'a>>),
    CoverProp(Box<CoverPropertyStatement<'a>>),
    CoverSeq(Box<CoverSequenceStatement<'a>>),
    RestrictProp(Box<RestrictPropertyStatement<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssertPropertyStatement<'a>(
    pub Metadata<'a>, // assert
    pub Metadata<'a>, // property
    pub Metadata<'a>, // (
    pub PropertySpec<'a>,
    pub Metadata<'a>, // )
    pub ActionBlock<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct AssumePropertyStatement<'a>(
    pub Metadata<'a>, // assume
    pub Metadata<'a>, // property
    pub Metadata<'a>, // (
    pub PropertySpec<'a>,
    pub Metadata<'a>, // )
    pub ActionBlock<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CoverPropertyStatement<'a>(
    pub Metadata<'a>, // cover
    pub Metadata<'a>, // property
    pub Metadata<'a>, // (
    pub PropertySpec<'a>,
    pub Metadata<'a>, // )
    pub StatementOrNull<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ExpectPropertyStatement<'a>(
    pub Metadata<'a>, // expect
    pub Metadata<'a>, // (
    pub PropertySpec<'a>,
    pub Metadata<'a>, // )
    pub ActionBlock<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CoverSequenceStatement<'a>(
    pub Metadata<'a>, // cover
    pub Metadata<'a>, // sequence
    pub Metadata<'a>, // (
    pub Option<ClockingEvent<'a>>,
    pub  Option<(
        Metadata<'a>, // disable
        Metadata<'a>, // iff
        Metadata<'a>, // (
        ExpressionOrDist<'a>,
        Metadata<'a>, // )
    )>,
    pub SequenceExpr<'a>,
    pub Metadata<'a>, // )
    pub StatementOrNull<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct RestrictPropertyStatement<'a>(
    pub Metadata<'a>, // restrict
    pub Metadata<'a>, // property
    pub Metadata<'a>, // (
    pub PropertySpec<'a>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyInstance<'a>(
    pub PsOrHierarchicalPropertyIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<PropertyListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyListOfArguments<'a> {
    PartialIdentifier(
        Box<(
            Option<PropertyActualArg<'a>>,
            Vec<(
                Metadata<'a>, // ,
                Option<PropertyActualArg<'a>>,
            )>,
            Vec<(
                Metadata<'a>, // ,
                Metadata<'a>, // .
                Identifier<'a>,
                Metadata<'a>, // (
                Option<PropertyActualArg<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Identifier(
        Box<(
            Metadata<'a>, // .
            Identifier<'a>,
            Metadata<'a>, // (
            Option<PropertyActualArg<'a>>,
            Metadata<'a>, // )
            Vec<(
                Metadata<'a>, // ,
                Metadata<'a>, // .
                Identifier<'a>,
                Metadata<'a>, // (
                Option<PropertyActualArg<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyActualArg<'a> {
    Property(Box<PropertyExpr<'a>>),
    Sequence(Box<SequenceActualArg<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssertionItemDeclaration<'a> {
    Property(Box<PropertyDeclaration<'a>>),
    Sequence(Box<SequenceDeclaration<'a>>),
    Let(Box<LetDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyDeclaration<'a>(
    pub Metadata<'a>, // property
    pub PropertyIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<PropertyPortList<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
    pub Vec<AssertionVariableDeclaration<'a>>,
    pub PropertySpec<'a>,
    pub Option<Metadata<'a>>, // ;
    pub Metadata<'a>,         // endproperty
    pub  Option<(
        Metadata<'a>, // :
        PropertyIdentifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyPortList<'a>(
    pub PropertyPortItem<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        PropertyPortItem<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyPortItem<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub  Option<(
        Metadata<'a>, // local
        Option<PropertyLvarPortDirection<'a>>,
    )>,
    pub PropertyFormalType<'a>,
    pub FormalPortIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub  Option<(
        Metadata<'a>, // =
        PropertyActualArg<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyLvarPortDirection<'a>(
    pub Metadata<'a>, // input
);

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyFormalType<'a> {
    Sequence(Box<SequenceFormalType<'a>>),
    Property(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PropertySpec<'a>(
    pub Option<ClockingEvent<'a>>,
    pub  Option<(
        Metadata<'a>, // disable
        Metadata<'a>, // iff
        Metadata<'a>, // (
        ExpressionOrDist<'a>,
        Metadata<'a>, // )
    )>,
    pub PropertyExpr<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyExpr<'a> {
    SeqExpr(Box<SequenceExpr<'a>>),
    Strong(
        Box<(
            Metadata<'a>, // strong
            Metadata<'a>, // (
            SequenceExpr<'a>,
            Metadata<'a>, // )
        )>,
    ),
    Weak(
        Box<(
            Metadata<'a>, // weak
            Metadata<'a>, // (
            SequenceExpr<'a>,
            Metadata<'a>, // )
        )>,
    ),
    Paren(
        Box<(
            Metadata<'a>, // (
            PropertyExpr<'a>,
            Metadata<'a>, // )
        )>,
    ),
    Not(
        Box<(
            Metadata<'a>, // not
            PropertyExpr<'a>,
        )>,
    ),
    Or(
        Box<(
            PropertyExpr<'a>,
            Metadata<'a>, // or
            PropertyExpr<'a>,
        )>,
    ),
    And(
        Box<(
            PropertyExpr<'a>,
            Metadata<'a>, // and
            PropertyExpr<'a>,
        )>,
    ),
    OverlapImpl(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // |->
            PropertyExpr<'a>,
        )>,
    ),
    NonoverlapImpl(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // |=>
            PropertyExpr<'a>,
        )>,
    ),
    Conditional(
        Box<(
            Metadata<'a>, // if
            Metadata<'a>, // (
            ExpressionOrDist<'a>,
            Metadata<'a>, // )
            PropertyExpr<'a>,
            Option<(
                Metadata<'a>, // else
                PropertyExpr<'a>,
            )>,
        )>,
    ),
    Case(
        Box<(
            Metadata<'a>, // case
            Metadata<'a>, // (
            ExpressionOrDist<'a>,
            Metadata<'a>, // )
            PropertyCaseItem<'a>,
            Vec<PropertyCaseItem<'a>>,
            Metadata<'a>, // endcase
        )>,
    ),
    OverlapFollowedBy(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // #-#
            PropertyExpr<'a>,
        )>,
    ),
    NonoverlapFollowedBy(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // #=#
            PropertyExpr<'a>,
        )>,
    ),
    Nexttime(
        Box<(
            Metadata<'a>, // nexttime
            PropertyExpr<'a>,
        )>,
    ),
    NexttimeExpr(
        Box<(
            Metadata<'a>, // nexttime
            Metadata<'a>, // [
            ConstantExpression<'a>,
            Metadata<'a>, // ]
            PropertyExpr<'a>,
        )>,
    ),
    SNexttime(
        Box<(
            Metadata<'a>, // s_nexttime
            PropertyExpr<'a>,
        )>,
    ),
    SNexttimeExpr(
        Box<(
            Metadata<'a>, // s_nexttime
            Metadata<'a>, // [
            ConstantExpression<'a>,
            Metadata<'a>, // ]
            PropertyExpr<'a>,
        )>,
    ),
    Always(
        Box<(
            Metadata<'a>, // always
            PropertyExpr<'a>,
        )>,
    ),
    AlwaysRange(
        Box<(
            Metadata<'a>, // always
            Metadata<'a>, // [
            CycleDelayConstRangeExpression<'a>,
            Metadata<'a>, // ]
            PropertyExpr<'a>,
        )>,
    ),
    SAlways(
        Box<(
            Metadata<'a>, // s_always
            Metadata<'a>, // [
            ConstantRange<'a>,
            Metadata<'a>, // ]
            PropertyExpr<'a>,
        )>,
    ),
    SEventually(
        Box<(
            Metadata<'a>, // s_eventually
            PropertyExpr<'a>,
        )>,
    ),
    Eventually(
        Box<(
            Metadata<'a>, // eventually
            Metadata<'a>, // [
            ConstantRange<'a>,
            Metadata<'a>, // ]
            PropertyExpr<'a>,
        )>,
    ),
    SEventuallyRange(
        Box<(
            Metadata<'a>, // s_eventually
            Metadata<'a>, // [
            CycleDelayConstRangeExpression<'a>,
            Metadata<'a>, // ]
            PropertyExpr<'a>,
        )>,
    ),
    Until(
        Box<(
            PropertyExpr<'a>,
            Metadata<'a>, // until
            PropertyExpr<'a>,
        )>,
    ),
    SUntil(
        Box<(
            PropertyExpr<'a>,
            Metadata<'a>, // s_until
            PropertyExpr<'a>,
        )>,
    ),
    UntilWith(
        Box<(
            PropertyExpr<'a>,
            Metadata<'a>, // until_with
            PropertyExpr<'a>,
        )>,
    ),
    SUntilWith(
        Box<(
            PropertyExpr<'a>,
            Metadata<'a>, // s_until_with
            PropertyExpr<'a>,
        )>,
    ),
    Implies(
        Box<(
            PropertyExpr<'a>,
            Metadata<'a>, // implies
            PropertyExpr<'a>,
        )>,
    ),
    Iff(
        Box<(
            PropertyExpr<'a>,
            Metadata<'a>, // iff
            PropertyExpr<'a>,
        )>,
    ),
    AcceptOn(
        Box<(
            Metadata<'a>, // accept_on
            Metadata<'a>, // (
            ExpressionOrDist<'a>,
            Metadata<'a>, // )
            PropertyExpr<'a>,
        )>,
    ),
    RejectOn(
        Box<(
            Metadata<'a>, // reject_on
            Metadata<'a>, // (
            ExpressionOrDist<'a>,
            Metadata<'a>, // )
            PropertyExpr<'a>,
        )>,
    ),
    SyncAcceptOn(
        Box<(
            Metadata<'a>, // sync_accept_on
            Metadata<'a>, // (
            ExpressionOrDist<'a>,
            Metadata<'a>, // )
            PropertyExpr<'a>,
        )>,
    ),
    SyncRejectOn(
        Box<(
            Metadata<'a>, // sync_reject_on
            Metadata<'a>, // (
            ExpressionOrDist<'a>,
            Metadata<'a>, // )
            PropertyExpr<'a>,
        )>,
    ),
    Instance(Box<PropertyInstance<'a>>),
    Clocking(Box<(ClockingEvent<'a>, PropertyExpr<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyCaseItem<'a> {
    Expr(
        Box<(
            ExpressionOrDist<'a>,
            Vec<(
                Metadata<'a>, // ,
                ExpressionOrDist<'a>,
            )>,
            Metadata<'a>, // :
            PropertyExpr<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Default(
        Box<(
            Metadata<'a>,         // default
            Option<Metadata<'a>>, // :
            PropertyExpr<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceDeclaration<'a>(
    pub Metadata<'a>, // sequence
    pub SequenceIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<SequencePortList<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
    pub Vec<AssertionVariableDeclaration<'a>>,
    pub SequenceExpr<'a>,
    pub Option<Metadata<'a>>, // ;
    pub Metadata<'a>,         // endsequence
    pub  Option<(
        Metadata<'a>, // :
        SequenceIdentifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct SequencePortList<'a>(
    pub SequencePortItem<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        SequencePortItem<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct SequencePortItem<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub  Option<(
        Metadata<'a>, // local
        Option<SequenceLvarPortDirection<'a>>,
    )>,
    pub SequenceFormalType<'a>,
    pub FormalPortIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub  Option<(
        Metadata<'a>, // =
        SequenceActualArg<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceLvarPortDirection<'a> {
    Input(Metadata<'a>),
    Inout(Metadata<'a>),
    Output(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceFormalType<'a> {
    DataTypeOrImplicit(Box<DataTypeOrImplicit<'a>>),
    Sequence(Box<Metadata<'a>>),
    Untyped(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceExpr<'a> {
    StartDelay(
        Box<(
            CycleDelayRange<'a>,
            SequenceExpr<'a>,
            Vec<(CycleDelayRange<'a>, SequenceExpr<'a>)>,
        )>,
    ),
    Delay(
        Box<(
            SequenceExpr<'a>,
            CycleDelayRange<'a>,
            SequenceExpr<'a>,
            Vec<(CycleDelayRange<'a>, SequenceExpr<'a>)>,
        )>,
    ),
    Expr(Box<(ExpressionOrDist<'a>, Option<BooleanAbbrev<'a>>)>),
    Inst(Box<(SequenceInstance<'a>, Option<SequenceAbbrev<'a>>)>),
    Paren(
        Box<(
            Metadata<'a>, // (
            SequenceExpr<'a>,
            Vec<(
                Metadata<'a>, // ,
                SequenceMatchItem<'a>,
            )>,
            Metadata<'a>, // )
            Option<SequenceAbbrev<'a>>,
        )>,
    ),
    And(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // and
            SequenceExpr<'a>,
        )>,
    ),
    Intersect(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // intersect
            SequenceExpr<'a>,
        )>,
    ),
    Or(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // or
            SequenceExpr<'a>,
        )>,
    ),
    FirstMatch(
        Box<(
            Metadata<'a>, // first_match
            Metadata<'a>, // (
            SequenceExpr<'a>,
            Vec<(
                Metadata<'a>, // ,
                SequenceMatchItem<'a>,
            )>,
            Metadata<'a>, // )
        )>,
    ),
    Throughout(
        Box<(
            ExpressionOrDist<'a>,
            Metadata<'a>, // throughout
            SequenceExpr<'a>,
        )>,
    ),
    Within(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // within
            SequenceExpr<'a>,
        )>,
    ),
    Clocking(Box<(ClockingEvent<'a>, SequenceExpr<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CycleDelayRange<'a> {
    Primary(
        Box<(
            Metadata<'a>, // ##
            ConstantPrimary<'a>,
        )>,
    ),
    Range(
        Box<(
            Metadata<'a>, // ##
            Metadata<'a>, // [
            CycleDelayConstRangeExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    Star(
        Box<(
            Metadata<'a>, // ##
            Metadata<'a>, // [
            Metadata<'a>, // *
            Metadata<'a>, // ]
        )>,
    ),
    Plus(
        Box<(
            Metadata<'a>, // ##
            Metadata<'a>, // [
            Metadata<'a>, // +
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceMethodCall<'a>(
    pub SequenceInstance<'a>,
    pub Metadata<'a>, // .
    pub MethodIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceMatchItem<'a> {
    Operator(Box<OperatorAssignment<'a>>),
    IncOrDec(Box<IncOrDecExpression<'a>>),
    Subroutine(Box<SubroutineCall<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceInstance<'a>(
    pub PsOrHierarchicalSequenceIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<SequenceListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceListOfArguments<'a> {
    PartialIdentifier(
        Box<(
            Option<SequenceActualArg<'a>>,
            Vec<(
                Metadata<'a>, // ,
                Option<SequenceActualArg<'a>>,
            )>,
            Vec<(
                Metadata<'a>, // ,
                Metadata<'a>, // .
                Identifier<'a>,
                Metadata<'a>, // (
                Option<SequenceActualArg<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Identifier(
        Box<(
            Metadata<'a>, // .
            Identifier<'a>,
            Metadata<'a>, // (
            Option<SequenceActualArg<'a>>,
            Metadata<'a>, // )
            Vec<(
                Metadata<'a>, // ,
                Metadata<'a>, // .
                Identifier<'a>,
                Metadata<'a>, // (
                Option<SequenceActualArg<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceActualArg<'a> {
    Event(Box<EventExpression<'a>>),
    Sequence(Box<SequenceExpr<'a>>),
    Dollar(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BooleanAbbrev<'a> {
    Consecutive(Box<ConsecutiveRepetition<'a>>),
    Nonconsecutive(Box<NonconsecutiveRepetition<'a>>),
    Goto(Box<GotoRepetition<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceAbbrev<'a>(pub ConsecutiveRepetition<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum ConsecutiveRepetition<'a> {
    Expr(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // *
            ConstOrRangeExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    Star(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // *
            Metadata<'a>, // ]
        )>,
    ),
    Plus(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // +
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonconsecutiveRepetition<'a>(
    pub Metadata<'a>, // [
    pub Metadata<'a>, // =
    pub ConstOrRangeExpression<'a>,
    pub Metadata<'a>, // ]
);

#[derive(Clone, Debug, PartialEq)]
pub struct GotoRepetition<'a>(
    pub Metadata<'a>, // [
    pub Metadata<'a>, //  ->
    pub ConstOrRangeExpression<'a>,
    pub Metadata<'a>, // ]
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstOrRangeExpression<'a> {
    Expr(Box<ConstantExpression<'a>>),
    Range(Box<CycleDelayConstRangeExpression<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CycleDelayConstRangeExpression<'a> {
    Bounded(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>, // :
            ConstantExpression<'a>,
        )>,
    ),
    Unbounded(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>, // :
            Metadata<'a>, // $
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssertionVariableDeclaration<'a>(
    pub VarDataType<'a>,
    pub ListOfVariableDeclAssignments<'a>,
    pub Metadata<'a>, // ;
);
