// =======================================================================
// line.rs
// =======================================================================
// Preprocessing for `line directives

use crate::Span;
use crate::*;

fn get_line_number<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    directive_span: &Span<'s>,
) -> Result<(&'s str, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: directive_span.clone(),
        });
    };
    match spanned_token {
        SpannedToken(Token::UnsignedNumber(num_text), num_span) => {
            Ok((unsafe { std::str::from_utf8_unchecked(num_text) }, num_span))
        }
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a line number")],
            },
        }),
    }
}

fn get_line_file<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    directive_span: &Span<'s>,
) -> Result<(&'s str, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: directive_span.clone(),
        });
    };
    match spanned_token {
        SpannedToken(Token::StringLiteral(file_name), file_name_span) => Ok((
            unsafe { std::str::from_utf8_unchecked(file_name) },
            file_name_span,
        )),
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a file name (as a string)")],
            },
        }),
    }
}

enum LineDirectiveLevel {
    EnterInclude,
    ExitInclude,
    Other,
}

fn get_line_level<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    directive_span: &Span<'s>,
) -> Result<(LineDirectiveLevel, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: directive_span.clone(),
        });
    };
    match spanned_token {
        SpannedToken(Token::UnsignedNumber(num_bytes), num_span) => {
            match Into::<&[u8]>::into(num_bytes) {
                b"0" => Ok((LineDirectiveLevel::Other, num_span)),
                b"1" => Ok((LineDirectiveLevel::EnterInclude, num_span)),
                b"2" => Ok((LineDirectiveLevel::ExitInclude, num_span)),
                _ => Err(PreprocessorError::VerboseError {
                    err: VerboseError {
                        span: num_span,
                        found: Some(spanned_token.0),
                        expected: vec![Expectation::Label(
                            "a line level (0, 1, or 2)",
                        )],
                    },
                }),
            }
        }
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a line level (0, 1, or 2)")],
            },
        }),
    }
}

pub fn preprocess_line<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let (new_number, _) = get_line_number(src, state, cache, &directive_span)?;
    let (new_filename, _) = get_line_file(src, state, cache, &directive_span)?;
    let _ = get_line_level(src, state, cache, &directive_span)?; // Not currently used
    state.add_line_directive(new_filename, new_number, directive_span); // TODO: Handle bad input
    Ok(())
}
