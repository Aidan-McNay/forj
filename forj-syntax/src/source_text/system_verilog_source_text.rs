// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.2
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceText<'a>(
    pub Vec<NonTriviaToken<'a>>, // Leading whitespace/comments/directives
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<Description<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum Description<'a> {
    ModuleDeclaration(Box<ModuleDeclaration<'a>>),
    UdpDeclaration(Box<UdpDeclaration<'a>>),
    InterfaceDeclaration(Box<InterfaceDeclaration<'a>>),
    ProgramDeclaration(Box<ProgramDeclaration<'a>>),
    PackageDeclaration(Box<PackageDeclaration<'a>>),
    DescriptionPackageItem(Box<DescriptionPackageItem<'a>>),
    DescriptionBindDirective(Box<DescriptionBindDirective<'a>>),
    ConfigDeclaration(Box<ConfigDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescriptionPackageItem<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub PackageItem<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct DescriptionBindDirective<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub BindDirective<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleNonansiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub ModuleKeyword<'a>,
    pub Option<Lifetime<'a>>,
    pub ModuleIdentifier<'a>,
    pub Vec<PackageImportDeclaration<'a>>,
    pub Option<ParameterPortList<'a>>,
    pub ListOfPorts<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleAnsiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub ModuleKeyword<'a>,
    pub Option<Lifetime<'a>>,
    pub ModuleIdentifier<'a>,
    pub Vec<PackageImportDeclaration<'a>>,
    pub Option<ParameterPortList<'a>>,
    pub Option<ListOfPortDeclarations<'a>>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleDeclaration<'a> {
    Nonansi(Box<ModuleDeclarationNonansi<'a>>),
    Ansi(Box<ModuleDeclarationAnsi<'a>>),
    Wildcard(Box<ModuleDeclarationWildcard<'a>>),
    ExternNonansi(Box<ModuleDeclarationExternNonansi<'a>>),
    ExternAnsi(Box<ModuleDeclarationExternAnsi<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationNonansi<'a>(
    pub ModuleNonansiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<ModuleItem<'a>>,
    pub Metadata<'a>,
    pub Option<(Metadata<'a>, ModuleIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationAnsi<'a>(
    pub ModuleAnsiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<NonPortModuleItem<'a>>,
    pub Metadata<'a>,
    pub Option<(Metadata<'a>, ModuleIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationWildcard<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub ModuleKeyword<'a>,
    pub Option<Lifetime<'a>>,
    pub ModuleIdentifier<'a>,
    pub Metadata<'a>, // (
    pub Metadata<'a>, // .
    pub Metadata<'a>, // *
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<ModuleItem<'a>>,
    pub Metadata<'a>, // endmodule
    pub Option<(Metadata<'a>, ModuleIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternNonansi<'a>(
    pub Metadata<'a>,
    pub ModuleNonansiHeader<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternAnsi<'a>(
    pub Metadata<'a>,
    pub ModuleAnsiHeader<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleKeyword<'a> {
    Module(Metadata<'a>),
    Macromodule(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceDeclaration<'a> {
    Nonansi(Box<InterfaceDeclarationNonansi<'a>>),
    Ansi(Box<InterfaceDeclarationAnsi<'a>>),
    Wildcard(Box<InterfaceDeclarationWildcard<'a>>),
    ExternNonansi(Box<InterfaceDeclarationExternNonansi<'a>>),
    ExternAnsi(Box<InterfaceDeclarationExternAnsi<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationNonansi<'a>(
    pub InterfaceNonansiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<InterfaceItem<'a>>,
    pub Metadata<'a>, // endinterface
    pub Option<(Metadata<'a>, InterfaceIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationAnsi<'a>(
    pub InterfaceAnsiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<NonPortInterfaceItem<'a>>,
    pub Metadata<'a>, // endinterface
    pub Option<(Metadata<'a>, InterfaceIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationWildcard<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // interface
    pub InterfaceIdentifier<'a>,
    pub Metadata<'a>, // (
    pub Metadata<'a>, // .
    pub Metadata<'a>, // *
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<InterfaceItem<'a>>,
    pub Metadata<'a>, // endinterface
    pub Option<(Metadata<'a>, InterfaceIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationExternNonansi<'a>(
    pub Metadata<'a>,
    pub InterfaceNonansiHeader<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationExternAnsi<'a>(
    pub Metadata<'a>,
    pub InterfaceAnsiHeader<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceNonansiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // interface
    pub Option<Lifetime<'a>>,
    pub InterfaceIdentifier<'a>,
    pub Vec<PackageImportDeclaration<'a>>,
    pub Option<ParameterPortList<'a>>,
    pub ListOfPorts<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceAnsiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // interface
    pub Option<Lifetime<'a>>,
    pub InterfaceIdentifier<'a>,
    pub Vec<PackageImportDeclaration<'a>>,
    pub Option<ParameterPortList<'a>>,
    pub Option<ListOfPortDeclarations<'a>>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum ProgramDeclaration<'a> {
    Nonansi(Box<ProgramDeclarationNonansi<'a>>),
    Ansi(Box<ProgramDeclarationAnsi<'a>>),
    Wildcard(Box<ProgramDeclarationWildcard<'a>>),
    ExternNonansi(Box<ProgramDeclarationExternNonansi<'a>>),
    ExternAnsi(Box<ProgramDeclarationExternAnsi<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationNonansi<'a>(
    pub ProgramNonansiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<ProgramItem<'a>>,
    pub Metadata<'a>, // endprogram
    pub Option<(Metadata<'a>, ProgramIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationAnsi<'a>(
    pub ProgramAnsiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<NonPortProgramItem<'a>>,
    pub Metadata<'a>, // endprogram
    pub Option<(Metadata<'a>, ProgramIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationWildcard<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // program
    pub ProgramIdentifier<'a>,
    pub Metadata<'a>, // (
    pub Metadata<'a>, // .
    pub Metadata<'a>, // *
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<ProgramItem<'a>>,
    pub Metadata<'a>, // endprogram
    pub Option<(Metadata<'a>, ProgramIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationExternNonansi<'a>(
    pub Metadata<'a>,
    pub ProgramNonansiHeader<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationExternAnsi<'a>(
    pub Metadata<'a>,
    pub ProgramAnsiHeader<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramNonansiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // program
    pub Option<Lifetime<'a>>,
    pub ProgramIdentifier<'a>,
    pub Vec<PackageImportDeclaration<'a>>,
    pub Option<ParameterPortList<'a>>,
    pub ListOfPorts<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramAnsiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // program
    pub Option<Lifetime<'a>>,
    pub ProgramIdentifier<'a>,
    pub Vec<PackageImportDeclaration<'a>>,
    pub Option<ParameterPortList<'a>>,
    pub Option<ListOfPortDeclarations<'a>>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerDeclaration<'a>(
    pub Metadata<'a>, // checker
    pub CheckerIdentifier<'a>,
    pub Option<(Metadata<'a>, CheckerPortList<'a>, Metadata<'a>)>,
    pub Metadata<'a>,
    pub Vec<(Vec<AttributeInstance<'a>>, CheckerOrGenerateItem<'a>)>,
    pub Metadata<'a>, // endchecker
    pub Option<(Metadata<'a>, CheckerIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassDeclaration<'a>(
    pub Option<Metadata<'a>>, // virtual
    pub Metadata<'a>,         // class
    pub Option<FinalSpecifier<'a>>,
    pub ClassIdentifier<'a>,
    pub Option<ParameterPortList<'a>>,
    pub  Option<(
        Metadata<'a>, // extends
        ClassType<'a>,
        Option<(
            Metadata<'a>, // (
            ClassDeclarationExtensionArguments<'a>,
            Metadata<'a>, // )
        )>,
    )>,
    pub  Option<(
        Metadata<'a>, // implements
        InterfaceClassType<'a>,
        Vec<(Metadata<'a>, InterfaceClassType<'a>)>,
    )>,
    pub Metadata<'a>, // ;
    pub Vec<ClassItem<'a>>,
    pub Metadata<'a>, // endclass
    pub Option<(Metadata<'a>, ClassIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ClassDeclarationExtensionArguments<'a> {
    ListOfArguments(Box<ListOfArguments<'a>>),
    Default(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceClassDeclaration<'a>(
    pub Metadata<'a>, // interface
    pub Metadata<'a>, // class
    pub ClassIdentifier<'a>,
    pub Option<ParameterPortList<'a>>,
    pub  Option<(
        Metadata<'a>, // extends
        InterfaceClassType<'a>,
        Vec<(Metadata<'a>, InterfaceClassType<'a>)>,
    )>,
    pub Metadata<'a>, // ;
    pub Vec<InterfaceClassItem<'a>>,
    pub Metadata<'a>, // endclass
    pub Option<(Metadata<'a>, ClassIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PackageDeclaration<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Metadata<'a>, // package
    pub Option<Lifetime<'a>>,
    pub PackageIdentifier<'a>,
    pub Metadata<'a>, // ;
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<(Vec<AttributeInstance<'a>>, PackageItem<'a>)>,
    pub Metadata<'a>, // endpackage
    pub Option<(Metadata<'a>, PackageIdentifier<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum TimeunitsDeclaration<'a> {
    Timeunit(
        Metadata<'a>, // timeunit
        TimeLiteral<'a>,
        Option<(Metadata<'a>, TimeLiteral<'a>)>,
        Metadata<'a>, // ;
    ),
    Timeprecision(
        Metadata<'a>, // timeprecision
        TimeLiteral<'a>,
        Metadata<'a>, // ;
    ),
    Timeunitprecision(
        Metadata<'a>, // timeunit
        TimeLiteral<'a>,
        Metadata<'a>, // ;
        Metadata<'a>, // timeprecision
        TimeLiteral<'a>,
        Metadata<'a>, // ;
    ),
    Timeprecisionunit(
        Metadata<'a>, // timeprecision
        TimeLiteral<'a>,
        Metadata<'a>, // ;
        Metadata<'a>, // timeunit
        TimeLiteral<'a>,
        Metadata<'a>, // ;
    ),
}
