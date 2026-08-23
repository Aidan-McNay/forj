// =======================================================================
// block_item_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.8

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

enum BlockItemDeclarationBody<'a> {
    Data(DataDeclaration<'a>),
    LocalParameter((LocalParameterDeclaration<'a>, Metadata<'a>)),
    Parameter((ParameterDeclaration<'a>, Metadata<'a>)),
    Let(LetDeclaration<'a>),
}

pub fn block_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BlockItemDeclaration<'s>, VerboseError<'s>> {
    let _body_parser = alt((
        data_declaration_parser.map(|a| BlockItemDeclarationBody::Data(a)),
        (local_parameter_declaration_parser, token(Token::SColon))
            .map(|(a, b)| BlockItemDeclarationBody::LocalParameter((a, b))),
        (parameter_declaration_parser, token(Token::SColon))
            .map(|(a, b)| BlockItemDeclarationBody::Parameter((a, b))),
        let_declaration_parser.map(|a| BlockItemDeclarationBody::Let(a)),
    ));
    (attribute_instance_vec_parser, _body_parser)
        .map(|(a, b)| match b {
            BlockItemDeclarationBody::Data(c) => {
                BlockItemDeclaration::Data(Box::new((a, c)))
            }
            BlockItemDeclarationBody::LocalParameter((c, d)) => {
                BlockItemDeclaration::LocalParameter(Box::new((a, c, d)))
            }
            BlockItemDeclarationBody::Parameter((c, d)) => {
                BlockItemDeclaration::Parameter(Box::new((a, c, d)))
            }
            BlockItemDeclarationBody::Let(c) => {
                BlockItemDeclaration::Let(Box::new((a, c)))
            }
        })
        .parse_next(input)
}
