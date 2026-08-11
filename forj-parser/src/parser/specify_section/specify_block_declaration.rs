// =======================================================================
// specify_block_declaration.rs
// =======================================================================
// Parsing for 1800-2023 A.7.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn specify_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecifyBlock<'s>, VerboseError<'s>> {
    (
        token(Token::Specify),
        repeat_note(specify_item_parser),
        token(Token::Endspecify),
    )
        .map(|(a, b, c)| SpecifyBlock(a, b, c))
        .parse_next(input)
}

pub fn specify_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecifyItem<'s>, VerboseError<'s>> {
    alt((
        specparam_declaration_parser
            .map(|a| SpecifyItem::Specparam(Box::new(a))),
        pulsestyle_declaration_parser
            .map(|a| SpecifyItem::Pulsestyle(Box::new(a))),
        showcancelled_declaration_parser
            .map(|a| SpecifyItem::Showcancelled(Box::new(a))),
        path_declaration_parser.map(|a| SpecifyItem::Path(Box::new(a))),
        system_timing_check_parser
            .map(|a| SpecifyItem::SystemTiming(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn pulsestyle_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PulsestyleDeclaration<'s>, VerboseError<'s>> {
    alt((
        (
            token(Token::PulsestyleOnevent),
            list_of_path_outputs_parser,
            token(Token::SColon),
        )
            .map(|(a, b, c)| {
                PulsestyleDeclaration::Onevent(Box::new((a, b, c)))
            }),
        (
            token(Token::PulsestyleOndetect),
            list_of_path_outputs_parser,
            token(Token::SColon),
        )
            .map(|(a, b, c)| {
                PulsestyleDeclaration::Ondetect(Box::new((a, b, c)))
            }),
    ))
    .parse_next(input)
}

pub fn showcancelled_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ShowcancelledDeclaration<'s>, VerboseError<'s>> {
    alt((
        (
            token(Token::Showcancelled),
            list_of_path_outputs_parser,
            token(Token::SColon),
        )
            .map(|(a, b, c)| {
                ShowcancelledDeclaration::Show(Box::new((a, b, c)))
            }),
        (
            token(Token::Noshowcancelled),
            list_of_path_outputs_parser,
            token(Token::SColon),
        )
            .map(|(a, b, c)| {
                ShowcancelledDeclaration::Noshow(Box::new((a, b, c)))
            }),
    ))
    .parse_next(input)
}
