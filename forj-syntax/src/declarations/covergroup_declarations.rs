// =======================================================================
// covergroup_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.11
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CovergroupDeclaration<'a> {
    Initial(
        Box<(
            Metadata<'a>, // covergroup
            CovergroupIdentifier<'a>,
            Option<(
                Metadata<'a>, // (
                Option<TfPortList<'a>>,
                Metadata<'a>, // )
            )>,
            Option<CoverageEvent<'a>>,
            Metadata<'a>, // ;
            Vec<CoverageSpecOrOption<'a>>,
            Metadata<'a>, // endgroup
            Option<(
                Metadata<'a>, // :
                CovergroupIdentifier<'a>,
            )>,
        )>,
    ),
    Extends(
        Box<(
            Metadata<'a>, // covergroup
            Metadata<'a>, // extends
            CovergroupIdentifier<'a>,
            Metadata<'a>, // ;
            Vec<CoverageSpecOrOption<'a>>,
            Metadata<'a>, // endgroup
            Option<(
                Metadata<'a>, // :
                CovergroupIdentifier<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoverageSpecOrOption<'a> {
    Spec(Box<(Vec<AttributeInstance<'a>>, CoverageSpec<'a>)>),
    Option(
        Box<(
            Vec<AttributeInstance<'a>>,
            CoverageOption<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoverageOption<'a> {
    Option(
        Box<(
            Metadata<'a>, // option
            Metadata<'a>, // .
            MemberIdentifier<'a>,
            Metadata<'a>, // =
            Expression<'a>,
        )>,
    ),
    TypeOption(
        Box<(
            Metadata<'a>, // type_option
            Metadata<'a>, // .
            MemberIdentifier<'a>,
            Metadata<'a>, // =
            ConstantExpression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoverageSpec<'a> {
    Point(Box<CoverPoint<'a>>),
    Cross(Box<CoverCross<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoverageEvent<'a> {
    Clocking(Box<ClockingEvent<'a>>),
    Function(
        Box<(
            Metadata<'a>, // with
            Metadata<'a>, // function
            Metadata<'a>, // sample
            Metadata<'a>, // (
            Option<TfPortList<'a>>,
            Metadata<'a>, // )
        )>,
    ),
    Block(
        Box<(
            Metadata<'a>, // @@
            Metadata<'a>, // (
            BlockEventExpression<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockEventExpression<'a> {
    Or(
        Box<(
            BlockEventExpression<'a>,
            Metadata<'a>,
            BlockEventExpression<'a>,
        )>,
    ),
    Begin(
        Box<(
            Metadata<'a>, // begin
            HierarchicalBtfIdentifier<'a>,
        )>,
    ),
    End(
        Box<(
            Metadata<'a>, // end
            HierarchicalBtfIdentifier<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HierarchicalIdentifierOrClassScope<'a> {
    Identifier(
        Box<(
            HierarchicalIdentifier<'a>,
            Metadata<'a>, // .
        )>,
    ),
    Scope(Box<ClassScope<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HierarchicalBtfIdentifier<'a> {
    Tf(Box<HierarchicalTfIdentifier<'a>>),
    Block(Box<HierarchicalBlockIdentifier<'a>>),
    Method(
        Box<(
            Option<HierarchicalIdentifierOrClassScope<'a>>,
            MethodIdentifier<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoverPoint<'a>(
    pub  Option<(
        Option<DataTypeOrImplicit<'a>>,
        CoverPointIdentifier<'a>,
        Metadata<'a>, // :
    )>,
    pub Metadata<'a>, // coverpoint
    pub Expression<'a>,
    pub  Option<(
        Metadata<'a>, // iff
        Metadata<'a>, // (
        Expression<'a>,
        Metadata<'a>, // )
    )>,
    pub BinsOrEmpty<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum BinsOrEmpty<'a> {
    Bins(
        Box<(
            Metadata<'a>, // {
            Vec<AttributeInstance<'a>>,
            Vec<(
                BinsOrOptions<'a>,
                Metadata<'a>, // ;
            )>,
            Metadata<'a>, // }
        )>,
    ),
    Empty(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinsOrOptions<'a> {
    Coverage(Box<CoverageOption<'a>>),
    Range(
        Box<(
            Option<Metadata<'a>>, // wildcard
            BinsKeyword<'a>,
            BinIdentifier<'a>,
            Option<(
                Metadata<'a>, // [
                Option<CovergroupExpression<'a>>,
                Metadata<'a>, // ]
            )>,
            Metadata<'a>, // =
            Metadata<'a>, // {
            CovergroupRangeList<'a>,
            Metadata<'a>, // }
            Option<(
                Metadata<'a>, // with
                Metadata<'a>, // (
                WithCovergroupExpression<'a>,
                Metadata<'a>, // )
            )>,
            Option<(
                Metadata<'a>, // iff
                Metadata<'a>, // (
                Expression<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Point(
        Box<(
            Option<Metadata<'a>>, // wildcard
            BinsKeyword<'a>,
            BinIdentifier<'a>,
            Option<(
                Metadata<'a>, // [
                Option<CovergroupExpression<'a>>,
                Metadata<'a>, // ]
            )>,
            Metadata<'a>, // =
            CoverPointIdentifier<'a>,
            Metadata<'a>, // with
            Metadata<'a>, // (
            WithCovergroupExpression<'a>,
            Metadata<'a>, // )
            Option<(
                Metadata<'a>, // iff
                Metadata<'a>, // (
                Expression<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Set(
        Box<(
            Option<Metadata<'a>>, // wildcard
            BinsKeyword<'a>,
            BinIdentifier<'a>,
            Option<(
                Metadata<'a>, // [
                Option<CovergroupExpression<'a>>,
                Metadata<'a>, // ]
            )>,
            Metadata<'a>, // =
            SetCovergroupExpression<'a>,
            Option<(
                Metadata<'a>, // iff
                Metadata<'a>, // (
                Expression<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Trans(
        Box<(
            Option<Metadata<'a>>, // wildcard
            BinsKeyword<'a>,
            BinIdentifier<'a>,
            Option<(
                Metadata<'a>, // [
                Metadata<'a>, // ]
            )>,
            Metadata<'a>, // =
            TransList<'a>,
            Option<(
                Metadata<'a>, // iff
                Metadata<'a>, // (
                Expression<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Default(
        Box<(
            BinsKeyword<'a>,
            BinIdentifier<'a>,
            Option<(
                Metadata<'a>, // [
                Option<CovergroupExpression<'a>>,
                Metadata<'a>, // ]
            )>,
            Metadata<'a>, // =
            Metadata<'a>, // default
            Option<(
                Metadata<'a>, // iff
                Metadata<'a>, // (
                Expression<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    DefaultSequence(
        Box<(
            BinsKeyword<'a>,
            BinIdentifier<'a>,
            Metadata<'a>, // =
            Metadata<'a>, // default
            Metadata<'a>, // sequence
            Option<(
                Metadata<'a>, // iff
                Metadata<'a>, // (
                Expression<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinsKeyword<'a> {
    Bins(Metadata<'a>),
    IllegalBins(Metadata<'a>),
    IgnoreBins(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransList<'a>(
    pub Metadata<'a>, // (
    pub TransSet<'a>,
    pub Metadata<'a>, // )
    pub  Vec<(
        Metadata<'a>, // ,
        Metadata<'a>, // (
        TransSet<'a>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct TransSet<'a>(
    pub TransRangeList<'a>,
    pub  Vec<(
        Metadata<'a>, // =>
        TransRangeList<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum TransRangeList<'a> {
    NoRepeat(Box<TransItem<'a>>),
    Repeat(
        Box<(
            TransItem<'a>,
            Metadata<'a>, // [
            Metadata<'a>, // *
            RepeatRange<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    GotoRepeat(
        Box<(
            TransItem<'a>,
            Metadata<'a>, // [
            Metadata<'a>, // ->
            RepeatRange<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    NonconsecutiveRepeat(
        Box<(
            TransItem<'a>,
            Metadata<'a>, // [
            Metadata<'a>, // =
            RepeatRange<'a>,
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransItem<'a>(pub CovergroupRangeList<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum RepeatRange<'a> {
    Expr(Box<CovergroupExpression<'a>>),
    Range(
        Box<(
            CovergroupExpression<'a>,
            Metadata<'a>, // :
            CovergroupExpression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoverCross<'a>(
    pub  Option<(
        CrossIdentifier<'a>,
        Metadata<'a>, // :
    )>,
    pub Metadata<'a>, // cross
    pub ListOfCrossItems<'a>,
    pub  Option<(
        Metadata<'a>, // iff
        Metadata<'a>, // (
        Expression<'a>,
        Metadata<'a>, // )
    )>,
    pub CrossBody<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfCrossItems<'a>(
    pub CrossItem<'a>,
    pub Metadata<'a>, // ,
    pub CrossItem<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        CrossItem<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum CrossItem<'a> {
    CoverPoint(Box<CoverPointIdentifier<'a>>),
    Variable(Box<VariableIdentifier<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CrossBody<'a> {
    Items(
        Box<(
            Metadata<'a>, // {
            Vec<CrossBodyItem<'a>>,
            Metadata<'a>, // }
        )>,
    ),
    Null(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CrossBodyItem<'a> {
    Function(Box<FunctionDeclaration<'a>>),
    BinsSelection(
        Box<(
            BinsSelectionOrOption<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinsSelectionOrOption<'a> {
    Coverage(Box<(Vec<AttributeInstance<'a>>, CoverageOption<'a>)>),
    Bins(Box<(Vec<AttributeInstance<'a>>, BinsSelection<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinsSelection<'a>(
    pub BinsKeyword<'a>,
    pub BinIdentifier<'a>,
    pub Metadata<'a>, // =
    pub SelectExpression<'a>,
    pub  Option<(
        Metadata<'a>, // iff
        Metadata<'a>, // (
        Expression<'a>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum SelectExpression<'a> {
    Condition(Box<SelectCondition<'a>>),
    Not(
        Box<(
            Metadata<'a>, // !
            SelectCondition<'a>,
        )>,
    ),
    And(
        Box<(
            SelectExpression<'a>,
            Metadata<'a>, // &&
            SelectExpression<'a>,
        )>,
    ),
    Or(
        Box<(
            SelectExpression<'a>,
            Metadata<'a>, // ||
            SelectExpression<'a>,
        )>,
    ),
    Paren(
        Box<(
            Metadata<'a>, // (
            SelectExpression<'a>,
            Metadata<'a>, // )
        )>,
    ),
    With(
        Box<(
            SelectExpression<'a>,
            Metadata<'a>, // with
            Metadata<'a>, // (
            WithCovergroupExpression<'a>,
            Metadata<'a>, // )
            Option<(
                Metadata<'a>, // matches
                IntegerCovergroupExpression<'a>,
            )>,
        )>,
    ),
    CrossIdentifier(Box<CrossIdentifier<'a>>),
    CrossSet(
        Box<(
            CrossSetExpression<'a>,
            Option<(
                Metadata<'a>, // matches
                IntegerCovergroupExpression<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectCondition<'a>(
    pub Metadata<'a>, // binsof
    pub Metadata<'a>, // (
    pub BinsExpression<'a>,
    pub Metadata<'a>, // )
    pub  Option<(
        Metadata<'a>, // intersect
        Metadata<'a>, // {
        CovergroupRangeList<'a>,
        Metadata<'a>, // }
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum BinsExpression<'a> {
    Variable(Box<VariableIdentifier<'a>>),
    CoverPoint(
        Box<(
            CoverPointIdentifier<'a>,
            Option<(
                Metadata<'a>, // .
                BinIdentifier<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CovergroupRangeList<'a>(
    pub CovergroupValueRange<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        CovergroupValueRange<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum CovergroupValueRange<'a> {
    Expr(Box<CovergroupExpression<'a>>),
    ExprRange(
        Box<(
            Metadata<'a>, // [
            CovergroupExpression<'a>,
            Metadata<'a>, // :
            CovergroupExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    DollarLow(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // $
            Metadata<'a>, // :
            CovergroupExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    DollarHigh(
        Box<(
            Metadata<'a>, // [
            CovergroupExpression<'a>,
            Metadata<'a>, // :
            Metadata<'a>, // $
            Metadata<'a>, // ]
        )>,
    ),
    AbsoluteTolerance(
        Box<(
            Metadata<'a>, // [
            CovergroupExpression<'a>,
            Metadata<'a>, // +/-
            CovergroupExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    RelativeTolerance(
        Box<(
            Metadata<'a>, // [
            CovergroupExpression<'a>,
            Metadata<'a>, // +%-
            CovergroupExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithCovergroupExpression<'a>(pub CovergroupExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct SetCovergroupExpression<'a>(pub CovergroupExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerCovergroupExpression<'a> {
    Expression(Box<CovergroupExpression<'a>>),
    Dollar(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CrossSetExpression<'a>(pub CovergroupExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CovergroupExpression<'a>(pub Expression<'a>);
