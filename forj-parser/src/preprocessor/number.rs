// =======================================================================
// number.rs
// =======================================================================
// Concatenate numbers, in the event they were split across text macros
//
// Depending on the number, they may span multiple tokens if not initially
// parsed as a number; the easiest/cleanest way to handle is to re-lex,
// knowing that numbers will always be in more tokens, never fewer

use core::ops::Range;

use crate::*;
use logos::{Lexer, Logos, SpannedIter};

#[derive(Logos, Debug, Clone, PartialEq, Eq, Copy)]
enum Number<'a> {
    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*", |lex| lex.slice())]
    FixedPointNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(b|B)[^\S\r\n]*[0-1xXzZ\?][0-1xXzZ\?_]*", |lex| lex.slice())]
    BinaryNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(o|O)[^\S\r\n]*[0-7xXzZ\?][0-7xXzZ\?_]*", |lex| lex.slice())]
    OctalNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(d|D)[^\S\r\n]*[0-9][0-9_]*", |lex| lex.slice())]
    #[regex(r"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(d|D)[^\S\r\n]*(x|X|z|Z|\?)_*", |lex| lex.slice())]
    DecimalNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(h|H)[^\S\r\n]*[0-9a-fA-FxXzZ\?][0-9a-fA-FxXzZ\?_]*", |lex| lex.slice())]
    HexNumber(&'a str),
    #[regex(r"[0-9][0-9_]*(\.[0-9][0-9_]*)?(e|E)(\+|-)?[0-9][0-9_]*", |lex| lex.slice())]
    ScientificNumber(&'a str),
}

impl<'a> From<Number<'a>> for Token<'a> {
    fn from(value: number::Number<'a>) -> Self {
        match value {
            Number::FixedPointNumber(text) => Token::FixedPointNumber(text),
            Number::BinaryNumber(text) => Token::BinaryNumber(text),
            Number::OctalNumber(text) => Token::OctalNumber(text),
            Number::DecimalNumber(text) => Token::DecimalNumber(text),
            Number::HexNumber(text) => Token::HexNumber(text),
            Number::ScientificNumber(text) => Token::ScientificNumber(text),
        }
    }
}

fn get_expanded_slice<'s>(
    span: &Span<'s>,
    state: &PreprocessorState<'s>,
) -> Option<&'s str> {
    let mut curr_span = span;
    loop {
        if let Some(expanded_span) = curr_span.expanded_from {
            curr_span = expanded_span;
        } else {
            break;
        }
    }
    state.get_slice(curr_span)
}

// The numbers we've tried so far
#[derive(Debug)]
struct NumberHistory<'s> {
    pub curr_num: (&'s str, Span<'s>),
    pub prev_num: (&'s str, Span<'s>),
}

impl<'s> NumberHistory<'s> {
    fn new(
        start_token: &SpannedToken<'s>,
        state: &PreprocessorState<'s>,
    ) -> Self {
        let curr_token_slice =
            get_expanded_slice(&start_token.1, state).unwrap();
        Self {
            curr_num: (
                curr_token_slice,
                Span {
                    expanded_from: None,
                    ..start_token.1.clone()
                },
            ),
            prev_num: (
                curr_token_slice,
                Span {
                    expanded_from: None,
                    ..start_token.1.clone()
                },
            ), // Need to initialize with something
        }
    }
    fn push_token(
        &mut self,
        new_token: &SpannedToken<'s>,
        state: &PreprocessorState<'s>,
        cache: &'s PreprocessorCache<'s>,
    ) {
        let mut next_num_str = self.curr_num.0.to_owned();
        if new_token.1.bytes.start > self.curr_num.1.bytes.end {
            next_num_str = next_num_str
                + std::iter::repeat(' ')
                    .take(new_token.1.bytes.start - self.curr_num.1.bytes.end)
                    .collect::<String>()
                    .as_str();
        };
        next_num_str =
            next_num_str + get_expanded_slice(&new_token.1, state).unwrap();
        let next_num_span = Span {
            bytes: Range {
                start: self.curr_num.1.bytes.start,
                end: new_token.1.bytes.end,
            },
            ..self.curr_num.1
        };
        self.prev_num = std::mem::replace(
            &mut self.curr_num,
            (cache.retain_string(next_num_str), next_num_span),
        );
    }
}

