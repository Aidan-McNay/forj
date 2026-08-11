// =======================================================================
// clocking_block.rs
// =======================================================================
// Parsing for 1800-2023 A.6.11

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn clocking_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockingDeclaration<'s>, VerboseError<'s>> {
    let _local_parser = (
        opt_note(token(Token::Default)),
        token(Token::Clocking),
        opt_note(clocking_identifier_parser),
        clocking_event_parser,
        token(Token::SColon),
        repeat_note(clocking_item_parser),
        token(Token::Endclocking),
        opt_note((token(Token::Colon), clocking_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            ClockingDeclaration::Local(Box::new((a, b, c, d, e, f, g, h)))
        });
    let _global_parser = (
        token(Token::Global),
        token(Token::Clocking),
        opt_note(clocking_identifier_parser),
        clocking_event_parser,
        token(Token::SColon),
        repeat_note(clocking_item_parser),
        token(Token::Endclocking),
        opt_note((token(Token::Colon), clocking_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            ClockingDeclaration::Global(Box::new((a, b, c, d, e, f, g, h)))
        });
    alt((_local_parser, _global_parser)).parse_next(input)
}

pub fn clocking_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockingItem<'s>, VerboseError<'s>> {
    let _default_parser = (
        token(Token::Default),
        default_skew_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| ClockingItem::Default(Box::new((a, b, c))));
    let _decl_parser = (
        clocking_direction_parser,
        list_of_clocking_decl_assign_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| ClockingItem::Decl(Box::new((a, b, c))));
    let _assert_parser = (
        attribute_instance_vec_parser,
        assertion_item_declaration_parser,
    )
        .map(|(a, b)| ClockingItem::Assert(Box::new((a, b))));
    alt((_default_parser, _decl_parser, _assert_parser)).parse_next(input)
}

pub fn default_skew_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DefaultSkew<'s>, VerboseError<'s>> {
    let _input_parser = (token(Token::Input), clocking_skew_parser);
    let _input_or_input_output_parser = (
        _input_parser,
        opt_note((token(Token::Output), clocking_skew_parser)),
    )
        .map(|((a, b), c)| match c {
            Some((c, d)) => DefaultSkew::InputOutput(Box::new((a, b, c, d))),
            None => DefaultSkew::Input(Box::new((a, b))),
        });
    let _output_parser = (token(Token::Output), clocking_skew_parser)
        .map(|(a, b)| DefaultSkew::Output(Box::new((a, b))));
    alt((_input_or_input_output_parser, _output_parser)).parse_next(input)
}

pub fn clocking_direction_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockingDirection<'s>, VerboseError<'s>> {
    let _input_parser = (token(Token::Input), opt_note(clocking_skew_parser));
    let _input_or_input_output_parser = (
        _input_parser,
        opt_note((token(Token::Output), opt_note(clocking_skew_parser))),
    )
        .map(|((a, b), c)| match c {
            Some((c, d)) => {
                ClockingDirection::InputOutput(Box::new((a, b, c, d)))
            }
            None => ClockingDirection::Input(Box::new((a, b))),
        });
    let _output_parser = (token(Token::Output), opt_note(clocking_skew_parser))
        .map(|(a, b)| ClockingDirection::Output(Box::new((a, b))));
    let _inout_parser =
        token(Token::Inout).map(|a| ClockingDirection::Inout(Box::new(a)));
    alt((_input_or_input_output_parser, _output_parser, _inout_parser))
        .parse_next(input)
}

pub fn list_of_clocking_decl_assign_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfClockingDeclAssign<'s>, VerboseError<'s>> {
    (
        clocking_decl_assign_parser,
        repeat_note((token(Token::Comma), clocking_decl_assign_parser)),
    )
        .map(|(a, b)| ListOfClockingDeclAssign(a, b))
        .parse_next(input)
}

pub fn clocking_decl_assign_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockingDeclAssign<'s>, VerboseError<'s>> {
    (
        signal_identifier_parser,
        opt_note((token(Token::Eq), expression_parser)),
    )
        .map(|(a, b)| ClockingDeclAssign(a, b))
        .parse_next(input)
}

pub fn clocking_skew_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockingSkew<'s>, VerboseError<'s>> {
    let _edge_parser = (edge_identifier_parser, opt_note(delay_control_parser))
        .map(|(a, b)| ClockingSkew::Edge(Box::new((a, b))));
    let _delay_parser =
        delay_control_parser.map(|a| ClockingSkew::Delay(Box::new(a)));
    alt((_edge_parser, _delay_parser)).parse_next(input)
}

pub fn clocking_drive_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockingDrive<'s>, VerboseError<'s>> {
    (
        clockvar_expression_parser,
        token(Token::LtEq),
        opt_note(cycle_delay_parser),
        expression_parser,
    )
        .map(|(a, b, c, d)| ClockingDrive(a, b, c, d))
        .parse_next(input)
}

pub fn cycle_delay_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CycleDelay<'s>, VerboseError<'s>> {
    let _integral_parser = (token(Token::PoundPound), integral_number_parser)
        .map(|(a, b)| CycleDelay::Integral(Box::new((a, b))));
    let _identifier_parser = (token(Token::PoundPound), identifier_parser)
        .map(|(a, b)| CycleDelay::Identifier(Box::new((a, b))));
    let _expression_parser = (
        token(Token::PoundPound),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| CycleDelay::Expression(Box::new((a, b, c, d))));
    alt((_integral_parser, _identifier_parser, _expression_parser))
        .parse_next(input)
}

pub fn clockvar_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Clockvar<'s>, VerboseError<'s>> {
    hierarchical_identifier_parser
        .map(|a| Clockvar(a))
        .parse_next(input)
}

pub fn clockvar_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockvarExpression<'s>, VerboseError<'s>> {
    (clockvar_parser, select_parser)
        .map(|(a, b)| ClockvarExpression(a, b))
        .parse_next(input)
}
