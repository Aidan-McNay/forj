// =======================================================================
// define.rs
// =======================================================================
// Preprocessing for timescale directives

use crate::*;
use forj_syntax::SpanRelation;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimescaleValue {
    One,
    Ten,
    Hundred,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimescaleUnit {
    FS,
    PS,
    NS,
    US,
    MS,
    S,
}

#[derive(Clone, Debug)]
pub struct Timescale<'a> {
    def_span: Option<Span<'a>>,
    pub unit: (TimescaleValue, TimescaleUnit),
    pub precision: (TimescaleValue, TimescaleUnit),
}

impl<'a> Timescale<'a> {
    pub(crate) const fn new_default(
        unit: (TimescaleValue, TimescaleUnit),
        precision: (TimescaleValue, TimescaleUnit),
    ) -> Timescale<'a> {
        Timescale {
            def_span: None,
            unit,
            precision,
        }
    }

    pub fn new(
        def_span: Span<'a>,
        unit: (TimescaleValue, TimescaleUnit),
        precision: (TimescaleValue, TimescaleUnit),
    ) -> Result<Timescale<'a>, PreprocessorError<'a>> {
        if unit.1 > precision.1 {
            Ok(Timescale {
                def_span: Some(def_span),
                unit,
                precision,
            })
        } else if unit.1 < precision.1 {
            Err(PreprocessorError::InvalidRelativeTimescales {
                timescale_span: def_span,
            })
        } else {
            if precision.0 > unit.0 {
                Err(PreprocessorError::InvalidRelativeTimescales {
                    timescale_span: def_span,
                })
            } else {
                Ok(Timescale {
                    def_span: Some(def_span),
                    unit,
                    precision,
                })
            }
        }
    }

    pub fn is_valid(&self, delay_span: &Span<'a>) -> bool {
        if let Some(def_span) = &self.def_span {
            def_span.compare(delay_span) == SpanRelation::Earlier
        } else {
            // Using a default Timescale
            true
        }
    }
}

fn get_timescale<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    def_span: Span<'s>,
) -> Result<(TimescaleValue, TimescaleUnit), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: def_span,
        });
    };
    let timescale_value = match spanned_token.0 {
        Token::UnsignedNumber("1") => TimescaleValue::One,
        Token::UnsignedNumber("10") => TimescaleValue::Ten,
        Token::UnsignedNumber("100") => TimescaleValue::Hundred,
        _ => {
            return Err(PreprocessorError::VerboseError {
                err: VerboseError {
                    span: spanned_token.1,
                    found: Some(spanned_token.0),
                    expected: vec![Expectation::Label("1, 10, or 100")],
                },
            });
        }
    };
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: def_span,
        });
    };
    let timescale_unit = match spanned_token.0 {
        Token::SimpleIdentifier("s") => TimescaleUnit::S,
        Token::SimpleIdentifier("ms") => TimescaleUnit::MS,
        Token::SimpleIdentifier("us") => TimescaleUnit::US,
        Token::SimpleIdentifier("ns") => TimescaleUnit::NS,
        Token::SimpleIdentifier("ps") => TimescaleUnit::PS,
        Token::SimpleIdentifier("fs") => TimescaleUnit::FS,
        _ => {
            return Err(PreprocessorError::VerboseError {
                err: VerboseError {
                    span: spanned_token.1,
                    found: Some(spanned_token.0),
                    expected: vec![Expectation::Label(
                        "a recognized unit of time",
                    )],
                },
            });
        }
    };
    Ok((timescale_value, timescale_unit))
}

fn get_divider<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    def_span: Span<'s>,
) -> Result<Span<'s>, PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: def_span,
        });
    };
    match spanned_token.0 {
        Token::Slash => Ok(spanned_token.1),
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Token(Token::Slash)],
            },
        }),
    }
}

pub fn preprocess_timescale<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let timeunit = get_timescale(src, state, cache, directive_span.clone())?;
    let _ = get_divider(src, state, cache, directive_span.clone())?;
    let timeprecision =
        get_timescale(src, state, cache, directive_span.clone())?;
    state.add_timescale(Timescale::new(
        directive_span,
        timeunit,
        timeprecision,
    )?);
    Ok(())
}

#[test]
fn timescale() {
    check_preprocessor!(
        "`timescale 1 ns / 1 ps
        `timescale 10s / 100us
        `timescale 100 ms     / 1fs",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic(expected = "Slash")]
fn no_divider() {
    check_preprocessor!("`timescale 1 ns 1 ps", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "IncompleteDirective")]
fn no_first_measurement() {
    check_preprocessor!("`timescale", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "IncompleteDirective")]
fn no_second_measurement() {
    check_preprocessor!("`timescale 1 fs", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "1, 10, or 100")]
fn invalid_magnitude() {
    check_preprocessor!("`timescale 23 fs", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "a recognized unit of time")]
fn invalid_unit() {
    check_preprocessor!("`timescale 10 bananas", Vec::<Token<'_>>::new())
}

#[test]
fn equal_timescales() {
    check_preprocessor!("`timescale 10 s / 10 s", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "InvalidRelativeTimescales")]
fn larger_precision_unit() {
    check_preprocessor!("`timescale 100 fs / 1 s", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "InvalidRelativeTimescales")]
fn larger_precision_value() {
    check_preprocessor!("`timescale 10 fs / 100 fs", Vec::<Token<'_>>::new())
}
