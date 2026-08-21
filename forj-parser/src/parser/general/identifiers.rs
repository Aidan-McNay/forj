// =======================================================================
// identifiers.rs
// =======================================================================
// Parsing for 1800-2023 A.9.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};
use winnow::token::any;

#[allow(dead_code)] // Never used
pub fn array_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ArrayIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ArrayIdentifier(a))
        .parse_next(input)
}

pub fn block_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BlockIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| BlockIdentifier(a))
        .parse_next(input)
}

pub fn bin_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| BinIdentifier(a))
        .parse_next(input)
}

pub fn c_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CIdentifier<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::SimpleIdentifier(text) => {
                if !(text.contains("$")) {
                    Some(CIdentifier(text, Metadata::new(s.1.clone(), vec![])))
                } else {
                    None
                }
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|(c_identifier, non_trivia)| {
            CIdentifier(
                c_identifier.0,
                replace_non_trivia(c_identifier.1, non_trivia),
            )
        })
        .context("a C identifier")
        .parse_next(input)
}

#[cfg(test)]
mod c_identifier {
    use super::*;

    #[test]
    fn basic() {
        check_parser!(
            "basic_identifier0",
            c_identifier_parser,
            CIdentifier("basic_identifier0", test_metadata())
        )
    }

    #[test]
    #[should_panic]
    fn no_dollar() {
        check_parser!(
            "bad_identifier$",
            c_identifier_parser,
            CIdentifier("bad_identifier$", test_metadata())
        )
    }

    #[test]
    #[should_panic]
    fn no_number_start() {
        check_parser!(
            "1bad_identifier",
            c_identifier_parser,
            CIdentifier("1bad_identifier", test_metadata())
        )
    }
}

pub fn cell_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CellIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| CellIdentifier(a))
        .parse_next(input)
}

pub fn checker_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| CheckerIdentifier(a))
        .parse_next(input)
}

pub fn class_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ClassIdentifier(a))
        .parse_next(input)
}

pub fn class_variable_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassVariableIdentifier<'s>, VerboseError<'s>> {
    variable_identifier_parser
        .map(|a| ClassVariableIdentifier(a))
        .parse_next(input)
}

pub fn clocking_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockingIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ClockingIdentifier(a))
        .parse_next(input)
}

pub fn config_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConfigIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ConfigIdentifier(a))
        .parse_next(input)
}

pub fn const_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ConstIdentifier(a))
        .parse_next(input)
}

pub fn constraint_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ConstraintIdentifier(a))
        .parse_next(input)
}

pub fn covergroup_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| CovergroupIdentifier(a))
        .parse_next(input)
}

#[allow(dead_code)] // Never used
pub fn covergroup_variable_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupVariableIdentifier<'s>, VerboseError<'s>> {
    variable_identifier_parser
        .map(|a| CovergroupVariableIdentifier(a))
        .parse_next(input)
}

pub fn cover_point_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CoverPointIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| CoverPointIdentifier(a))
        .parse_next(input)
}

pub fn cross_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CrossIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| CrossIdentifier(a))
        .parse_next(input)
}

pub fn dynamic_array_variable_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DynamicArrayVariableIdentifier<'s>, VerboseError<'s>> {
    variable_identifier_parser
        .map(|a| DynamicArrayVariableIdentifier(a))
        .parse_next(input)
}

pub fn enum_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EnumIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| EnumIdentifier(a))
        .parse_next(input)
}

#[allow(dead_code)] // Never used
pub fn formal_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FormalIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| FormalIdentifier(a))
        .parse_next(input)
}

pub fn formal_port_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FormalPortIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| FormalPortIdentifier(a))
        .parse_next(input)
}

pub fn function_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| FunctionIdentifier(a))
        .parse_next(input)
}

pub fn generate_block_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenerateBlockIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| GenerateBlockIdentifier(a))
        .parse_next(input)
}

