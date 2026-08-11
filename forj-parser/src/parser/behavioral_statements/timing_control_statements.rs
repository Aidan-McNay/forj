// =======================================================================
// timing_control_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.5

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn procedural_timing_control_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProceduralTimingControlStatement<'s>, VerboseError<'s>> {
    (procedural_timing_control_parser, statement_or_null_parser)
        .map(|(a, b)| ProceduralTimingControlStatement(a, b))
        .parse_next(input)
}

pub fn delay_or_event_control_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DelayOrEventControl<'s>, VerboseError<'s>> {
    let _repeat_parser = (
        token(Token::Repeat),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        event_control_parser,
    )
        .map(|(a, b, c, d, e)| {
            DelayOrEventControl::Repeat(Box::new((a, b, c, d, e)))
        });
    alt((
        _repeat_parser,
        delay_control_parser.map(|a| DelayOrEventControl::Delay(Box::new(a))),
        event_control_parser.map(|a| DelayOrEventControl::Event(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn delay_control_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DelayControl<'s>, VerboseError<'s>> {
    let _value_parser = (token(Token::Pound), delay_value_parser)
        .map(|(a, b)| DelayControl::Value(Box::new((a, b))));
    let _mintypmax_parser = (
        token(Token::Pound),
        token(Token::Paren),
        mintypmax_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| DelayControl::Mintypmax(Box::new((a, b, c, d))));
    alt((_value_parser, _mintypmax_parser)).parse_next(input)
}

pub fn event_control_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EventControl<'s>, VerboseError<'s>> {
    let _clocking_parser =
        clocking_event_parser.map(|a| EventControl::Clocking(Box::new(a)));
    let _wildcard_parser = (token(Token::At), token(Token::Star))
        .map(|(a, b)| EventControl::Wildcard(Box::new((a, b))));
    let _wildcard_paren_parser = (
        token(Token::At),
        token(Token::Paren),
        token(Token::Star),
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| {
            EventControl::ParenWildcard(Box::new((a, b, c, d)))
        });
    alt((_wildcard_paren_parser, _wildcard_parser, _clocking_parser))
        .parse_next(input)
}

pub fn clocking_event_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClockingEvent<'s>, VerboseError<'s>> {
    let _identifier_parser = |input: &mut Tokens<'s>| {
        let at_sign = token(Token::At).parse_next(input)?;
        let opt_scope = opt_note(package_scope_parser).parse_next(input)?;
        match opt_scope {
            None => {
                let identifier =
                    hierarchical_identifier_parser.parse_next(input)?;
                Ok(ClockingEvent::Hierarchical(Box::new((at_sign, identifier))))
            }
            Some(scope) => {
                let identifier = identifier_parser.parse_next(input)?;
                Ok(ClockingEvent::Ps(Box::new((
                    at_sign,
                    PsIdentifier(Some(scope), identifier),
                ))))
            }
        }
    };
    let _expression_parser = (
        token(Token::At),
        token(Token::Paren),
        event_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| ClockingEvent::Expression(Box::new((a, b, c, d))));
    alt((_expression_parser, _identifier_parser)).parse_next(input)
}

fn basic_event_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EventExpression<'s>, VerboseError<'s>> {
    let _trigger_event_parser = (
        opt_note(edge_identifier_parser),
        expression_parser,
        opt_note((token(Token::Iff), expression_parser)),
    )
        .map(|(a, b, c)| EventExpression::Trigger(Box::new((a, b, c))));
    let _sequence_event_parser = (
        sequence_instance_parser,
        opt_note((token(Token::Iff), expression_parser)),
    )
        .map(|(a, b)| EventExpression::Sequence(Box::new((a, b))));
    let _paren_event_parser = (
        token(Token::Paren),
        event_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| EventExpression::Paren(Box::new((a, b, c))));
    alt((
        _paren_event_parser,
        _trigger_event_parser,
        _sequence_event_parser,
    ))
    .parse_next(input)
}

pub fn event_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EventExpression<'s>, VerboseError<'s>> {
    let _or_expression_parser = (
        basic_event_expression_parser,
        token(Token::Or),
        event_expression_parser,
    )
        .map(|(a, b, c)| EventExpression::Or(Box::new((a, b, c))));
    let _comma_expression_parser = (
        basic_event_expression_parser,
        token(Token::Comma),
        event_expression_parser,
    )
        .map(|(a, b, c)| EventExpression::Comma(Box::new((a, b, c))));
    alt((
        _or_expression_parser,
        _comma_expression_parser,
        basic_event_expression_parser,
    ))
    .parse_next(input)
}

pub fn procedural_timing_control_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProceduralTimingControl<'s>, VerboseError<'s>> {
    alt((
        delay_control_parser
            .map(|a| ProceduralTimingControl::Delay(Box::new(a))),
        event_control_parser
            .map(|a| ProceduralTimingControl::Event(Box::new(a))),
        cycle_delay_parser.map(|a| ProceduralTimingControl::Cycle(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn jump_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<JumpStatement<'s>, VerboseError<'s>> {
    let _return_parser = (
        token(Token::Return),
        opt_note(expression_parser),
        token(Token::SColon),
    )
        .map(|(a, b, c)| JumpStatement::Return(Box::new((a, b, c))));
    let _break_parser = (token(Token::Break), token(Token::SColon))
        .map(|(a, b)| JumpStatement::Break(Box::new((a, b))));
    let _continue_parser = (token(Token::Continue), token(Token::SColon))
        .map(|(a, b)| JumpStatement::Continue(Box::new((a, b))));
    alt((_return_parser, _break_parser, _continue_parser)).parse_next(input)
}

pub fn wait_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<WaitStatement<'s>, VerboseError<'s>> {
    let _expression_parser = (
        token(Token::Wait),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e)| {
            WaitStatement::Expression(Box::new((a, b, c, d, e)))
        });
    let _fork_parser =
        (token(Token::Wait), token(Token::Fork), token(Token::SColon))
            .map(|(a, b, c)| WaitStatement::Fork(Box::new((a, b, c))));
    let _order_parser = (
        token(Token::WaitOrder),
        token(Token::Paren),
        hierarchical_identifier_parser,
        repeat_note((token(Token::Comma), hierarchical_identifier_parser)),
        token(Token::EParen),
        action_block_parser,
    )
        .map(|(a, b, c, d, e, f)| {
            WaitStatement::Order(Box::new((a, b, c, d, e, f)))
        });
    alt((_fork_parser, _expression_parser, _order_parser)).parse_next(input)
}

pub fn event_trigger_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EventTrigger<'s>, VerboseError<'s>> {
    let _blocking_parser = (
        token(Token::MinusGt),
        hierarchical_event_identifier_parser,
        nonrange_select_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| EventTrigger::Blocking(Box::new((a, b, c, d))));
    let _nonblocking_parser = (
        token(Token::MinusGtGt),
        opt_note(delay_or_event_control_parser),
        hierarchical_event_identifier_parser,
        nonrange_select_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| {
            EventTrigger::Nonblocking(Box::new((a, b, c, d, e)))
        });
    alt((_blocking_parser, _nonblocking_parser)).parse_next(input)
}

pub fn disable_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DisableStatement<'s>, VerboseError<'s>> {
    let _task_parser = (
        token(Token::Disable),
        hierarchical_task_identifier_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| DisableStatement::Task(Box::new((a, b, c))));
    let _block_parser = (
        token(Token::Disable),
        hierarchical_block_identifier_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| DisableStatement::Block(Box::new((a, b, c))));
    let _fork_parser = (
        token(Token::Disable),
        token(Token::Fork),
        token(Token::SColon),
    )
        .map(|(a, b, c)| DisableStatement::Fork(Box::new((a, b, c))));
    alt((_task_parser, _block_parser, _fork_parser)).parse_next(input)
}
