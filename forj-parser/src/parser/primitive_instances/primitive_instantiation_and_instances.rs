// =======================================================================
// primitive_strengths.rs
// =======================================================================
// Parsing for 1800-2023 A.3.1

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, peek, terminated};

pub fn gate_instantiation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GateInstantiation<'s>, VerboseError<'s>> {
    let _cmos_parser = (
        cmos_switchtype_parser,
        opt_note(delay3_parser),
        cmos_switch_instance_parser,
        repeat_note((token(Token::Comma), cmos_switch_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            GateInstantiation::Cmos(Box::new((a, b, c, d, e)))
        });
    let _mos_parser = (
        mos_switchtype_parser,
        opt_note(delay3_parser),
        mos_switch_instance_parser,
        repeat_note((token(Token::Comma), mos_switch_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            GateInstantiation::Mos(Box::new((a, b, c, d, e)))
        });
    let _enable_parser = (
        enable_gatetype_parser,
        opt_note(drive_strength_parser),
        opt_note(delay3_parser),
        enable_gate_instance_parser,
        repeat_note((token(Token::Comma), enable_gate_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            GateInstantiation::Enable(Box::new((a, b, c, d, e, f)))
        });
    let _n_input_parser = (
        n_input_gatetype_parser,
        opt_note(drive_strength_parser),
        opt_note(delay2_parser),
        n_input_gate_instance_parser,
        repeat_note((token(Token::Comma), n_input_gate_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            GateInstantiation::NInput(Box::new((a, b, c, d, e, f)))
        });
    let _n_output_parser = (
        n_output_gatetype_parser,
        opt_note(drive_strength_parser),
        opt_note(delay2_parser),
        n_output_gate_instance_parser,
        repeat_note((token(Token::Comma), n_output_gate_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| {
            GateInstantiation::NOutput(Box::new((a, b, c, d, e, f)))
        });
    let _pass_en_parser = (
        pass_en_switchtype_parser,
        opt_note(delay2_parser),
        pass_enable_switch_instance_parser,
        repeat_note((token(Token::Comma), pass_enable_switch_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            GateInstantiation::PassEn(Box::new((a, b, c, d, e)))
        });
    let _pass_parser = (
        pass_switchtype_parser,
        pass_switch_instance_parser,
        repeat_note((token(Token::Comma), pass_switch_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| GateInstantiation::Pass(Box::new((a, b, c, d))));
    let _pulldown_parser = (
        token(Token::Pulldown),
        opt_note(pulldown_strength_parser),
        pull_gate_instance_parser,
        repeat_note((token(Token::Comma), pull_gate_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            GateInstantiation::Pulldown(Box::new((a, b, c, d, e)))
        });
    let _pullup_parser = (
        token(Token::Pullup),
        opt_note(pullup_strength_parser),
        pull_gate_instance_parser,
        repeat_note((token(Token::Comma), pull_gate_instance_parser)),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            GateInstantiation::Pullup(Box::new((a, b, c, d, e)))
        });
    alt((
        _cmos_parser,
        _mos_parser,
        _enable_parser,
        _n_input_parser,
        _n_output_parser,
        _pass_en_parser,
        _pass_parser,
        _pulldown_parser,
        _pullup_parser,
    ))
    .parse_next(input)
}

pub fn cmos_switch_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CmosSwitchInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        output_terminal_parser,
        token(Token::Comma),
        input_terminal_parser,
        token(Token::Comma),
        ncontrol_terminal_parser,
        token(Token::Comma),
        pcontrol_terminal_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            CmosSwitchInstance(a, b, c, d, e, f, g, h, i, j)
        })
        .parse_next(input)
}

pub fn enable_gate_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EnableGateInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        output_terminal_parser,
        token(Token::Comma),
        input_terminal_parser,
        token(Token::Comma),
        enable_terminal_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            EnableGateInstance(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn mos_switch_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MosSwitchInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        output_terminal_parser,
        token(Token::Comma),
        input_terminal_parser,
        token(Token::Comma),
        enable_terminal_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            MosSwitchInstance(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn n_input_gate_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NInputGateInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        output_terminal_parser,
        token(Token::Comma),
        input_terminal_parser,
        repeat_note((token(Token::Comma), input_terminal_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g)| NInputGateInstance(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn n_output_gate_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NOutputGateInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        output_terminal_parser,
        repeat_note((
            terminated(
                token(Token::Comma),
                peek((output_terminal_parser, token(Token::Comma))),
            ),
            output_terminal_parser,
        )),
        token(Token::Comma),
        input_terminal_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g)| NOutputGateInstance(a, b, c, d, e, f, g))
        .parse_next(input)
}

pub fn pass_switch_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PassSwitchInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        inout_terminal_parser,
        token(Token::Comma),
        inout_terminal_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| PassSwitchInstance(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn pass_enable_switch_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PassEnableSwitchInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        inout_terminal_parser,
        token(Token::Comma),
        inout_terminal_parser,
        token(Token::Comma),
        enable_terminal_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            PassEnableSwitchInstance(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn pull_gate_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PullGateInstance<'s>, VerboseError<'s>> {
    (
        opt_note(name_of_instance_parser),
        token(Token::Paren),
        output_terminal_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| PullGateInstance(a, b, c, d))
        .parse_next(input)
}
