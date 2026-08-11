// =======================================================================
// type_declarations.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.1.3
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum DataDeclaration<'a> {
    Variable(
        Box<(
            Option<Metadata<'a>>, // const
            Option<Metadata<'a>>, // var
            Option<Lifetime<'a>>,
            DataTypeOrImplicit<'a>,
            ListOfVariableDeclAssignments<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Type(Box<TypeDeclaration<'a>>),
    PackageImport(Box<PackageImportDeclaration<'a>>),
    Nettype(Box<NettypeDeclaration<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackageImportDeclaration<'a>(
    pub Metadata<'a>, // import
    pub PackageImportItem<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        PackageImportItem<'a>,
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum PackageExportDeclaration<'a> {
    Wildcard(
        Box<(
            Metadata<'a>, // export
            Metadata<'a>, // *
            Metadata<'a>, // ::
            Metadata<'a>, // *
            Metadata<'a>, // ;
        )>,
    ),
    Import(
        Box<(
            Metadata<'a>, // export
            PackageImportItem<'a>,
            Vec<(
                Metadata<'a>, // ,
                PackageImportItem<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PackageImportItem<'a> {
    Identifier(
        Box<(
            PackageIdentifier<'a>,
            Metadata<'a>, // ::
            Identifier<'a>,
        )>,
    ),
    Wildcard(
        Box<(
            PackageIdentifier<'a>,
            Metadata<'a>, // ::
            Metadata<'a>, // *
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenvarDeclaration<'a>(
    pub Metadata<'a>, // genvar
    pub ListOfGenvarIdentifiers<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum DriveOrChargeStrength<'a> {
    DriveStrength(DriveStrength<'a>),
    ChargeStrength(ChargeStrength<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum VectoredOrScalared<'a> {
    Vectored(Metadata<'a>),
    Scalared(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NetDeclaration<'a> {
    NetType(
        Box<(
            NetType<'a>,
            Option<DriveOrChargeStrength<'a>>,
            Option<VectoredOrScalared<'a>>,
            DataTypeOrImplicit<'a>,
            Option<Delay3<'a>>,
            ListOfNetDeclAssignments<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    NettypeIdentifier(
        Box<(
            NettypeIdentifier<'a>,
            Option<DelayControl<'a>>,
            ListOfNetDeclAssignments<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    Interconnect(
        Box<(
            Metadata<'a>, // interconnect
            ImplicitDataType<'a>,
            Option<(
                Metadata<'a>, // #
                DelayValue<'a>,
            )>,
            NetIdentifier<'a>,
            Vec<UnpackedDimension<'a>>,
            Option<(
                Metadata<'a>, // ,
                NetIdentifier<'a>,
                Vec<UnpackedDimension<'a>>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeDeclaration<'a> {
    DataTypeOrIncompleteClassScoped(
        Box<(
            Metadata<'a>, // typedef
            DataTypeOrIncompleteClassScopedType<'a>,
            TypeIdentifier<'a>,
            Vec<VariableDimension<'a>>,
            Metadata<'a>, // ;
        )>,
    ),
    InterfacePort(
        Box<(
            Metadata<'a>, // typedef
            InterfacePortIdentifier<'a>,
            ConstantBitSelect<'a>,
            Metadata<'a>, // .
            TypeIdentifier<'a>,
            TypeIdentifier<'a>,
            Metadata<'a>, // ;
        )>,
    ),
    ForwardType(
        Box<(
            Metadata<'a>, // typedef
            Option<ForwardType<'a>>,
            TypeIdentifier<'a>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForwardType<'a> {
    Enum(Metadata<'a>),
    Struct(Metadata<'a>),
    Union(Metadata<'a>),
    Class(Metadata<'a>),
    InterfaceClass(Metadata<'a>, Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NettypeDeclaration<'a> {
    WithScope(
        Box<(
            Metadata<'a>, // nettype
            DataType<'a>,
            NettypeIdentifier<'a>,
            Option<(
                Metadata<'a>, // with
                Option<PackageOrClassScope<'a>>,
                TfIdentifier<'a>,
            )>,
            Metadata<'a>, // ;
        )>,
    ),
    Scoped(
        Box<(
            Metadata<'a>, // nettype
            Option<PackageOrClassScope<'a>>,
            NettypeIdentifier<'a>,
            NettypeIdentifier<'a>,
            Metadata<'a>, //  ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Lifetime<'a> {
    Static(Metadata<'a>),
    Automatic(Metadata<'a>),
}
