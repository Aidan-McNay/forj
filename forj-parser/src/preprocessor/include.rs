// =======================================================================
// define.rs
// =======================================================================
// Preprocessing for preprocessor definitions

use crate::Span;
use crate::*;

pub(crate) const MAX_INCLUDE_DEPTH: usize = 50;

pub enum IncludePath<'a> {
    ProjectRelative(&'a str),
    ToolRelative(&'a str),
}

fn get_include_path<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    define_span: Span<'s>,
) -> Result<(IncludePath<'s>, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: define_span,
        });
    };
    match spanned_token.0 {
        Token::StringLiteral(id_str) => {
            Ok((IncludePath::ProjectRelative(id_str), spanned_token.1))
        }
        Token::Lt => loop {
            let Some(next_token) = preprocess_single(src, state, cache)? else {
                break Err(PreprocessorError::VerboseError {
                    err: VerboseError {
                        span: spanned_token.1,
                        found: Some(spanned_token.0),
                        expected: vec![Expectation::Label("an include path")],
                    },
                });
            };
            match next_token.0 {
                Token::Newline => {
                    break Err(PreprocessorError::VerboseError {
                        err: VerboseError {
                            span: spanned_token.1,
                            found: Some(spanned_token.0),
                            expected: vec![Expectation::Label(
                                "an include path",
                            )],
                        },
                    });
                }
                Token::Gt => {
                    let mut path_span = spanned_token.1.clone();
                    path_span.bytes.start = spanned_token.1.bytes.end;
                    path_span.bytes.end = next_token.1.bytes.start;
                    let path = state.get_slice(&path_span).unwrap();
                    let mut overall_span = next_token.1;
                    overall_span.bytes.start = spanned_token.1.bytes.start;
                    break Ok((IncludePath::ToolRelative(path), overall_span));
                }
                _ => (),
            }
        },
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("an include path")],
            },
        }),
    }
}

pub fn preprocess_include<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    include_span: &'s Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    if state.include_depth >= MAX_INCLUDE_DEPTH {
        return Err(PreprocessorError::IncludeDepth {
            include_span: include_span.clone(),
        });
    }
    let (include_path_text, file_span) =
        get_include_path(src, state, cache, include_span.clone())?;
    // Treat both include types as the same
    let include_path_text = match include_path_text {
        IncludePath::ProjectRelative(text) => text,
        IncludePath::ToolRelative(text) => text,
    };
    let (include_path, included_file) =
        state.retain_include_file(include_path_text, file_span, cache)?;
    let included_file_contents =
        lex_helper(included_file, include_path, Some(include_span)).tokens();
    if let Some(size_hint) = included_file_contents.size_hint().1 {
        dest.reserve(size_hint);
    }
    state.include_depth += 1;
    let result = preprocess_helper(
        &mut TokenIterator::new(included_file_contents),
        dest,
        state,
        cache,
    );
    state.include_depth -= 1;
    result
}

#[test]
fn include_in_untaken_conditional() {
    check_preprocessor!(
        "`ifdef NOT_DEFINED
        `include \"dont/get/this/file.v\"
        `endif",
        Vec::<Token<'_>>::new()
    )
}
