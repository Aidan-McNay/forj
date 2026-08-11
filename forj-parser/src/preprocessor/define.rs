// =======================================================================
// define.rs
// =======================================================================
// Preprocessing for preprocessor definitions

use crate::Span;
use crate::*;

fn get_define_token<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    err_span: Span<'s>,
) -> Result<(SpannedToken<'s>, bool), PreprocessorError<'s>> {
    // Return a token, as well as indicating whether it's the
    // end of the define
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: err_span,
        });
    };
    let next_token = src.peek();
    match (spanned_token.0, next_token) {
        (Token::Bslash, Some(next_token)) => match next_token {
            SpannedToken(Token::Newline, _) => {
                let newline_token = src.next().unwrap();
                Ok((newline_token, false))
            }
            _ => Ok((spanned_token, false)),
        },
        (Token::Newline, _) => Ok((spanned_token, true)),
        (_, None) => Ok((spanned_token, true)),
        (_, Some(SpannedToken(Token::Newline, _))) => Ok((spanned_token, true)),
        (_, Some(_)) => Ok((spanned_token, false)),
    }
}

fn get_define_name<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    define_span: Span<'s>,
) -> Result<SpannedString<'s>, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: define_span,
        });
    };
    match spanned_token.0 {
        Token::SimpleIdentifier(id_str) => {
            Ok(SpannedString(id_str, spanned_token.1))
        }
        Token::EscapedIdentifier(id_str) => {
            Ok(SpannedString(id_str, spanned_token.1))
        }
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a preprocessor macro name")],
            },
        }),
    }
}

fn get_define_function_args<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    define_name: &'s str,
    define_name_span: &Span<'s>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<
    Option<(
        Span<'s>,
        Vec<(
            SpannedString<'s>,
            Option<(
                Span<'s>, // =
                Vec<SpannedToken<'s>>,
            )>,
        )>,
    )>,
    PreprocessorError<'s>,
> {
    let Some(spanned_token) = src.peek() else {
        return Ok(None);
    };
    match spanned_token.0 {
        Token::Paren => {
            if spanned_token.1.bytes.start != define_name_span.bytes.end {
                // Space in between indicates define body
                return Ok(None);
            }
            let paren_span = src.next().unwrap().1;
            let mut started_defaults = None;
            let mut function_args: Vec<(
                SpannedString<'s>,
                Option<(Span, Vec<SpannedToken<'s>>)>,
            )> = vec![];
            'get_args: loop {
                let mut next_arg_token = match get_define_function_arg(
                    src,
                    &mut function_args,
                    state,
                    cache,
                    define_name,
                    &mut started_defaults,
                    paren_span.clone(),
                ) {
                    Ok(()) => {
                        let (next_token, eod) =
                            get_define_token(src, paren_span.clone())?;
                        if eod && (next_token != Token::EParen) {
                            return Err(PreprocessorError::IncompleteDefine {
                                other_token: next_token.0,
                                other_span: next_token.1,
                            });
                        } else {
                            next_token
                        }
                    }
                    Err(PreprocessorError::EndOfFunctionArgument(
                        next_token,
                    )) => next_token,
                    Err(PreprocessorError::IncompleteDirective { .. })
                    | Err(PreprocessorError::NewlineInDefine(_)) => {
                        return Err(PreprocessorError::IncompleteDefine {
                            other_token: Token::Paren,
                            other_span: paren_span,
                        });
                    }
                    Err(err) => {
                        return Err(err);
                    }
                };
                loop {
                    match next_arg_token {
                        SpannedToken(Token::EParen, eparen_span) => {
                            break 'get_args Ok(Some((
                                eparen_span,
                                function_args,
                            )));
                        }
                        SpannedToken(Token::Comma, _) => {
                            continue 'get_args;
                        }
                        SpannedToken(Token::Newline, _) => {
                            let (next_token, eod) =
                                get_define_token(src, paren_span.clone())?;
                            next_arg_token = if eod {
                                return Err(
                                    PreprocessorError::IncompleteDefine {
                                        other_token: next_token.0,
                                        other_span: next_token.1,
                                    },
                                );
                            } else {
                                next_token
                            }
                        }
                        SpannedToken(_, _) => {
                            break 'get_args Err(
                                PreprocessorError::InvalidDefineArgument {
                                    other_token: next_arg_token.0,
                                    other_span: next_arg_token.1,
                                },
                            );
                        }
                    }
                }
            }
        }
        _ => Ok(None),
    }
}