pub fn preprocess_possible_number<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    start_token: SpannedToken<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let mut number_hist = NumberHistory::new(&start_token, state);
    let mut popped_tokens = vec![];
    loop {
        let mut lexer: SpannedIter<'s, Number> =
            Lexer::new_partial(number_hist.curr_num.0).spanned();
        match lexer.next() {
            Some((Ok(number), _)) => {
                // Produces the number before the latest addition, since the lexer
                // now finishes with an unmatched addition
                let new_span = number_hist.prev_num.1;
                src.prepend_tokens(
                    popped_tokens
                        .into_iter()
                        .filter(|token: &SpannedToken<'_>| {
                            token.1.bytes.start >= new_span.bytes.end
                        })
                        .collect::<Vec<_>>()
                        .into_iter(),
                );
                src.prepend_tokens(std::iter::once(SpannedToken(
                    number.into(),
                    new_span,
                )));
                return Ok(());
            }
            Some((Err(_), _)) => {
                // Not a number; continue as usual
                src.prepend_tokens(popped_tokens.into_iter());
                dest.push(start_token);
                return Ok(());
            }
            None => {
                // Add the next token to the buffer
                let next_spanned_token =
                    match preprocess_single(src, state, cache) {
                        Ok(Some(next_token)) => next_token,
                        Ok(None) => {
                            // Check if we have a non-greedy match
                            let mut complete_lexer: SpannedIter<'s, Number> =
                                Lexer::new(number_hist.curr_num.0).spanned();
                            if let Some((Ok(number), _)) = complete_lexer.next()
                            {
                                src.prepend_tokens(std::iter::once(
                                    SpannedToken(
                                        number.into(),
                                        number_hist.curr_num.1,
                                    ),
                                ));
                            } else {
                                // Clean up first
                                src.prepend_tokens(popped_tokens.into_iter());
                                dest.push(start_token);
                            }
                            return Ok(());
                        }
                        Err(err) => {
                            // Clean up first
                            src.prepend_tokens(popped_tokens.into_iter());
                            dest.push(start_token);
                            return Err(err);
                        }
                    };
                number_hist.push_token(&next_spanned_token, state, cache);
                popped_tokens.push(next_spanned_token);
            }
        }
    }
}

#[test]
fn basic_number() {
    // Check that basic number parsing still works
    check_preprocessor!("3'o1", vec![Token::OctalNumber("3'o1")]);
    check_preprocessor!("7'hF", vec![Token::HexNumber("7'hF")])
}

#[test]
fn spaced_number() {
    // Check that basic number parsing still works
    check_preprocessor!("1 'b 0", vec![Token::BinaryNumber("1 'b 0")]);
    check_preprocessor!(
        "2    'd    7",
        vec![Token::DecimalNumber("2    'd    7")]
    )
}

#[test]
fn basic_substitution() {
    check_preprocessor!(
        "`define WIDTH 7
        `WIDTH'h4",
        vec![Token::HexNumber("7'h4")]
    )
}

#[test]
fn non_start_substitution() {
    check_preprocessor!(
        "`define BASE 'd
        5`BASE 4",
        vec![Token::DecimalNumber("5'd 4")]
    );
    check_preprocessor!(
        "`define VALUE 12
        4'O`VALUE",
        vec![Token::OctalNumber("4'O12")]
    )
}

#[test]
fn multiple_substitution() {
    check_preprocessor!(
        "`define SIZE 3
        `define BASE 'h
        `define VALUE 7
        `SIZE`BASE 7",
        vec![Token::HexNumber("3'h 7")]
    );
    check_preprocessor!(
        "`define SIZE 3
        `define BASE 'h
        `define VALUE 7
        `SIZE'h`VALUE",
        vec![Token::HexNumber("3'h7")]
    );
    check_preprocessor!(
        "`define SIZE 3
        `define BASE 'h
        `define VALUE 7
        3`BASE`VALUE",
        vec![Token::HexNumber("3'h7")]
    );
    check_preprocessor!(
        "`define SIZE 3
        `define BASE 'h
        `define VALUE 7
        `SIZE`BASE`VALUE",
        vec![Token::HexNumber("3'h7")]
    );
}

#[test]
fn multi_substitution_value() {
    check_preprocessor!(
        "`define VALUE1 F
        `define VALUE2 A
        16'h`VALUE1`VALUE2",
        vec![Token::HexNumber("16'hFA")]
    );
}

#[test]
fn trailing_tokens() {
    check_preprocessor!(
        "`define DEPTH 9
        `DEPTH'h4 + 9'h5",
        vec![
            Token::HexNumber("9'h4"),
            Token::Plus,
            Token::HexNumber("9'h5")
        ]
    );
    check_preprocessor!(
        "`define DEPTH 13
        `DEPTH'hFA+8'hDE",
        vec![
            Token::HexNumber("13'hFA"),
            Token::Plus,
            Token::HexNumber("8'hDE")
        ]
    );
    check_preprocessor!(
        "`define DEPTH 13
        `define VALUE FA
        `DEPTH'h`VALUE+8'hDE",
        vec![
            Token::HexNumber("13'hFA"),
            Token::Plus,
            Token::HexNumber("8'hDE")
        ]
    )
}
