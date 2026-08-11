// =======================================================================
// system_timing_check_command_arguments.rs
// =======================================================================
// Parsing for 1800-2023 A.7.5.2

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;

pub fn controlled_reference_event_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ControlledReferenceEvent<'s>, VerboseError<'s>> {
    controlled_timing_check_event_parser
        .map(|a| ControlledReferenceEvent(a))
        .parse_next(input)
}

pub fn data_event_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataEvent<'s>, VerboseError<'s>> {
    timing_check_event_parser
        .map(|a| DataEvent(a))
        .parse_next(input)
}

pub fn delayed_data_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DelayedData<'s>, VerboseError<'s>> {
    (
        terminal_identifier_parser,
        opt_note((
            token(Token::Bracket),
            constant_mintypmax_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| match b {
            Some((b, c, d)) => DelayedData::Slice(Box::new((a, b, c, d))),
            None => DelayedData::Identifier(Box::new(a)),
        })
        .parse_next(input)
}

pub fn delayed_reference_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DelayedReference<'s>, VerboseError<'s>> {
    (
        terminal_identifier_parser,
        opt_note((
            token(Token::Bracket),
            constant_mintypmax_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| match b {
            Some((b, c, d)) => DelayedReference::Slice(Box::new((a, b, c, d))),
            None => DelayedReference::Identifier(Box::new(a)),
        })
        .parse_next(input)
}

pub fn end_edge_offset_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EndEdgeOffset<'s>, VerboseError<'s>> {
    mintypmax_expression_parser
        .map(|a| EndEdgeOffset(a))
        .parse_next(input)
}

pub fn event_based_flag_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EventBasedFlag<'s>, VerboseError<'s>> {
    constant_expression_parser
        .map(|a| EventBasedFlag(a))
        .parse_next(input)
}

pub fn notifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Notifier<'s>, VerboseError<'s>> {
    variable_identifier_parser
        .map(|a| Notifier(a))
        .parse_next(input)
}

pub fn reference_event_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ReferenceEvent<'s>, VerboseError<'s>> {
    timing_check_event_parser
        .map(|a| ReferenceEvent(a))
        .parse_next(input)
}

pub fn remain_active_flag_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RemainActiveFlag<'s>, VerboseError<'s>> {
    constant_mintypmax_expression_parser
        .map(|a| RemainActiveFlag(a))
        .parse_next(input)
}

pub fn timecheck_condition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimecheckCondition<'s>, VerboseError<'s>> {
    mintypmax_expression_parser
        .map(|a| TimecheckCondition(a))
        .parse_next(input)
}

pub fn timestamp_condition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimestampCondition<'s>, VerboseError<'s>> {
    mintypmax_expression_parser
        .map(|a| TimestampCondition(a))
        .parse_next(input)
}

pub fn start_edge_offset_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StartEdgeOffset<'s>, VerboseError<'s>> {
    mintypmax_expression_parser
        .map(|a| StartEdgeOffset(a))
        .parse_next(input)
}

pub fn threshold_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Threshold<'s>, VerboseError<'s>> {
    constant_expression_parser
        .map(|a| Threshold(a))
        .parse_next(input)
}

pub fn timing_check_limit_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimingCheckLimit<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| TimingCheckLimit(a))
        .parse_next(input)
}
