// =======================================================================
// primitive_gate_and_switch_types.rs
// =======================================================================
// Parsing for 1800-2023 A.3.4

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn cmos_switchtype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CmosSwitchtype<'s>, VerboseError<'s>> {
    alt((
        token(Token::Cmos).map(|a| CmosSwitchtype::Cmos(a)),
        token(Token::Rcmos).map(|a| CmosSwitchtype::Rcmos(a)),
    ))
    .parse_next(input)
}

pub fn enable_gatetype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EnableGatetype<'s>, VerboseError<'s>> {
    alt((
        token(Token::Bufif0).map(|a| EnableGatetype::Bufif0(a)),
        token(Token::Bufif1).map(|a| EnableGatetype::Bufif1(a)),
        token(Token::Notif0).map(|a| EnableGatetype::Notif0(a)),
        token(Token::Notif1).map(|a| EnableGatetype::Notif1(a)),
    ))
    .parse_next(input)
}

pub fn mos_switchtype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MosSwitchtype<'s>, VerboseError<'s>> {
    alt((
        token(Token::Nmos).map(|a| MosSwitchtype::Nmos(a)),
        token(Token::Pmos).map(|a| MosSwitchtype::Pmos(a)),
        token(Token::Rnmos).map(|a| MosSwitchtype::Rnmos(a)),
        token(Token::Rpmos).map(|a| MosSwitchtype::Rpmos(a)),
    ))
    .parse_next(input)
}

pub fn n_input_gatetype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NInputGatetype<'s>, VerboseError<'s>> {
    alt((
        token(Token::And).map(|a| NInputGatetype::And(a)),
        token(Token::Nand).map(|a| NInputGatetype::Nand(a)),
        token(Token::Or).map(|a| NInputGatetype::Or(a)),
        token(Token::Nor).map(|a| NInputGatetype::Nor(a)),
        token(Token::Xor).map(|a| NInputGatetype::Xor(a)),
        token(Token::Xnor).map(|a| NInputGatetype::Xnor(a)),
    ))
    .parse_next(input)
}

pub fn n_output_gatetype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NOutputGatetype<'s>, VerboseError<'s>> {
    alt((
        token(Token::Buf).map(|a| NOutputGatetype::Buf(a)),
        token(Token::Not).map(|a| NOutputGatetype::Not(a)),
    ))
    .parse_next(input)
}

pub fn pass_en_switchtype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PassEnSwitchtype<'s>, VerboseError<'s>> {
    alt((
        token(Token::Tranif0).map(|a| PassEnSwitchtype::Tranif0(a)),
        token(Token::Tranif1).map(|a| PassEnSwitchtype::Tranif1(a)),
        token(Token::Rtranif0).map(|a| PassEnSwitchtype::Rtranif0(a)),
        token(Token::Rtranif1).map(|a| PassEnSwitchtype::Rtranif1(a)),
    ))
    .parse_next(input)
}

pub fn pass_switchtype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PassSwitchtype<'s>, VerboseError<'s>> {
    alt((
        token(Token::Tran).map(|a| PassSwitchtype::Tran(a)),
        token(Token::Rtran).map(|a| PassSwitchtype::Rtran(a)),
    ))
    .parse_next(input)
}
