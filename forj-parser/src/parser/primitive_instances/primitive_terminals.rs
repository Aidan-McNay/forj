// =======================================================================
// primitive_terminals.rs
// =======================================================================
// Parsing for 1800-2023 A.3.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;

pub fn enable_terminal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EnableTerminal<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| EnableTerminal(a))
        .parse_next(input)
}

pub fn inout_terminal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InoutTerminal<'s>, VerboseError<'s>> {
    net_lvalue_parser
        .map(|a| InoutTerminal(a))
        .parse_next(input)
}

pub fn input_terminal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InputTerminal<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| InputTerminal(a))
        .parse_next(input)
}

pub fn ncontrol_terminal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NcontrolTerminal<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| NcontrolTerminal(a))
        .parse_next(input)
}

pub fn output_terminal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OutputTerminal<'s>, VerboseError<'s>> {
    net_lvalue_parser
        .map(|a| OutputTerminal(a))
        .parse_next(input)
}

pub fn pcontrol_terminal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PcontrolTerminal<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| PcontrolTerminal(a))
        .parse_next(input)
}