pub fn genvar_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GenvarIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| GenvarIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_array_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalArrayIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalArrayIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_block_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalBlockIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalBlockIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_event_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalEventIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalEventIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalIdentifier<'s>, VerboseError<'s>> {
    let identifiers_parser = repeat_note(terminated(
        (
            identifier_parser,
            constant_bit_select_parser,
            token(Token::Period),
        ),
        peek(identifier_parser),
    ));
    let _hierarchical_parser = (
        opt_note((token(Token::DollarRoot), token(Token::Period))),
        identifiers_parser,
        identifier_parser,
    )
        .map(|(a, b, c)| HierarchicalIdentifier::Hierarchical((a, b, c)));
    alt((
        _hierarchical_parser,
        token(Token::DollarRoot).map(|a| HierarchicalIdentifier::Root(a)),
        token(Token::DollarUnit).map(|a| HierarchicalIdentifier::Unit(a)),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod hierarchical_identifier {
    use super::*;

    #[test]
    fn basic() {
        let mut storage = vec![];
        check_parser!(
            "test",
            hierarchical_identifier_parser,
            HierarchicalIdentifier::Hierarchical((
                None,
                vec![],
                apply_parser!("test", identifier_parser, &mut storage)
            ))
        )
    }

    #[test]
    fn hierarchy() {
        let mut storage_parent = vec![];
        let mut storage_child1 = vec![];
        let mut storage_child2 = vec![];
        check_parser!(
            "parent.child1.child2",
            hierarchical_identifier_parser,
            HierarchicalIdentifier::Hierarchical((
                None,
                vec![
                    (
                        apply_parser!(
                            "parent",
                            identifier_parser,
                            &mut storage_parent
                        ),
                        ConstantBitSelect(vec![]),
                        test_metadata()
                    ),
                    (
                        apply_parser!(
                            "child1",
                            identifier_parser,
                            &mut storage_child1
                        ),
                        ConstantBitSelect(vec![]),
                        test_metadata()
                    )
                ],
                apply_parser!("child2", identifier_parser, &mut storage_child2)
            ))
        )
    }

    #[test]
    fn select_hierarchy() {
        let mut storage_food = vec![];
        let mut storage_food_idx = vec![];
        let mut storage_fruit = vec![];
        let mut storage_fruit_idx = vec![];
        let mut storage_apple = vec![];
        check_parser!(
            "food[0].fruit[3].apple",
            hierarchical_identifier_parser,
            HierarchicalIdentifier::Hierarchical((
                None,
                vec![
                    (
                        apply_parser!(
                            "food",
                            identifier_parser,
                            &mut storage_food
                        ),
                        ConstantBitSelect(vec![(
                            test_metadata(),
                            apply_parser!(
                                "0",
                                constant_expression_parser,
                                &mut storage_food_idx
                            ),
                            test_metadata()
                        )]),
                        test_metadata()
                    ),
                    (
                        apply_parser!(
                            "fruit",
                            identifier_parser,
                            &mut storage_fruit
                        ),
                        ConstantBitSelect(vec![(
                            test_metadata(),
                            apply_parser!(
                                "3",
                                constant_expression_parser,
                                &mut storage_fruit_idx
                            ),
                            test_metadata()
                        )]),
                        test_metadata()
                    )
                ],
                apply_parser!("apple", identifier_parser, &mut storage_apple)
            ))
        )
    }

    #[test]
    fn root_hierarchy() {
        let mut storage_trunk = vec![];
        let mut storage_branch = vec![];
        check_parser!(
            "$root.trunk.branch",
            hierarchical_identifier_parser,
            HierarchicalIdentifier::Hierarchical((
                Some((test_metadata(), test_metadata())),
                vec![(
                    apply_parser!(
                        "trunk",
                        identifier_parser,
                        &mut storage_trunk
                    ),
                    ConstantBitSelect(vec![]),
                    test_metadata()
                )],
                apply_parser!("branch", identifier_parser, &mut storage_branch)
            ))
        )
    }

    #[test]
    fn root() {
        check_parser!(
            "$root",
            hierarchical_identifier_parser,
            HierarchicalIdentifier::Root(test_metadata())
        )
    }

    #[test]
    fn unit() {
        check_parser!(
            "$unit",
            hierarchical_identifier_parser,
            HierarchicalIdentifier::Unit(test_metadata())
        )
    }
}

pub fn hierarchical_net_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalNetIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalNetIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_parameter_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalParameterIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalParameterIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_property_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalPropertyIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalPropertyIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_sequence_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalSequenceIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalSequenceIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_task_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalTaskIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalTaskIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_tf_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalTfIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalTfIdentifier(a))
        .parse_next(input)
}

pub fn hierarchical_variable_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<HierarchicalVariableIdentifier<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| HierarchicalVariableIdentifier(a))
        .parse_next(input)
}