fn get_define_function_arg<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<(
        SpannedString<'s>,
        Option<(Span<'s>, Vec<SpannedToken<'s>>)>,
    )>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    define_name: &'s str,
    started_defaults: &mut Option<SpannedString<'s>>,
    paren_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let arg_id = loop {
        match get_define_token(src, paren_span.clone())? {
            (SpannedToken(Token::SimpleIdentifier(id_str), id_span), _) => {
                break SpannedString(id_str, id_span);
            }
            (SpannedToken(Token::Newline, _), _) => {
                continue;
            }
            (err_spanned_token, _) => {
                return Err(PreprocessorError::InvalidDefineParameter {
                    other_token: err_spanned_token.0,
                    other_span: err_spanned_token.1,
                });
            }
        }
    };
    for prev_arg_id in dest.iter().map(|(id, _)| id) {
        if prev_arg_id.0 == arg_id.0 {
            return Err(PreprocessorError::DuplicateMacroParameter {
                define_name,
                param_name: arg_id.0,
                dup_span: arg_id.1,
                prev_span: prev_arg_id.1.clone(),
            });
        }
    }
    let eq_span = match src.peek() {
        None => {
            return Err(PreprocessorError::IncompleteDirective {
                directive_span: paren_span,
            });
        }
        Some(SpannedToken(Token::Eq, _)) => {
            let SpannedToken(_, eq_span) = src.next().unwrap();
            eq_span
        }
        Some(_) => {
            if let Some(last_define_arg) = started_defaults {
                return Err(PreprocessorError::NoDefaultAfterDefault {
                    default_param: last_define_arg.0,
                    default_param_span: last_define_arg.1.clone(),
                    non_default_param: arg_id.0,
                    non_default_param_span: arg_id.1,
                });
            }
            dest.push((arg_id, None));
            return Ok(());
        }
    };
    let mut default_arg_text: Vec<SpannedToken<'s>> = vec![];
    let prev_in_define = state.enter_define();
    let prev_in_define_arg = state.enter_define_arg();
    let result = preprocess_helper(src, &mut default_arg_text, state, cache);
    state.exit_define(prev_in_define);
    state.exit_define_arg(prev_in_define_arg);
    match result {
        Ok(()) => Err(PreprocessorError::IncompleteDefine {
            other_token: Token::Paren,
            other_span: paren_span,
        }),
        Err(PreprocessorError::EndOfFunctionArgument(spanned_token)) => {
            *started_defaults = Some(arg_id.clone());
            dest.push((arg_id, Some((eq_span, default_arg_text))));
            Err(PreprocessorError::EndOfFunctionArgument(spanned_token))
        }
        Err(err) => Err(err),
    }
}

fn get_define_body<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<Option<Vec<SpannedToken<'s>>>, PreprocessorError<'s>> {
    let mut define_body: Vec<SpannedToken<'s>> = vec![];
    let prev_in_define = state.enter_define();
    let result = preprocess_helper(src, &mut define_body, state, cache);
    state.exit_define(prev_in_define);
    match result {
        Ok(()) => Ok(Some(define_body)),
        Err(PreprocessorError::NewlineInDefine(_)) => Ok(Some(define_body)),
        Err(err) => Err(err),
    }
}

