// =======================================================================
// udp_body.rs
// =======================================================================
// Parsing for 1800-2023 A.5.3

use crate::Span;
use crate::*;
use logos::Span as ByteSpan;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn udp_body_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpBody<'s>, VerboseError<'s>> {
    alt((
        combinational_body_parser.map(|a| UdpBody::Combinational(Box::new(a))),
        sequential_body_parser.map(|a| UdpBody::Sequential(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn combinational_body_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CombinationalBody<'s>, VerboseError<'s>> {
    (
        token(Token::Table),
        combinational_entry_parser,
        repeat_note(combinational_entry_parser),
        token(Token::Endtable),
    )
        .map(|(a, b, c, d)| CombinationalBody(a, b, c, d))
        .parse_next(input)
}

pub fn combinational_entry_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CombinationalEntry<'s>, VerboseError<'s>> {
    (
        level_input_list_parser,
        token(Token::Colon),
        output_symbol_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| CombinationalEntry(a, b, c, d))
        .parse_next(input)
}

pub fn sequential_body_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequentialBody<'s>, VerboseError<'s>> {
    (
        opt_note(udp_initial_statement_parser),
        token(Token::Table),
        sequential_entry_parser,
        repeat_note(sequential_entry_parser),
        token(Token::Endtable),
    )
        .map(|(a, b, c, d, e)| SequentialBody(a, b, c, d, e))
        .parse_next(input)
}

pub fn udp_initial_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpInitialStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Initial),
        output_port_identifier_parser,
        token(Token::Eq),
        init_val_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| UdpInitialStatement(a, b, c, d, e))
        .parse_next(input)
}

pub fn init_val_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InitVal<'s>, VerboseError<'s>> {
    let _binary_parser =
        binary_number_parser.verify_map(|BinaryNumber(text, metadata)| {
            match text {
                "1'b0" => Some(InitVal::LittleB0(metadata)),
                "1'b1" => Some(InitVal::LittleB1(metadata)),
                "1'bx" => Some(InitVal::LittleBx(metadata)),
                "1'bX" => Some(InitVal::LittleBX(metadata)),
                "1'B0" => Some(InitVal::BigB0(metadata)),
                "1'B1" => Some(InitVal::BigB1(metadata)),
                "1'Bx" => Some(InitVal::BigBx(metadata)),
                "1'BX" => Some(InitVal::BigBX(metadata)),
                _ => None,
            }
        });
    let _number_parser = unsigned_number_parser.verify_map(|a| match a {
        UnsignedNumber("0", metadata) => Some(InitVal::Zero(metadata)),
        UnsignedNumber("1", metadata) => Some(InitVal::One(metadata)),
        _ => None,
    });
    alt((_binary_parser, _number_parser)).parse_next(input)
}

pub fn sequential_entry_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequentialEntry<'s>, VerboseError<'s>> {
    (
        seq_input_list_parser,
        token(Token::Colon),
        current_state_parser,
        token(Token::Colon),
        next_state_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| SequentialEntry(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn seq_input_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SeqInputList<'s>, VerboseError<'s>> {
    alt((
        level_input_list_parser.map(|a| SeqInputList::Level(Box::new(a))),
        edge_input_list_parser.map(|a| SeqInputList::Edge(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn level_input_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LevelInputList<'s>, VerboseError<'s>> {
    (level_symbol_parser, repeat_note(level_symbol_parser))
        .map(|(a, b)| LevelInputList(a, b))
        .parse_next(input)
}

pub fn edge_input_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EdgeInputList<'s>, VerboseError<'s>> {
    (
        repeat_note(level_symbol_parser),
        edge_indicator_parser,
        repeat_note(level_symbol_parser),
    )
        .map(|(a, b, c)| EdgeInputList(a, b, c))
        .parse_next(input)
}

fn double_level_symbol_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<(LevelSymbol<'s>, LevelSymbol<'s>), VerboseError<'s>> {
    let text_to_level_symbol =
        |character: char, metadata: Metadata<'s>| match character {
            '0' => Some(LevelSymbol::Zero(metadata)),
            '1' => Some(LevelSymbol::One(metadata)),
            'x' => Some(LevelSymbol::LittleX(metadata)),
            'X' => Some(LevelSymbol::BigX(metadata)),
            '?' => Some(LevelSymbol::Quest(metadata)),
            'b' => Some(LevelSymbol::LittleB(metadata)),
            'B' => Some(LevelSymbol::BigB(metadata)),
            _ => None,
        };
    identifier_parser
        .verify_map(|a| match a {
            Identifier::EscapedIdentifier(_) => None,
            Identifier::SimpleIdentifier((text, metadata)) => {
                let char_vec: Vec<char> = text.chars().collect();
                if char_vec.len() != 2 {
                    None
                } else {
                    let span_midpoint = (metadata.span.bytes.start
                        + metadata.span.bytes.end)
                        / 2;
                    let span0 = Span {
                        file: metadata.span.file,
                        bytes: ByteSpan {
                            start: metadata.span.bytes.start,
                            end: span_midpoint,
                        },
                        expanded_from: metadata.span.expanded_from,
                        included_from: metadata.span.included_from,
                    };
                    let span1 = Span {
                        file: metadata.span.file,
                        bytes: ByteSpan {
                            start: span_midpoint,
                            end: metadata.span.bytes.end,
                        },
                        expanded_from: metadata.span.expanded_from,
                        included_from: metadata.span.included_from,
                    };
                    let metadata0 = Metadata::new(span0, vec![]);
                    let mut metadata1 = metadata;
                    metadata1.span = span1;
                    match (
                        text_to_level_symbol(char_vec[0], metadata0),
                        text_to_level_symbol(char_vec[1], metadata1),
                    ) {
                        (Some(ls0), Some(ls1)) => Some((ls0, ls1)),
                        _ => None,
                    }
                }
            }
        })
        .parse_next(input)
}

pub fn edge_indicator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EdgeIndicator<'s>, VerboseError<'s>> {
    let _explicit_parser = (
        token(Token::Paren),
        alt((
            (level_symbol_parser, level_symbol_parser),
            double_level_symbol_parser,
        )),
        token(Token::EParen),
    )
        .map(|(a, (b, c), d)| EdgeIndicator::Explicit(Box::new((a, b, c, d))));
    alt((
        _explicit_parser,
        edge_symbol_parser.map(|a| EdgeIndicator::Shorthand(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn current_state_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CurrentState<'s>, VerboseError<'s>> {
    level_symbol_parser
        .map(|a| CurrentState(a))
        .parse_next(input)
}

pub fn next_state_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NextState<'s>, VerboseError<'s>> {
    alt((
        output_symbol_parser.map(|a| NextState::Output(Box::new(a))),
        token(Token::Minus).map(|a| NextState::Minus(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn output_symbol_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OutputSymbol<'s>, VerboseError<'s>> {
    let _identifier_parser = identifier_parser.verify_map(|a| match a {
        Identifier::SimpleIdentifier(("x", metadata)) => {
            Some(OutputSymbol::LittleX(metadata))
        }
        Identifier::SimpleIdentifier(("X", metadata)) => {
            Some(OutputSymbol::BigX(metadata))
        }
        _ => None,
    });
    let _number_parser = unsigned_number_parser.verify_map(|a| match a {
        UnsignedNumber("0", metadata) => Some(OutputSymbol::Zero(metadata)),
        UnsignedNumber("1", metadata) => Some(OutputSymbol::One(metadata)),
        _ => None,
    });
    alt((_identifier_parser, _number_parser)).parse_next(input)
}

pub fn level_symbol_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LevelSymbol<'s>, VerboseError<'s>> {
    let _identifier_parser = identifier_parser.verify_map(|a| match a {
        Identifier::SimpleIdentifier(("x", metadata)) => {
            Some(LevelSymbol::LittleX(metadata))
        }
        Identifier::SimpleIdentifier(("X", metadata)) => {
            Some(LevelSymbol::BigX(metadata))
        }
        Identifier::SimpleIdentifier(("b", metadata)) => {
            Some(LevelSymbol::LittleB(metadata))
        }
        Identifier::SimpleIdentifier(("B", metadata)) => {
            Some(LevelSymbol::BigB(metadata))
        }
        _ => None,
    });
    let _number_parser = unsigned_number_parser.verify_map(|a| match a {
        UnsignedNumber("0", metadata) => Some(LevelSymbol::Zero(metadata)),
        UnsignedNumber("1", metadata) => Some(LevelSymbol::One(metadata)),
        _ => None,
    });
    alt((
        _identifier_parser,
        _number_parser,
        token(Token::Quest).map(|a| LevelSymbol::Quest(a)),
    ))
    .parse_next(input)
}

pub fn edge_symbol_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EdgeSymbol<'s>, VerboseError<'s>> {
    let _identifier_parser = identifier_parser.verify_map(|a| match a {
        Identifier::SimpleIdentifier(("r", metadata)) => {
            Some(EdgeSymbol::LittleR(metadata))
        }
        Identifier::SimpleIdentifier(("R", metadata)) => {
            Some(EdgeSymbol::BigR(metadata))
        }
        Identifier::SimpleIdentifier(("f", metadata)) => {
            Some(EdgeSymbol::LittleF(metadata))
        }
        Identifier::SimpleIdentifier(("F", metadata)) => {
            Some(EdgeSymbol::BigF(metadata))
        }
        Identifier::SimpleIdentifier(("p", metadata)) => {
            Some(EdgeSymbol::LittleP(metadata))
        }
        Identifier::SimpleIdentifier(("P", metadata)) => {
            Some(EdgeSymbol::BigP(metadata))
        }
        Identifier::SimpleIdentifier(("n", metadata)) => {
            Some(EdgeSymbol::LittleN(metadata))
        }
        Identifier::SimpleIdentifier(("N", metadata)) => {
            Some(EdgeSymbol::BigN(metadata))
        }
        _ => None,
    });
    alt((
        _identifier_parser,
        token(Token::Star).map(|a| EdgeSymbol::Star(a)),
    ))
    .parse_next(input)
}
