// =======================================================================
// attributes.rs
// =======================================================================
// Parsing for 1800-2023 A.9.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;

pub fn attribute_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AttributeInstance<'s>, VerboseError<'s>> {
    (
        token(Token::Paren),
        token(Token::Star),
        attr_spec_parser,
        repeat_note((token(Token::Comma), attr_spec_parser)),
        token(Token::Star),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| AttributeInstance(a, b, c, d, e, f))
        .parse_next(input)
}

pub(crate) fn attribute_instance_vec_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Vec<AttributeInstance<'s>>, VerboseError<'s>> {
    repeat_note(attribute_instance_parser).parse_next(input)
}

pub fn attr_spec_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AttrSpec<'s>, VerboseError<'s>> {
    (
        attr_name_parser,
        opt_note((token(Token::Eq), constant_expression_parser)),
    )
        .map(|(a, b)| AttrSpec(a, b))
        .parse_next(input)
}

pub fn attr_name_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AttrName<'s>, VerboseError<'s>> {
    identifier_parser.map(|a| AttrName(a)).parse_next(input)
}
