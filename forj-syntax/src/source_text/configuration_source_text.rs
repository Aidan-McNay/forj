// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.1.5
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigDeclaration<'a>(
    pub Metadata<'a>, // config
    pub ConfigIdentifier<'a>,
    pub Metadata<'a>, // ;
    pub  Vec<(
        LocalParameterDeclaration<'a>,
        Metadata<'a>, // ;
    )>,
    pub DesignStatement<'a>,
    pub Vec<ConfigRuleStatement<'a>>,
    pub Metadata<'a>, // endconfig
    pub  Option<(
        Metadata<'a>, // :
        ConfigIdentifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct DesignStatement<'a>(
    pub Metadata<'a>, // design
    pub  Vec<(
        Option<(
            LibraryIdentifier<'a>,
            Metadata<'a>, // .
        )>,
        CellIdentifier<'a>,
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigRuleStatement<'a> {
    DefaultLiblist(
        Box<(
            DefaultClause<'a>,
            LiblistClause<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    InstLiblist(
        Box<(
            InstClause<'a>,
            LiblistClause<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    InstUse(
        Box<(
            InstClause<'a>,
            UseClause<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    CellLiblist(
        Box<(
            CellClause<'a>,
            LiblistClause<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    CellUse(
        Box<(
            CellClause<'a>,
            UseClause<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefaultClause<'a>(
    pub Metadata<'a>, // default
);

#[derive(Clone, Debug, PartialEq)]
pub struct InstClause<'a>(
    pub Metadata<'a>, // instance
    pub InstName<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InstName<'a>(
    pub TopmoduleIdentifier<'a>,
    pub  Vec<(
        Metadata<'a>, // .
        InstanceIdentifier<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CellClause<'a>(
    pub Metadata<'a>, // cell
    pub  Option<(
        LibraryIdentifier<'a>,
        Metadata<'a>, // .
    )>,
    pub CellIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct LiblistClause<'a>(
    pub Metadata<'a>, // liblist
    pub Vec<LibraryIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum UseClause<'a> {
    Cell(
        Box<(
            Metadata<'a>, // use
            Option<(
                LibraryIdentifier<'a>,
                Metadata<'a>, // .
            )>,
            CellIdentifier<'a>,
            Option<(
                Metadata<'a>, // :
                Metadata<'a>, // config
            )>,
        )>,
    ),
    Parameter(
        Box<(
            Metadata<'a>, // use
            NamedParameterAssignment<'a>,
            Vec<(
                Metadata<'a>, // ,
                NamedParameterAssignment<'a>,
            )>,
            Option<(
                Metadata<'a>, // :
                Metadata<'a>, // config
            )>,
        )>,
    ),
    CellParameter(
        Box<(
            Metadata<'a>, // use
            Option<(
                LibraryIdentifier<'a>,
                Metadata<'a>, // .
            )>,
            CellIdentifier<'a>,
            NamedParameterAssignment<'a>,
            Vec<(
                Metadata<'a>, // ,
                NamedParameterAssignment<'a>,
            )>,
            Option<(
                Metadata<'a>, // :
                Metadata<'a>, // config
            )>,
        )>,
    ),
}
