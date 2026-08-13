// =======================================================================
// patterns.rs
// =======================================================================
// Parsing for 1800-2023 A.6.7.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn pattern_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Pattern<'s>, VerboseError<'s>> {
    let _parentheses_parser =
        (token(Token::Paren), pattern_parser, token(Token::EParen))
            .map(|(a, b, c)| Pattern::Parentheses(Box::new((a, b, c))));
    let _variable_identifier_parser =
        (token(Token::Period), variable_identifier_parser)
            .map(|(a, b)| Pattern::VariableIdentifier(Box::new((a, b))));
    let _wildcard_parser = (token(Token::Period), token(Token::Star))
        .map(|(a, b)| Pattern::Wildcard(Box::new((a, b))));
    let _constant_expression_parser = constant_expression_parser
        .map(|a| Pattern::ConstantExpression(Box::new(a)));
    let _tagged_member_parser = (
        token(Token::Tagged),
        member_identifier_parser,
        opt_note(pattern_parser),
    )
        .map(|(a, b, c)| Pattern::TaggedMember(Box::new((a, b, c))));
    let _multi_pattern_parser = (
        token(Token::Apost),
        token(Token::Brace),
        pattern_parser,
        repeat_note((token(Token::Apost), pattern_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| {
            Pattern::MultiPattern(Box::new((a, b, c, d, e)))
        });
    let _multi_identifier_pattern_parser = (
        token(Token::Apost),
        token(Token::Brace),
        member_identifier_parser,
        token(Token::Colon),
        pattern_parser,
        repeat_note((
            token(Token::Apost),
            member_identifier_parser,
            token(Token::Colon),
            pattern_parser,
        )),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e, f, g)| {
            Pattern::MultiIdentifierPattern(Box::new((a, b, c, d, e, f, g)))
        });
    alt((
        _parentheses_parser,
        _variable_identifier_parser,
        _wildcard_parser,
        _constant_expression_parser,
        _tagged_member_parser,
        _multi_pattern_parser,
        _multi_identifier_pattern_parser,
    ))
    .parse_next(input)
}

pub fn assignment_pattern_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPattern<'s>, VerboseError<'s>> {
    let _empty_parser = (
        token(Token::Apost),
        token(Token::Brace),
        token(Token::EBrace),
    )
        .map(|(a, b, c)| AssignmentPattern::Empty(Box::new((a, b, c))));
    let _expression_parser = (
        token(Token::Apost),
        token(Token::Brace),
        expression_parser,
        repeat_note((token(Token::Comma), expression_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| {
            AssignmentPattern::Expression(Box::new((a, b, c, d, e)))
        });
    let _structure_parser = (
        token(Token::Apost),
        token(Token::Brace),
        structure_pattern_key_parser,
        token(Token::Colon),
        expression_parser,
        repeat_note((
            token(Token::Comma),
            structure_pattern_key_parser,
            token(Token::Colon),
            expression_parser,
        )),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e, f, g)| {
            AssignmentPattern::Structure(Box::new((a, b, c, d, e, f, g)))
        });
    let _array_parser = (
        token(Token::Apost),
        token(Token::Brace),
        array_pattern_key_parser,
        token(Token::Colon),
        expression_parser,
        repeat_note((
            token(Token::Comma),
            array_pattern_key_parser,
            token(Token::Colon),
            expression_parser,
        )),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e, f, g)| {
            AssignmentPattern::Array(Box::new((a, b, c, d, e, f, g)))
        });
    let _constant_parser = (
        token(Token::Apost),
        token(Token::Brace),
        constant_expression_parser,
        token(Token::Brace),
        expression_parser,
        repeat_note((token(Token::Comma), expression_parser)),
        token(Token::EBrace),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            AssignmentPattern::Constant(Box::new((a, b, c, d, e, f, g, h)))
        });
    alt((
        _empty_parser,
        _expression_parser,
        _structure_parser,
        _array_parser,
        _constant_parser,
    ))
    .parse_next(input)
}

pub fn structure_pattern_key_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StructurePatternKey<'s>, VerboseError<'s>> {
    alt((
        member_identifier_parser
            .map(|a| StructurePatternKey::MemberIdentifier(Box::new(a))),
        assignment_pattern_key_parser
            .map(|a| StructurePatternKey::AssignmentPatternKey(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn array_pattern_key_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ArrayPatternKey<'s>, VerboseError<'s>> {
    alt((
        constant_expression_parser
            .map(|a| ArrayPatternKey::ConstantExpression(Box::new(a))),
        assignment_pattern_key_parser
            .map(|a| ArrayPatternKey::AssignmentPatternKey(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn assignment_pattern_key_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternKey<'s>, VerboseError<'s>> {
    alt((
        token(Token::Default)
            .map(|a| AssignmentPatternKey::Default(Box::new(a))),
        simple_type_parser
            .map(|a| AssignmentPatternKey::SimpleType(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn assignment_pattern_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternExpression<'s>, VerboseError<'s>> {
    (
        opt_note(assignment_pattern_expression_type_parser),
        assignment_pattern_parser,
    )
        .map(|(a, b)| AssignmentPatternExpression(a, b))
        .parse_next(input)
}

pub fn assignment_pattern_expression_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternExpressionType<'s>, VerboseError<'s>> {
    alt((
        ps_type_identifier_parser
            .map(|a| AssignmentPatternExpressionType::PsType(Box::new(a))),
        ps_parameter_identifier_parser
            .map(|a| AssignmentPatternExpressionType::PsParameter(Box::new(a))),
        integer_atom_type_parser
            .map(|a| AssignmentPatternExpressionType::Integer(Box::new(a))),
        type_reference_parser
            .map(|a| AssignmentPatternExpressionType::Type(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn constant_assignment_pattern_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantAssignmentPatternExpression<'s>, VerboseError<'s>> {
    assignment_pattern_expression_parser
        .map(|a| ConstantAssignmentPatternExpression(a))
        .parse_next(input)
}

pub fn assignment_pattern_net_lvalue_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternNetLvalue<'s>, VerboseError<'s>> {
    (
        token(Token::Apost),
        token(Token::Brace),
        net_lvalue_parser,
        repeat_note((token(Token::Comma), net_lvalue_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| AssignmentPatternNetLvalue(a, b, c, d, e))
        .parse_next(input)
}

// Inlined for variable_lvalue_parser
pub fn assignment_pattern_variable_lvalue_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternVariableLvalue<'s>, VerboseError<'s>> {
    (
        token(Token::Apost),
        token(Token::Brace),
        variable_lvalue_parser,
        repeat_note((token(Token::Comma), variable_lvalue_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| AssignmentPatternVariableLvalue(a, b, c, d, e))
        .parse_next(input)
}
