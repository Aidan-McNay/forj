// =======================================================================
// program_items.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.7
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ProgramItem<'a> {
    Port(
        Box<(
            PortDeclaration<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    NonPort(Box<NonPortProgramItem<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NonPortProgramItem<'a> {
    Assign(Box<(Vec<AttributeInstance<'a>>, ContinuousAssign<'a>)>),
    ModuleOrGenerateDeclaration(
        Box<(
            Vec<AttributeInstance<'a>>,
            ModuleOrGenerateItemDeclaration<'a>,
        )>,
    ),
    Initial(Box<(Vec<AttributeInstance<'a>>, InitialConstruct<'a>)>),
    Final(Box<(Vec<AttributeInstance<'a>>, FinalConstruct<'a>)>),
    Assertion(Box<(Vec<AttributeInstance<'a>>, ConcurrentAssertionItem<'a>)>),
    Timeunits(Box<TimeunitsDeclaration<'a>>),
    Generate(Box<ProgramGenerateItem<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProgramGenerateItem<'a> {
    Loop(Box<LoopGenerateConstruct<'a>>),
    Conditional(Box<ConditionalGenerateConstruct<'a>>),
    Region(Box<GenerateRegion<'a>>),
    ElaborationSeverity(Box<ElaborationSeveritySystemTask<'a>>),
}
