// =======================================================================
// declaration_assignments.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.2.4
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefparamAssignment<'a>(
    pub HierarchicalParameterIdentifier<'a>,
    pub Metadata<'a>, // =
    pub ConstantMintypmaxExpression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct NetDeclAssignment<'a>(
    pub NetIdentifier<'a>,
    pub Vec<UnpackedDimension<'a>>,
    pub  Option<(
        Metadata<'a>, // =
        Expression<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ParamAssignment<'a>(
    pub ParameterIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub  Option<(
        Metadata<'a>, // =
        ConstantParamExpression<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum SpecparamAssignment<'a> {
    Base(
        Box<(
            SpecparamIdentifier<'a>,
            Metadata<'a>, // =
            ConstantMintypmaxExpression<'a>,
        )>,
    ),
    Pulse(Box<PulseControlSpecparam<'a>>),
}

// Note that the syntax for path-specific pulses isn't parsed fully.
// This is due to the complexity on the parsing side, as the current
// parser cannot (with current limitations of logos) distinguish
// between the long pulse path and a long identifier, since the
// tokens are concatenated
#[derive(Clone, Debug, PartialEq)]
pub enum PulseControlSpecparam<'a> {
    NonpathSpecific(
        Box<(
            Metadata<'a>, // PATHPULSE$
            Metadata<'a>, // =
            Metadata<'a>, // (
            RejectLimitValue<'a>,
            Option<(
                Metadata<'a>, // ,
                ErrorLimitValue<'a>,
            )>,
            Metadata<'a>, // )
        )>,
    ),
    PathSpecific(
        Box<(
            Identifier<'a>, // PATHPULSE$<input>$<output> - see above
            Metadata<'a>,   // =
            Metadata<'a>,   // (
            RejectLimitValue<'a>,
            Option<(
                Metadata<'a>, // ,
                ErrorLimitValue<'a>,
            )>,
            Metadata<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorLimitValue<'a>(pub LimitValue<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct RejectLimitValue<'a>(pub LimitValue<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct LimitValue<'a>(pub ConstantMintypmaxExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAssignment<'a>(
    pub TypeIdentifier<'a>,
    pub Option<(Metadata<'a>, DataTypeOrIncompleteClassScopedType<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum VariableDeclAssignment<'a> {
    Variable(
        Box<(
            VariableIdentifier<'a>,
            Vec<VariableDimension<'a>>,
            Option<(
                Metadata<'a>, // =
                Expression<'a>,
            )>,
        )>,
    ),
    DynamicVariable(
        Box<(
            DynamicArrayVariableIdentifier<'a>,
            UnsizedDimension<'a>,
            Vec<VariableDimension<'a>>,
            Option<(
                Metadata<'a>, // =
                DynamicArrayNew<'a>,
            )>,
        )>,
    ),
    ClassVariable(
        Box<(
            ClassVariableIdentifier<'a>,
            Option<(
                Metadata<'a>, // =
                ClassNew<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassNew<'a> {
    Args(
        Box<(
            Option<ClassScope<'a>>,
            Metadata<'a>, // new
            Option<(
                Metadata<'a>, // (
                ListOfArguments<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Expression(
        Box<(
            Metadata<'a>, // new
            Expression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicArrayNew<'a>(
    pub Metadata<'a>, // new
    pub Metadata<'a>, // [
    pub Expression<'a>,
    pub Metadata<'a>, // ]
    pub  Option<(
        Metadata<'a>, // (
        Expression<'a>,
        Metadata<'a>, // )
    )>,
);