pub fn preprocess_define<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    define_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let define_name = get_define_name(src, define_span)?;
    if let Some(prev_def_span) = state.get_define_decl(define_name.0) {
        state.err(PreprocessorError::RedefinedMacro {
            macro_name: define_name.0,
            redef_span: define_name.1.clone(),
            prev_def_span,
        });
        state.undefine(define_name.0);
    }
    let function_args = get_define_function_args(
        src,
        define_name.0,
        &define_name.1,
        state,
        cache,
    )?;
    let define_text = get_define_body(src, state, cache)?;
    let (define_body, define_span) = match function_args {
        Some((end_span, arg_vec)) => {
            let mut overall_span = define_name.1;
            overall_span.bytes.end = end_span.bytes.end;
            (
                DefineBody::Function(DefineFunction {
                    args: arg_vec,
                    body: define_text,
                }),
                overall_span,
            )
        }
        None => match define_text {
            Some(text_vec) => (DefineBody::Text(text_vec), define_name.1),
            None => (DefineBody::Empty, define_name.1),
        },
    };
    state.define(define_name.0, define_span, define_body);
    Ok(())
}

pub fn preprocess_undefine<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    define_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let undefine_name = get_define_name(src, define_span)?;
    if !state.undefine(undefine_name.0) {
        state.err(PreprocessorError::NotPreviouslyDefinedMacro {
            macro_name: undefine_name.0,
            macro_span: undefine_name.1,
        });
    }
    Ok(())
}

#[test]
fn basic() {
    check_preprocessor!(
        "`define TEST 1
        `TEST",
        vec![Token::UnsignedNumber("1")]
    )
}

#[test]
fn multi_token() {
    check_preprocessor!(
        "`define TEST 1 + 1 = 2
        `TEST",
        vec![
            Token::UnsignedNumber("1"),
            Token::Plus,
            Token::UnsignedNumber("1"),
            Token::Eq,
            Token::UnsignedNumber("2")
        ]
    )
}

#[test]
fn empty() {
    check_preprocessor!(
        "`define TEST
        2
        `TEST
        `TEST
        `TEST",
        vec![Token::UnsignedNumber("2"),]
    )
}

#[test]
fn escaped_newlines() {
    check_preprocessor!(
        "`define TEST \
        assign        \
        test_signal   \
        =             \
        1
        2
        3
        `TEST",
        vec![
            Token::UnsignedNumber("2"),
            Token::UnsignedNumber("3"),
            Token::Assign,
            Token::SimpleIdentifier("test_signal"),
            Token::Eq,
            Token::UnsignedNumber("1"),
        ]
    )
}

#[test]
#[should_panic(expected = "a preprocessor macro name")]
fn illegal_name() {
    check_preprocessor!(
        "`define logic 2
        ",
        Vec::<Token<'_>>::new()
    )
}

#[test]
fn undefine() {
    check_preprocessor!(
        "`define TEST 1
        `undef TEST",
        Vec::<Token<'_>>::new()
    )
}

#[test]
fn undefine_redefine() {
    check_preprocessor!(
        "`define TEST 1
        `undef TEST
        `define TEST 2
        `TEST",
        vec![Token::UnsignedNumber("2")]
    )
}

#[test]
#[should_panic(expected = "NotPreviouslyDefinedMacro")]
fn undef_without_defining() {
    check_preprocessor!("`undef TEST", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "RedefinedMacro")]
fn redefining() {
    check_preprocessor!(
        "`define TEST 1
        `define TEST",
        Vec::<Token<'_>>::new()
    )
}

#[test]
fn function() {
    check_preprocessor!("`define TEST(a, b) a + b", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "InvalidDefineParameter")]
fn empty_function() {
    check_preprocessor!("`define TEST()", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "InvalidDefineParameter")]
fn missing_arg_function() {
    check_preprocessor!("`define TEST(a,)", Vec::<Token<'_>>::new())
}

#[test]
fn function_with_defaults() {
    check_preprocessor!(
        "`define TEST(a, b = 1 + 2) a - b",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic(expected = "NoDefaultAfterDefault")]
fn function_with_no_default_after_default() {
    check_preprocessor!(
        "`define TEST(a, b = 1 + 2, c) a - b + c",
        Vec::<Token<'_>>::new()
    )
}