pub fn identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Identifier<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::SimpleIdentifier(text) => {
                Some(Identifier::SimpleIdentifier((
                    text,
                    Metadata::new(s.1.clone(), vec![]),
                )))
            }
            Token::EscapedIdentifier(text) => {
                Some(Identifier::EscapedIdentifier((
                    text,
                    Metadata::new(s.1.clone(), vec![]),
                )))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|(identifier, non_trivia)| match identifier {
            Identifier::SimpleIdentifier((text, metadata)) => {
                Identifier::SimpleIdentifier((
                    text,
                    replace_non_trivia(metadata, non_trivia),
                ))
            }
            Identifier::EscapedIdentifier((text, metadata)) => {
                Identifier::EscapedIdentifier((
                    text,
                    replace_non_trivia(metadata, non_trivia),
                ))
            }
        })
        .context("an identifier")
        .parse_next(input)
}

#[cfg(test)]
mod identifier {
    use super::*;
    #[test]
    fn simple_identifier() {
        check_parser!(
            "test",
            identifier_parser,
            Identifier::SimpleIdentifier(("test", test_metadata()))
        )
    }

    #[test]
    fn escaped_identifier() {
        check_parser!(
            "\\test/identifier+#$",
            identifier_parser,
            Identifier::EscapedIdentifier((
                "\\test/identifier+#$",
                test_metadata()
            ))
        )
    }

    #[test]
    #[should_panic]
    fn no_number_start() {
        check_parser!(
            "1bad_test",
            identifier_parser,
            Identifier::SimpleIdentifier(("1bad_test", test_metadata()))
        )
    }

    #[test]
    #[should_panic]
    fn no_dollar_start() {
        check_parser!(
            "$bad_test",
            identifier_parser,
            Identifier::SimpleIdentifier(("$bad_test", test_metadata()))
        )
    }
}

pub fn index_variable_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IndexVariableIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| IndexVariableIdentifier(a))
        .parse_next(input)
}

pub fn interface_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| InterfaceIdentifier(a))
        .parse_next(input)
}

pub fn interface_port_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfacePortIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| InterfacePortIdentifier(a))
        .parse_next(input)
}

pub fn inout_port_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InoutPortIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| InoutPortIdentifier(a))
        .parse_next(input)
}

pub fn input_port_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InputPortIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| InputPortIdentifier(a))
        .parse_next(input)
}

pub fn instance_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InstanceIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| InstanceIdentifier(a))
        .parse_next(input)
}

pub fn library_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LibraryIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| LibraryIdentifier(a))
        .parse_next(input)
}

pub fn member_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MemberIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| MemberIdentifier(a))
        .parse_next(input)
}

pub fn method_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MethodIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| MethodIdentifier(a))
        .parse_next(input)
}

pub fn modport_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModportIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ModportIdentifier(a))
        .parse_next(input)
}

pub fn module_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ModuleIdentifier(a))
        .parse_next(input)
}

pub fn net_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| NetIdentifier(a))
        .parse_next(input)
}

pub fn nettype_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NettypeIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| NettypeIdentifier(a))
        .parse_next(input)
}

pub fn output_port_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OutputPortIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| OutputPortIdentifier(a))
        .parse_next(input)
}

pub fn package_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| PackageIdentifier(a))
        .parse_next(input)
}

pub fn package_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageScope<'s>, VerboseError<'s>> {
    let _identifier_parser =
        (package_identifier_parser, token(Token::ColonColon))
            .map(|(a, b)| PackageScope::Identifier(Box::new((a, b))));
    let _unit_parser = (token(Token::DollarUnit), token(Token::ColonColon))
        .map(|(a, b)| PackageScope::Unit(Box::new((a, b))));
    alt((_identifier_parser, _unit_parser)).parse_next(input)
}

#[cfg(test)]
mod package_scope {
    use super::*;

    #[test]
    fn unit() {
        check_parser!(
            "$unit::",
            package_scope_parser,
            PackageScope::Unit(Box::new((test_metadata(), test_metadata())))
        );
    }

    #[test]
    fn identifier() {
        let mut storage = vec![];
        check_parser!(
            "my_package::",
            package_scope_parser,
            PackageScope::Identifier(Box::new((
                apply_parser!(
                    "my_package",
                    package_identifier_parser,
                    &mut storage
                ),
                test_metadata()
            )))
        )
    }
}

