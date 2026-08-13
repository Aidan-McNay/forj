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
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(b|B)\s*[0-1xXzZ\?][0-1xXzZ\?_]*", |lex| lex.slice())]
    BinaryNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(o|O)\s*[0-7xXzZ\?][0-7xXzZ\?_]*", |lex| lex.slice())]
    OctalNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(d|D)\s*[0-9][0-9_]*", |lex| lex.slice())]
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(d|D)\s*(x|X|z|Z|\?)_*", |lex| lex.slice())]
    DecimalNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(h|H)\s*[0-9a-fA-FxXzZ\?][0-9a-fA-FxXzZ\?_]*", |lex| lex.slice())]
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

fn form_span<'a>(bytes: Range<usize>, original_span: &Span<'a>) -> Span<'a> {
    Span {
        file: original_span.file,
        bytes: Range {
            start: bytes.start + original_span.bytes.start,
            end: bytes.end + original_span.bytes.start,
        },
        included_from: original_span.included_from,
        expanded_from: original_span.expanded_from,
    }
}

// TODO: Put start_token directly in dest
pub fn preprocess_possible_number<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    start_token: SpannedToken<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let mut buffer = state.get_slice(&start_token.1).unwrap();
    let original_span = &start_token.1;
    let prev_span_end = start_token.1.bytes.end;
    let mut popped_tokens = vec![];
    loop {
        let mut lexer: SpannedIter<'s, Number> =
            Lexer::new_partial(buffer).spanned();
        match lexer.next() {
            Some((Ok(number), span)) => {
                src.prepend_tokens(std::iter::once(SpannedToken(
                    number.into(),
                    form_span(span, original_span),
                )));
            }
            Some((Err(_), _)) => {
                // Not a number; continue as usual
                src.prepend_tokens(std::iter::once(start_token));
                return Ok(());
            }
            None => {
                // Add the next token to the buffer
                let next_spanned_token =
                    match preprocess_single(src, state, cache) {
                        Ok(Some(next_token)) => next_token,
                        Ok(None) => {
                            // Clean up first
                            src.prepend_tokens(popped_tokens.into_iter());
                            src.prepend_tokens(std::iter::once(start_token));
                            return Ok(());
                        }
                        Err(err) => {
                            // Clean up first
                            src.prepend_tokens(popped_tokens.into_iter());
                            src.prepend_tokens(std::iter::once(start_token));
                            return Err(err);
                        }
                    };
                let mut new_buffer: String = buffer.to_owned();
                if next_spanned_token.1.bytes.end > prev_span_end {
                    new_buffer = new_buffer
                        + std::iter::repeat(' ')
                            .take(
                                next_spanned_token.1.bytes.end - prev_span_end,
                            )
                            .collect::<String>()
                            .as_str();
                }
                new_buffer = new_buffer
                    + state.get_slice(&next_spanned_token.1).unwrap();
                buffer = cache.retain_string(new_buffer);
                popped_tokens.push(next_spanned_token);
            }
        }
    }
}
