// =======================================================================
// specify_block_terminals.rs
// =======================================================================
// Parsing for 1800-2023 A.7.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn list_of_path_inputs_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfPathInputs<'s>, VerboseError<'s>> {
    (
        specify_input_terminal_descriptor_parser,
        repeat_note((
            token(Token::Comma),
            specify_input_terminal_descriptor_parser,
        )),
    )
        .map(|(a, b)| ListOfPathInputs(a, b))
        .parse_next(input)
}

pub fn list_of_path_outputs_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfPathOutputs<'s>, VerboseError<'s>> {
    (
        specify_output_terminal_descriptor_parser,
        repeat_note((
            token(Token::Comma),
            specify_output_terminal_descriptor_parser,
        )),
    )
        .map(|(a, b)| ListOfPathOutputs(a, b))
        .parse_next(input)
}

pub fn specify_input_terminal_descriptor_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecifyInputTerminalDescriptor<'s>, VerboseError<'s>> {
    (
        input_identifier_parser,
        opt_note((
            token(Token::Bracket),
            constant_range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| SpecifyInputTerminalDescriptor(a, b))
        .parse_next(input)
}

pub fn specify_output_terminal_descriptor_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecifyOutputTerminalDescriptor<'s>, VerboseError<'s>> {
    (
        output_identifier_parser,
        opt_note((
            token(Token::Bracket),
            constant_range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| SpecifyOutputTerminalDescriptor(a, b))
        .parse_next(input)
}

pub fn input_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InputIdentifier<'s>, VerboseError<'s>> {
    alt((
        input_port_identifier_parser
            .map(|a| InputIdentifier::Input(Box::new(a))),
        inout_port_identifier_parser
            .map(|a| InputIdentifier::Inout(Box::new(a))),
        (
            interface_identifier_parser,
            token(Token::Period),
            port_identifier_parser,
        )
            .map(|(a, b, c)| InputIdentifier::Interface(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn output_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OutputIdentifier<'s>, VerboseError<'s>> {
    alt((
        output_port_identifier_parser
            .map(|a| OutputIdentifier::Output(Box::new(a))),
        inout_port_identifier_parser
            .map(|a| OutputIdentifier::Inout(Box::new(a))),
        (
            interface_identifier_parser,
            token(Token::Period),
            port_identifier_parser,
        )
            .map(|(a, b, c)| OutputIdentifier::Interface(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}
