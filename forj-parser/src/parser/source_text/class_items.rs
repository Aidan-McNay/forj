// =======================================================================
// class_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.9

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

enum ClassItemBody<'a> {
    Property(ClassProperty<'a>),
    Method(ClassMethod<'a>),
    Constraint(ClassConstraint<'a>),
    Declaration(ClassDeclaration<'a>),
    InterfaceDeclaration(InterfaceClassDeclaration<'a>),
    Covergroup(CovergroupDeclaration<'a>),
}

pub fn class_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassItem<'s>, VerboseError<'s>> {
    let _class_item_body_parser = alt((
        class_property_parser.map(|a| ClassItemBody::Property(a)),
        class_method_parser.map(|a| ClassItemBody::Method(a)),
        class_constraint_parser.map(|a| ClassItemBody::Constraint(a)),
        class_declaration_parser.map(|a| ClassItemBody::Declaration(a)),
        interface_class_declaration_parser
            .map(|a| ClassItemBody::InterfaceDeclaration(a)),
        covergroup_declaration_parser.map(|a| ClassItemBody::Covergroup(a)),
    ));
    let _basic_class_item_parser = (
        attribute_instance_vec_parser,
        _class_item_body_parser,
    )
        .map(|(a, b)| match b {
            ClassItemBody::Property(c) => ClassItem::Property(Box::new((a, c))),
            ClassItemBody::Method(c) => ClassItem::Method(Box::new((a, c))),
            ClassItemBody::Constraint(c) => {
                ClassItem::Constraint(Box::new((a, c)))
            }
            ClassItemBody::Declaration(c) => {
                ClassItem::Declaration(Box::new((a, c)))
            }
            ClassItemBody::InterfaceDeclaration(c) => {
                ClassItem::InterfaceDeclaration(Box::new((a, c)))
            }
            ClassItemBody::Covergroup(c) => {
                ClassItem::Covergroup(Box::new((a, c)))
            }
        });
    let _local_parameter_parser =
        (local_parameter_declaration_parser, token(Token::SColon))
            .map(|(a, b)| ClassItem::LocalParameter(Box::new((a, b))));
    let _parameter_parser =
        (parameter_declaration_parser, token(Token::SColon))
            .map(|(a, b)| ClassItem::Parameter(Box::new((a, b))));
    let _null_parser =
        token(Token::SColon).map(|a| ClassItem::Null(Box::new(a)));
    alt((
        _basic_class_item_parser,
        _local_parameter_parser,
        _parameter_parser,
        _null_parser,
    ))
    .parse_next(input)
}

pub fn class_property_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassProperty<'s>, VerboseError<'s>> {
    let _data_parser = (
        repeat_note(property_qualifier_parser),
        data_declaration_parser,
    )
        .map(|(a, b)| ClassProperty::Data(Box::new((a, b))));
    let _const_parser = (
        token(Token::Const),
        repeat_note(class_item_qualifier_parser),
        data_type_parser,
        const_identifier_parser,
        opt_note((token(Token::Eq), constant_expression_or_class_new_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            ClassProperty::Const(Box::new((a, b, c, d, e, f)))
        });
    alt((_const_parser, _data_parser)).parse_next(input)
}

pub fn constant_expression_or_class_new_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantExpressionOrClassNew<'s>, VerboseError<'s>> {
    alt((
        class_new_parser.map(|a| ConstantExpressionOrClassNew::ClassNew(a)),
        constant_expression_parser
            .map(|a| ConstantExpressionOrClassNew::ConstantExpression(a)),
    ))
    .parse_next(input)
}

pub fn class_method_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassMethod<'s>, VerboseError<'s>> {
    let _task_parser = (
        repeat_note(method_qualifier_parser),
        task_declaration_parser,
    )
        .map(|(a, b)| ClassMethod::Task(Box::new((a, b))));
    let _function_parser = (
        repeat_note(method_qualifier_parser),
        function_declaration_parser,
    )
        .map(|(a, b)| ClassMethod::Function(Box::new((a, b))));
    let _pure_virtual_method_parser = (
        token(Token::Pure),
        token(Token::Virtual),
        repeat_note(class_item_qualifier_parser),
        method_prototype_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            ClassMethod::PureVirtualMethod(Box::new((a, b, c, d, e)))
        });
    let _extern_method_parser = (
        token(Token::Extern),
        repeat_note(method_qualifier_parser),
        method_prototype_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| ClassMethod::ExternMethod(Box::new((a, b, c, d))));
    let _constructor_declaration_parser = (
        repeat_note(method_qualifier_parser),
        class_constructor_declaration_parser,
    )
        .map(|(a, b)| ClassMethod::ConstructorDeclaration(Box::new((a, b))));
    let _constructor_prototype_parser = (
        token(Token::Extern),
        repeat_note(method_qualifier_parser),
        class_constructor_prototype_parser,
    )
        .map(|(a, b, c)| {
            ClassMethod::ConstructorPrototype(Box::new((a, b, c)))
        });
    alt((
        _task_parser,
        _pure_virtual_method_parser,
        _extern_method_parser,
        _constructor_declaration_parser,
        _constructor_prototype_parser,
        _function_parser,
    ))
    .parse_next(input)
}

pub fn list_of_arguments_or_default_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfArgumentsOrDefault<'s>, VerboseError<'s>> {
    alt((
        list_of_arguments_parser
            .map(|a| ListOfArgumentsOrDefault::ListOfArguments(Box::new(a))),
        token(Token::Default)
            .map(|a| ListOfArgumentsOrDefault::Default(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn class_constructor_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassConstructorDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Function),
        opt_note(class_scope_parser),
        token(Token::New),
        opt_note((
            token(Token::Paren),
            opt_note(class_constructor_arg_list_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
        repeat_note(block_item_declaration_parser),
        opt_note((
            token(Token::Super),
            token(Token::Period),
            token(Token::New),
            opt_note((
                token(Token::Paren),
                opt_note(list_of_arguments_or_default_parser),
                token(Token::EParen),
            )),
            token(Token::SColon),
        )),
        repeat_note(function_statement_or_null_parser),
        token(Token::Endfunction),
        opt_note((token(Token::Colon), token(Token::New))),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            ClassConstructorDeclaration(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn class_constructor_prototype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassConstructorPrototype<'s>, VerboseError<'s>> {
    (
        token(Token::Function),
        token(Token::New),
        opt_note((
            token(Token::Paren),
            opt_note(class_constructor_arg_list_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| ClassConstructorPrototype(a, b, c, d))
        .parse_next(input)
}

pub fn class_constructor_arg_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassConstructorArgList<'s>, VerboseError<'s>> {
    (
        class_constructor_arg_parser,
        repeat_note((token(Token::Comma), class_constructor_arg_parser)),
    )
        .map(|(a, b)| ClassConstructorArgList(a, b))
        .parse_next(input)
}

pub fn class_constructor_arg_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassConstructorArg<'s>, VerboseError<'s>> {
    alt((
        tf_port_item_parser.map(|a| ClassConstructorArg::TfPort(Box::new(a))),
        token(Token::Default)
            .map(|a| ClassConstructorArg::Default(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn interface_class_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceClassItem<'s>, VerboseError<'s>> {
    let _type_parser =
        type_declaration_parser.map(|a| InterfaceClassItem::Type(Box::new(a)));
    let _interface_class_parser =
        (attribute_instance_vec_parser, interface_class_method_parser)
            .map(|(a, b)| InterfaceClassItem::InterfaceClass(Box::new((a, b))));
    let _local_parameter_parser =
        (local_parameter_declaration_parser, token(Token::SColon))
            .map(|(a, b)| InterfaceClassItem::LocalParameter(Box::new((a, b))));
    let _parameter_parser =
        (parameter_declaration_parser, token(Token::SColon))
            .map(|(a, b)| InterfaceClassItem::Parameter(Box::new((a, b))));
    let _null_parser =
        token(Token::SColon).map(|a| InterfaceClassItem::Null(Box::new(a)));
    alt((
        _type_parser,
        _interface_class_parser,
        _local_parameter_parser,
        _parameter_parser,
        _null_parser,
    ))
    .parse_next(input)
}

pub fn interface_class_method_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceClassMethod<'s>, VerboseError<'s>> {
    (
        token(Token::Pure),
        token(Token::Virtual),
        method_prototype_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| InterfaceClassMethod(a, b, c, d))
        .parse_next(input)
}

pub fn class_constraint_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassConstraint<'s>, VerboseError<'s>> {
    alt((
        constraint_prototype_parser
            .map(|a| ClassConstraint::Prototype(Box::new(a))),
        constraint_declaration_parser
            .map(|a| ClassConstraint::Declaration(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn class_item_qualifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassItemQualifier<'s>, VerboseError<'s>> {
    alt((
        token(Token::Static).map(|a| ClassItemQualifier::Static(a)),
        token(Token::Protected).map(|a| ClassItemQualifier::Protected(a)),
        token(Token::Local).map(|a| ClassItemQualifier::Local(a)),
    ))
    .parse_next(input)
}

pub fn property_qualifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PropertyQualifier<'s>, VerboseError<'s>> {
    alt((
        random_qualifier_parser.map(|a| PropertyQualifier::Random(Box::new(a))),
        class_item_qualifier_parser
            .map(|a| PropertyQualifier::ClassItem(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn random_qualifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RandomQualifier<'s>, VerboseError<'s>> {
    alt((
        token(Token::Rand).map(|a| RandomQualifier::Rand(a)),
        token(Token::Randc).map(|a| RandomQualifier::Randc(a)),
    ))
    .parse_next(input)
}

pub fn method_qualifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MethodQualifier<'s>, VerboseError<'s>> {
    let _pure_virtual_parser =
        (opt_note(token(Token::Pure)), token(Token::Virtual))
            .map(|(a, b)| MethodQualifier::PureVirtual(Box::new((a, b))));
    let _class_item_parser = class_item_qualifier_parser
        .map(|a| MethodQualifier::ClassItem(Box::new(a)));
    alt((_pure_virtual_parser, _class_item_parser)).parse_next(input)
}

pub fn method_prototype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MethodPrototype<'s>, VerboseError<'s>> {
    alt((
        task_prototype_parser.map(|a| MethodPrototype::Task(Box::new(a))),
        function_prototype_parser
            .map(|a| MethodPrototype::Function(Box::new(a))),
    ))
    .parse_next(input)
}
