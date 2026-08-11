// =======================================================================
// module_items.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.4
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum SeveritySystemTask<'a> {
    Fatal(Box<FatalSeveritySystemTask<'a>>),
    Error(Box<ErrorSeveritySystemTask<'a>>),
    Warning(Box<WarningSeveritySystemTask<'a>>),
    Info(Box<InfoSeveritySystemTask<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FatalSeveritySystemTask<'a>(
    pub Metadata<'a>, // $fatal
    pub  Option<(
        Metadata<'a>, // (
        FinishNumber<'a>,
        Option<(
            Metadata<'a>, // ,
            ListOfArguments<'a>,
        )>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorSeveritySystemTask<'a>(
    pub Metadata<'a>, // $error
    pub  Option<(
        Metadata<'a>, // (
        Option<ListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct WarningSeveritySystemTask<'a>(
    pub Metadata<'a>, // $warning
    pub  Option<(
        Metadata<'a>, // (
        Option<ListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct InfoSeveritySystemTask<'a>(
    pub Metadata<'a>, // $info
    pub  Option<(
        Metadata<'a>, // (
        Option<ListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum FinishNumber<'a> {
    Zero(Metadata<'a>),
    One(Metadata<'a>),
    Two(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElaborationSeveritySystemTask<'a>(pub SeveritySystemTask<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleCommonItem<'a> {
    ModuleOrGenerateDeclaration(Box<ModuleOrGenerateItemDeclaration<'a>>),
    Interface(Box<InterfaceInstantiation<'a>>),
    Program(Box<ProgramInstantiation<'a>>),
    Assertion(Box<AssertionItem<'a>>),
    Bind(Box<BindDirective<'a>>),
    Assign(Box<ContinuousAssign<'a>>),
    Alias(Box<NetAlias<'a>>),
    Initial(Box<InitialConstruct<'a>>),
    Final(Box<FinalConstruct<'a>>),
    Always(Box<AlwaysConstruct<'a>>),
    LoopGenerate(Box<LoopGenerateConstruct<'a>>),
    ConditionalGenerateConstruct(Box<ConditionalGenerateConstruct<'a>>),
    SystemSeverity(Box<ElaborationSeveritySystemTask<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleItem<'a> {
    Port(
        Box<(
            PortDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    NonPort(Box<NonPortModuleItem<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleOrGenerateItem<'a> {
    ParameterOverride(Box<(Vec<AttributeInstance<'a>>, ParameterOverride<'a>)>),
    Gate(Box<(Vec<AttributeInstance<'a>>, GateInstantiation<'a>)>),
    Udp(Box<(Vec<AttributeInstance<'a>>, UdpInstantiation<'a>)>),
    Module(Box<(Vec<AttributeInstance<'a>>, ModuleInstantiation<'a>)>),
    ModuleCommon(Box<(Vec<AttributeInstance<'a>>, ModuleCommonItem<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleOrGenerateItemDeclaration<'a> {
    PackageOrGenerate(Box<PackageOrGenerateItemDeclaration<'a>>),
    Genvar(Box<GenvarDeclaration<'a>>),
    Clocking(Box<ClockingDeclaration<'a>>),
    DefaultClocking(
        Box<(
            Metadata<'a>, // default
            Metadata<'a>, // clocking
            ClockingIdentifier<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    DefaultDisable(
        Box<(
            Metadata<'a>, // default
            Metadata<'a>, // disable
            Metadata<'a>, // iff
            ExpressionOrDist<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NonPortModuleItem<'a> {
    Region(Box<GenerateRegion<'a>>),
    ModuleOrGenerate(Box<ModuleOrGenerateItem<'a>>),
    Specify(Box<SpecifyBlock<'a>>),
    Specparam(Box<(Vec<AttributeInstance<'a>>, SpecparamAssignment<'a>)>),
    Program(Box<ProgramDeclaration<'a>>),
    Module(Box<ModuleDeclaration<'a>>),
    Interface(Box<InterfaceDeclaration<'a>>),
    Timeunits(Box<TimeunitsDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterOverride<'a>(
    pub Metadata<'a>, // defparam
    pub ListOfDefparamAssignments<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum BindDirective<'a> {
    Scope(
        Box<(
            Metadata<'a>, // bind
            BindTargetScope<'a>,
            Option<(
                Metadata<'a>, // :
                BindTargetInstanceList<'a>,
            )>,
            BindInstantiation<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Instance(
        Box<(
            Metadata<'a>, // bind
            BindTargetInstance<'a>,
            BindInstantiation<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BindTargetScope<'a> {
    Module(ModuleIdentifier<'a>),
    Interface(InterfaceIdentifier<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BindTargetInstance<'a>(
    pub HierarchicalIdentifier<'a>,
    pub ConstantBitSelect<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct BindTargetInstanceList<'a>(
    pub BindTargetInstance<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        BindTargetInstance<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum BindInstantiation<'a> {
    Program(Box<ProgramInstantiation<'a>>),
    Module(Box<ModuleInstantiation<'a>>),
    Interface(Box<InterfaceInstantiation<'a>>),
    Checker(Box<CheckerInstantiation<'a>>),
}
