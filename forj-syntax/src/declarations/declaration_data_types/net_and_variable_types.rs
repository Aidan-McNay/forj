// =======================================================================
// net_and_variable_types.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.2.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CastingType<'a> {
    SimpleType(Box<SimpleType<'a>>),
    ConstantPrimary(Box<ConstantPrimary<'a>>),
    Signing(Box<Signing<'a>>),
    String(Box<Metadata<'a>>),
    Const(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassOrPackageScope<'a> {
    Class(Box<ClassScope<'a>>),
    Package(Box<PackageScope<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataType<'a> {
    Vector(
        Box<(
            IntegerVectorType<'a>,
            Option<Signing<'a>>,
            Vec<PackedDimension<'a>>,
        )>,
    ),
    Atom(Box<(IntegerAtomType<'a>, Option<Signing<'a>>)>),
    NonInteger(Box<NonIntegerType<'a>>),
    StructUnion(
        Box<(
            StructUnion<'a>,
            Option<(
                Metadata<'a>, // packed
                Option<Signing<'a>>,
            )>,
            Metadata<'a>, // {
            StructUnionMember<'a>,
            Vec<StructUnionMember<'a>>,
            Metadata<'a>, // }
            Vec<PackedDimension<'a>>,
        )>,
    ),
    Enum(
        Box<(
            Metadata<'a>, // enum
            Option<EnumBaseType<'a>>,
            Metadata<'a>, // {
            EnumNameDeclaration<'a>,
            Vec<(
                Metadata<'a>, // ,
                EnumNameDeclaration<'a>,
            )>,
            Metadata<'a>, // }
            Vec<PackedDimension<'a>>,
        )>,
    ),
    String(Box<Metadata<'a>>),
    Chandle(Box<Metadata<'a>>),
    Virtual(
        Box<(
            Metadata<'a>,         // virtual
            Option<Metadata<'a>>, // interface
            InterfaceIdentifier<'a>,
            Option<ParameterValueAssignment<'a>>,
            Option<(
                Metadata<'a>, // .
                ModportIdentifier<'a>,
            )>,
        )>,
    ),
    Type(
        Box<(
            Option<ClassOrPackageScope<'a>>,
            TypeIdentifier<'a>,
            Vec<PackedDimension<'a>>,
        )>,
    ),
    ClassType(Box<ClassType<'a>>),
    Event(Box<Metadata<'a>>),
    PsCovergroup(Box<PsCovergroupIdentifier<'a>>),
    TypeRef(Box<TypeReference<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataTypeOrImplicit<'a> {
    DataType(DataType<'a>),
    ImplicitDataType(ImplicitDataType<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplicitDataType<'a>(
    pub Option<Signing<'a>>,
    pub Vec<PackedDimension<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum EnumBaseType<'a> {
    Atom(Box<(IntegerAtomType<'a>, Option<Signing<'a>>)>),
    Vector(
        Box<(
            IntegerVectorType<'a>,
            Option<Signing<'a>>,
            Option<PackedDimension<'a>>,
        )>,
    ),
    Type(Box<(TypeIdentifier<'a>, Option<PackedDimension<'a>>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumNameDeclaration<'a>(
    pub EnumIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // [
        IntegralNumber<'a>,
        Option<(
            Metadata<'a>, // :
            IntegralNumber<'a>,
        )>,
        Metadata<'a>, // ]
    )>,
    pub  Option<(
        Metadata<'a>, // =
        ConstantExpression<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassScope<'a>(
    pub ClassType<'a>,
    pub Metadata<'a>, // ::
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassType<'a>(
    pub PsClassIdentifier<'a>,
    pub Option<ParameterValueAssignment<'a>>,
    pub  Vec<(
        Metadata<'a>, // ::
        ClassIdentifier<'a>,
        Option<ParameterValueAssignment<'a>>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceClassType<'a>(
    pub PsClassIdentifier<'a>,
    pub Option<ParameterValueAssignment<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerType<'a> {
    Vector(Box<IntegerVectorType<'a>>),
    Atom(Box<IntegerAtomType<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerAtomType<'a> {
    Byte(Metadata<'a>),
    Shortint(Metadata<'a>),
    Int(Metadata<'a>),
    Longint(Metadata<'a>),
    Integer(Metadata<'a>),
    Time(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerVectorType<'a> {
    Bit(Metadata<'a>),
    Logic(Metadata<'a>),
    Reg(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NonIntegerType<'a> {
    Shortreal(Metadata<'a>),
    Real(Metadata<'a>),
    Realtime(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NetType<'a> {
    Supply0(Metadata<'a>),
    Supply1(Metadata<'a>),
    Tri(Metadata<'a>),
    Triand(Metadata<'a>),
    Trior(Metadata<'a>),
    Trireg(Metadata<'a>),
    Tri0(Metadata<'a>),
    Tri1(Metadata<'a>),
    Uwire(Metadata<'a>),
    Wire(Metadata<'a>),
    Wand(Metadata<'a>),
    Wor(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NetPortType<'a> {
    Implicit(Box<(Option<NetType<'a>>, DataTypeOrImplicit<'a>)>),
    Nettype(Box<NettypeIdentifier<'a>>),
    Interconnect(
        Box<(
            Metadata<'a>, // interconnect
            ImplicitDataType<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariablePortType<'a>(pub VarDataType<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum VarDataType<'a> {
    Data(Box<DataType<'a>>),
    Var(
        Box<(
            Metadata<'a>, // var
            DataTypeOrImplicit<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Signing<'a> {
    Signed(Metadata<'a>),
    Unsigned(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SimpleType<'a> {
    Integer(Box<IntegerType<'a>>),
    NonInteger(Box<NonIntegerType<'a>>),
    PsType(Box<PsTypeIdentifier<'a>>),
    PsParameter(Box<PsParameterIdentifier<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SoftOrTagged<'a> {
    Soft(Metadata<'a>),
    Tagged(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructUnion<'a> {
    Struct(Metadata<'a>),
    Union(Metadata<'a>, Option<SoftOrTagged<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructUnionMember<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<RandomQualifier<'a>>,
    pub DataTypeOrVoid<'a>,
    pub ListOfVariableDeclAssignments<'a>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum DataTypeOrVoid<'a> {
    DataType(Box<DataType<'a>>),
    Void(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeReference<'a> {
    Expression(
        Box<(
            Metadata<'a>, // type
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
        )>,
    ),
    DataTypeOrIncompleteClassScopedType(
        Box<(
            Metadata<'a>, // type
            Metadata<'a>, // (
            DataTypeOrIncompleteClassScopedType<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataTypeOrIncompleteClassScopedType<'a> {
    Data(Box<DataType<'a>>),
    IncompleteClassScoped(Box<IncompleteClassScopedType<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IncompleteClassScopedType<'a> {
    Base(
        Box<(
            TypeIdentifier<'a>,
            Metadata<'a>, // ::
            TypeIdentifierOrClassType<'a>,
        )>,
    ),
    Recursive(
        Box<(
            IncompleteClassScopedType<'a>,
            Metadata<'a>, // ::
            TypeIdentifierOrClassType<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeIdentifierOrClassType<'a> {
    Type(Box<TypeIdentifier<'a>>),
    Class(Box<ClassType<'a>>),
}