pub fn parameter_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParameterIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ParameterIdentifier(a))
        .parse_next(input)
}

pub fn port_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PortIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| PortIdentifier(a))
        .parse_next(input)
}

pub fn program_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| ProgramIdentifier(a))
        .parse_next(input)
}

pub fn property_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| PropertyIdentifier(a))
        .parse_next(input)
}

pub fn ps_class_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsClassIdentifier<'s>, VerboseError<'s>> {
    (opt_note(package_scope_parser), class_identifier_parser)
        .map(|(a, b)| PsClassIdentifier(a, b))
        .parse_next(input)
}

pub fn ps_covergroup_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsCovergroupIdentifier<'s>, VerboseError<'s>> {
    (opt_note(package_scope_parser), covergroup_identifier_parser)
        .map(|(a, b)| PsCovergroupIdentifier(a, b))
        .parse_next(input)
}

pub fn ps_checker_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsCheckerIdentifier<'s>, VerboseError<'s>> {
    (opt_note(package_scope_parser), checker_identifier_parser)
        .map(|(a, b)| PsCheckerIdentifier(a, b))
        .parse_next(input)
}

pub fn ps_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsIdentifier<'s>, VerboseError<'s>> {
    (opt_note(package_scope_parser), identifier_parser)
        .map(|(a, b)| PsIdentifier(a, b))
        .parse_next(input)
}

pub fn ps_or_hierarchical_array_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsOrHierarchicalArrayIdentifier<'s>, VerboseError<'s>> {
    (
        opt_note(implicit_class_handle_or_class_scope_or_package_scope_parser),
        hierarchical_array_identifier_parser,
    )
        .map(|(a, b)| PsOrHierarchicalArrayIdentifier(a, b))
        .parse_next(input)
}

pub fn ps_or_hierarchical_net_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsOrHierarchicalNetIdentifier<'s>, VerboseError<'s>> {
    let opt_scope = opt_note(package_scope_parser).parse_next(input)?;
    match opt_scope {
        None => hierarchical_net_identifier_parser
            .map(|a| PsOrHierarchicalNetIdentifier::Hierarchical(a))
            .parse_next(input),
        Some(scope) => {
            let identifier = net_identifier_parser.parse_next(input)?;
            Ok(PsOrHierarchicalNetIdentifier::PackageScope(
                Some(scope),
                identifier,
            ))
        }
    }
}

pub fn ps_or_hierarchical_property_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsOrHierarchicalPropertyIdentifier<'s>, VerboseError<'s>> {
    let opt_scope = opt_note(package_scope_parser).parse_next(input)?;
    match opt_scope {
        None => hierarchical_property_identifier_parser
            .map(|a| PsOrHierarchicalPropertyIdentifier::Hierarchical(a))
            .parse_next(input),
        Some(scope) => {
            let identifier = property_identifier_parser.parse_next(input)?;
            Ok(PsOrHierarchicalPropertyIdentifier::PackageScope(
                Some(scope),
                identifier,
            ))
        }
    }
}

pub fn ps_or_hierarchical_sequence_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsOrHierarchicalSequenceIdentifier<'s>, VerboseError<'s>> {
    let opt_scope = opt_note(package_scope_parser).parse_next(input)?;
    match opt_scope {
        None => hierarchical_sequence_identifier_parser
            .map(|a| PsOrHierarchicalSequenceIdentifier::Hierarchical(a))
            .parse_next(input),
        Some(scope) => {
            let identifier = sequence_identifier_parser.parse_next(input)?;
            Ok(PsOrHierarchicalSequenceIdentifier::PackageScope(
                Some(scope),
                identifier,
            ))
        }
    }
}

pub fn ps_or_hierarchical_tf_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsOrHierarchicalTfIdentifier<'s>, VerboseError<'s>> {
    let opt_scope =
        opt_note(implicit_class_handle_or_class_scope_or_package_scope_parser)
            .parse_next(input)?;
    match opt_scope {
        None => hierarchical_tf_identifier_parser
            .map(|a| PsOrHierarchicalTfIdentifier::Hierarchical(a))
            .parse_next(input),
        Some(scope) => {
            let identifier = tf_identifier_parser.parse_next(input)?;
            Ok(PsOrHierarchicalTfIdentifier::Scoped(
                Some(scope),
                identifier,
            ))
        }
    }
}

