// =======================================================================
// identifiers.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.9.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct BlockIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct BinIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CIdentifier<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CellIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassVariableIdentifier<'a>(pub VariableIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ClockingIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CovergroupIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CovergroupVariableIdentifier<'a>(pub VariableIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CoverPointIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CrossIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicArrayVariableIdentifier<'a>(pub VariableIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct EnumIdentifier<'a>(pub Identifier<'a>);

pub type EscapedIdentifier<'a> = (&'a str, Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct FormalIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct FormalPortIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct GenerateBlockIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct GenvarIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalArrayIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalBlockIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalEventIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalIdentifier<'a>(
    pub Option<(Metadata<'a>, Metadata<'a>)>, // $root .
    pub  Vec<(
        Identifier<'a>,
        ConstantBitSelect<'a>,
        Metadata<'a>, // .
    )>,
    pub Identifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalNetIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalParameterIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalPropertyIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalSequenceIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalTaskIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalTfIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct HierarchicalVariableIdentifier<'a>(pub HierarchicalIdentifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum Identifier<'a> {
    SimpleIdentifier(SimpleIdentifier<'a>),
    EscapedIdentifier(EscapedIdentifier<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexVariableIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfacePortIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InoutPortIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InputPortIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InstanceIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct LibraryIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct MemberIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct MethodIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ModportIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct NetIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct NettypeIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct OutputPortIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PackageIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum PackageScope<'a> {
    Identifier(Box<(PackageIdentifier<'a>, Metadata<'a>)>),
    Unit(Box<(Metadata<'a>, Metadata<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PortIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PsClassIdentifier<'a>(
    pub Option<PackageScope<'a>>,
    pub ClassIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PsCovergroupIdentifier<'a>(
    pub Option<PackageScope<'a>>,
    pub CovergroupIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PsCheckerIdentifier<'a>(
    pub Option<PackageScope<'a>>,
    pub CheckerIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PsIdentifier<'a>(pub Option<PackageScope<'a>>, pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PsOrHierarchicalArrayIdentifier<'a>(
    pub Option<ImplicitClassHandleOrClassScopeOrPackageScope<'a>>,
    pub HierarchicalArrayIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum PsOrHierarchicalNetIdentifier<'a> {
    PackageScope(Option<PackageScope<'a>>, NetIdentifier<'a>),
    Hierarchical(HierarchicalNetIdentifier<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PsOrHierarchicalPropertyIdentifier<'a> {
    PackageScope(Option<PackageScope<'a>>, PropertyIdentifier<'a>),
    Hierarchical(HierarchicalPropertyIdentifier<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PsOrHierarchicalSequenceIdentifier<'a> {
    PackageScope(Option<PackageScope<'a>>, SequenceIdentifier<'a>),
    Hierarchical(HierarchicalSequenceIdentifier<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PsOrHierarchicalTfIdentifier<'a> {
    Scoped(
        Option<ImplicitClassHandleOrClassScopeOrPackageScope<'a>>,
        TfIdentifier<'a>,
    ),
    Hierarchical(HierarchicalTfIdentifier<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PackageOrClassScope<'a> {
    ClassScope(ClassScope<'a>),
    PackageScope(PackageScope<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PsParameterIdentifier<'a> {
    Scoped(Option<PackageOrClassScope<'a>>, ParameterIdentifier<'a>),
    Generated(
        Vec<(
            GenerateBlockIdentifier<'a>,
            Option<(Metadata<'a>, ConstantExpression<'a>, Metadata<'a>)>,
            Metadata<'a>, // .
        )>,
        ParameterIdentifier<'a>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PsTypeIdentifierScope<'a> {
    LocalScope(Metadata<'a>, Metadata<'a>),
    ClassScope(ClassScope<'a>),
    PackageScope(PackageScope<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PsTypeIdentifier<'a>(
    pub Option<PsTypeIdentifierScope<'a>>,
    pub TypeIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct RsProductionIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct SignalIdentifier<'a>(pub Identifier<'a>);

pub type SimpleIdentifier<'a> = (&'a str, Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct SpecparamIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct SystemTfIdentifier<'a>(pub &'a str, pub Metadata<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TaskIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TfIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TerminalIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TopmoduleIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TypeIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct UdpIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct VariableIdentifier<'a>(pub Identifier<'a>);
