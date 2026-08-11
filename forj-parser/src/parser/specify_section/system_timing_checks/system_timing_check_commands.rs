// =======================================================================
// system_timing_check_commands.rs
// =======================================================================
// Parsing for 1800-2023 A.7.5.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn system_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SystemTimingCheck<'s>, VerboseError<'s>> {
    alt((
        dollar_setup_timing_check_parser
            .map(|a| SystemTimingCheck::Setup(Box::new(a))),
        dollar_hold_timing_check_parser
            .map(|a| SystemTimingCheck::Hold(Box::new(a))),
        dollar_setuphold_timing_check_parser
            .map(|a| SystemTimingCheck::Setuphold(Box::new(a))),
        dollar_recovery_timing_check_parser
            .map(|a| SystemTimingCheck::Recovery(Box::new(a))),
        dollar_removal_timing_check_parser
            .map(|a| SystemTimingCheck::Removal(Box::new(a))),
        dollar_recrem_timing_check_parser
            .map(|a| SystemTimingCheck::Recrem(Box::new(a))),
        dollar_skew_timing_check_parser
            .map(|a| SystemTimingCheck::Skew(Box::new(a))),
        dollar_timeskew_timing_check_parser
            .map(|a| SystemTimingCheck::Timeskew(Box::new(a))),
        dollar_fullskew_timing_check_parser
            .map(|a| SystemTimingCheck::Fullskew(Box::new(a))),
        dollar_period_timing_check_parser
            .map(|a| SystemTimingCheck::Period(Box::new(a))),
        dollar_width_timing_check_parser
            .map(|a| SystemTimingCheck::Width(Box::new(a))),
        dollar_nochange_timing_check_parser
            .map(|a| SystemTimingCheck::Nochange(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn dollar_setup_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarSetupTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarSetup),
        token(Token::Paren),
        data_event_parser,
        token(Token::Comma),
        reference_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((token(Token::Comma), opt_note(notifier_parser))),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            DollarSetupTimingCheck(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn dollar_hold_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarHoldTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarHold),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((token(Token::Comma), opt_note(notifier_parser))),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            DollarHoldTimingCheck(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn dollar_setuphold_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarSetupholdTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarSetuphold),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((
            token(Token::Comma),
            opt_note(notifier_parser),
            opt_note((
                token(Token::Comma),
                opt_note(timestamp_condition_parser),
                opt_note((
                    token(Token::Comma),
                    opt_note(timecheck_condition_parser),
                    opt_note((
                        token(Token::Comma),
                        opt_note(delayed_reference_parser),
                        opt_note((
                            token(Token::Comma),
                            opt_note(delayed_data_parser),
                        )),
                    )),
                )),
            )),
        )),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            DollarSetupholdTimingCheck(a, b, c, d, e, f, g, h, i, j, k, l)
        })
        .parse_next(input)
}

pub fn dollar_recovery_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarRecoveryTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarRecovery),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((token(Token::Comma), opt_note(notifier_parser))),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            DollarRecoveryTimingCheck(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn dollar_removal_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarRemovalTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarRemoval),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((token(Token::Comma), opt_note(notifier_parser))),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            DollarRemovalTimingCheck(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn dollar_recrem_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarRecremTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarRecrem),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((
            token(Token::Comma),
            opt_note(notifier_parser),
            opt_note((
                token(Token::Comma),
                opt_note(timestamp_condition_parser),
                opt_note((
                    token(Token::Comma),
                    opt_note(timecheck_condition_parser),
                    opt_note((
                        token(Token::Comma),
                        opt_note(delayed_reference_parser),
                        opt_note((
                            token(Token::Comma),
                            opt_note(delayed_data_parser),
                        )),
                    )),
                )),
            )),
        )),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            DollarRecremTimingCheck(a, b, c, d, e, f, g, h, i, j, k, l)
        })
        .parse_next(input)
}

pub fn dollar_skew_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarSkewTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarSkew),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((token(Token::Comma), opt_note(notifier_parser))),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            DollarSkewTimingCheck(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn dollar_timeskew_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarTimeskewTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarTimeskew),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((
            token(Token::Comma),
            opt_note(notifier_parser),
            opt_note((
                token(Token::Comma),
                opt_note(event_based_flag_parser),
                opt_note((
                    token(Token::Comma),
                    opt_note(remain_active_flag_parser),
                )),
            )),
        )),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            DollarTimeskewTimingCheck(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn dollar_fullskew_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarFullskewTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarFullskew),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((
            token(Token::Comma),
            opt_note(notifier_parser),
            opt_note((
                token(Token::Comma),
                opt_note(event_based_flag_parser),
                opt_note((
                    token(Token::Comma),
                    opt_note(remain_active_flag_parser),
                )),
            )),
        )),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            DollarFullskewTimingCheck(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn dollar_period_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarPeriodTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarPeriod),
        token(Token::Paren),
        controlled_reference_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        opt_note((token(Token::Comma), opt_note(notifier_parser))),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            DollarPeriodTimingCheck(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn dollar_width_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarWidthTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarWidth),
        token(Token::Paren),
        controlled_reference_event_parser,
        token(Token::Comma),
        timing_check_limit_parser,
        token(Token::Comma),
        threshold_parser,
        opt_note((token(Token::Comma), opt_note(notifier_parser))),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            DollarWidthTimingCheck(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn dollar_nochange_timing_check_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DollarNochangeTimingCheck<'s>, VerboseError<'s>> {
    (
        token(Token::DollarNochange),
        token(Token::Paren),
        reference_event_parser,
        token(Token::Comma),
        data_event_parser,
        token(Token::Comma),
        start_edge_offset_parser,
        token(Token::Comma),
        end_edge_offset_parser,
        opt_note((token(Token::Comma), opt_note(notifier_parser))),
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            DollarNochangeTimingCheck(a, b, c, d, e, f, g, h, i, j, k, l)
        })
        .parse_next(input)
}
