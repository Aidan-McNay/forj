// =======================================================================
// program_instantiation.rs
// =======================================================================
// Parsing for 1800-2023 A.4.1.3

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;

pub fn program_instantiation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramInstantiation<'s>, VerboseError<'s>> {
    (
        program_identifier_parser,
        opt_note(parameter_value_assignment_parser),
        hierarchical_instance_parser,
        repeat_note((token(Token::Comma), hierarchical_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| ProgramInstantiation(a, b, c, d, e))
        .parse_next(input)
}
