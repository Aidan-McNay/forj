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
    directive_span: Span<'s>,
) -> Result<(&'s str, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective { directive_span });
    };
    match spanned_token {
        SpannedToken(Token::UnsignedNumber(num_text), num_span) => {
            Ok((num_text, num_span))
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
    directive_span: Span<'s>,
) -> Result<(&'s str, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective { directive_span });
    };
    match spanned_token {
        SpannedToken(Token::StringLiteral(file_name), file_name_span) => {
            Ok((file_name, file_name_span))
        }
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a file name")],
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
    directive_span: Span<'s>,
) -> Result<(LineDirectiveLevel, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective { directive_span });
    };
    match spanned_token {
        SpannedToken(Token::UnsignedNumber("0"), num_span) => {
            Ok((LineDirectiveLevel::Other, num_span))
        }
        SpannedToken(Token::UnsignedNumber("1"), num_span) => {
            Ok((LineDirectiveLevel::EnterInclude, num_span))
        }
        SpannedToken(Token::UnsignedNumber("2"), num_span) => {
            Ok((LineDirectiveLevel::ExitInclude, num_span))
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

pub fn preprocess_line<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let (new_number, _) =
        get_line_number(src, state, cache, directive_span.clone())?;
    let (new_filename, _) =
        get_line_file(src, state, cache, directive_span.clone())?;
    let _ = get_line_level(src, state, cache, directive_span.clone())?; // Not currently used
    state.add_line_directive(new_filename, new_number, directive_span); // TODO: Handle bad input
    Ok(())
}