pub fn package_or_class_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageOrClassScope<'s>, VerboseError<'s>> {
    alt((
        class_scope_parser.map(|a| PackageOrClassScope::ClassScope(a)),
        package_scope_parser.map(|a| PackageOrClassScope::PackageScope(a)),
    ))
    .parse_next(input)
}

pub fn ps_parameter_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsParameterIdentifier<'s>, VerboseError<'s>> {
    let _scoped_parser = (
        opt_note(package_or_class_scope_parser),
        parameter_identifier_parser,
    )
        .map(|(a, b)| PsParameterIdentifier::Scoped(a, b));
    let _generated_parser = (
        repeat_note(terminated(
            (
                generate_block_identifier_parser,
                opt_note((
                    token(Token::Bracket),
                    constant_expression_parser,
                    token(Token::EBracket),
                )),
                token(Token::Period),
            ),
            peek(parameter_identifier_parser),
        )),
        parameter_identifier_parser,
    )
        .map(|(a, b)| PsParameterIdentifier::Generated(a, b));
    alt((_scoped_parser, _generated_parser)).parse_next(input)
}

pub fn ps_type_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PsTypeIdentifier<'s>, VerboseError<'s>> {
    let _scope_parser = alt((
        (token(Token::Local), token(Token::ColonColon))
            .map(|(a, b)| PsTypeIdentifierScope::LocalScope(a, b)),
        package_scope_parser.map(|a| PsTypeIdentifierScope::PackageScope(a)),
        class_scope_parser.map(|a| PsTypeIdentifierScope::ClassScope(a)),
    ));
    (opt_note(_scope_parser), type_identifier_parser)
        .map(|(a, b)| PsTypeIdentifier(a, b))
        .parse_next(input)
}

pub fn rs_production_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RsProductionIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| RsProductionIdentifier(a))
        .parse_next(input)
}

pub fn sequence_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| SequenceIdentifier(a))
        .parse_next(input)
}

pub fn signal_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SignalIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| SignalIdentifier(a))
        .parse_next(input)
}

pub fn specparam_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecparamIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| SpecparamIdentifier(a))
        .parse_next(input)
}

// Handle system tf identifiers with explicit tokens separately
const SYSTEM_TF_FATAL: &str = "$fatal";
const SYSTEM_TF_ERROR: &str = "$error";
const SYSTEM_TF_WARNING: &str = "$warning";
const SYSTEM_TF_INFO: &str = "$info";

pub fn system_tf_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SystemTfIdentifier<'s>, VerboseError<'s>> {
    let general_system_tf_identifier_parser = (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::SystemTfIdentifier(text) => Some(SystemTfIdentifier(
                text,
                Metadata::new(s.1.clone(), vec![]),
            )),
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|(identifier, non_trivia)| {
            SystemTfIdentifier(
                identifier.0,
                replace_non_trivia(identifier.1, non_trivia),
            )
        });
    alt((
        token(Token::DollarFatal)
            .map(|a| SystemTfIdentifier(SYSTEM_TF_FATAL, a)),
        token(Token::DollarError)
            .map(|a| SystemTfIdentifier(SYSTEM_TF_ERROR, a)),
        token(Token::DollarWarning)
            .map(|a| SystemTfIdentifier(SYSTEM_TF_WARNING, a)),
        token(Token::DollarInfo).map(|a| SystemTfIdentifier(SYSTEM_TF_INFO, a)),
        general_system_tf_identifier_parser,
    ))
    .context("a system tf identifier")
    .parse_next(input)
}

pub fn task_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TaskIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| TaskIdentifier(a))
        .parse_next(input)
}

pub fn tf_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TfIdentifier<'s>, VerboseError<'s>> {
    identifier_parser.map(|a| TfIdentifier(a)).parse_next(input)
}

pub fn terminal_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TerminalIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| TerminalIdentifier(a))
        .parse_next(input)
}

pub fn topmodule_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TopmoduleIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| TopmoduleIdentifier(a))
        .parse_next(input)
}

pub fn type_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TypeIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| TypeIdentifier(a))
        .parse_next(input)
}

pub fn udp_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| UdpIdentifier(a))
        .parse_next(input)
}

pub fn variable_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariableIdentifier<'s>, VerboseError<'s>> {
    identifier_parser
        .map(|a| VariableIdentifier(a))
        .parse_next(input)
}
