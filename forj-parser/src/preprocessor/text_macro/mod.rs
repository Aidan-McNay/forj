// =======================================================================
// mod.rs
// =======================================================================
// Preprocessing for macro instantiations

use crate::*;
use bstr::{BStr, BString};
use std::collections::HashMap;
use std::vec::IntoIter;
mod arg_replace;
use arg_replace::ArgReplace;
mod stringify;
use stringify::Stringify;
mod token_gluer;
use token_gluer::TokenGluer;

// Turn a token into the corresponding text
pub(crate) fn into_text<'a>(token: &Token<'a>) -> BString {
    match token {
        Token::UnsignedNumber(text)
        | Token::FixedPointNumber(text)
        | Token::BinaryNumber(text)
        | Token::OctalNumber(text)
        | Token::DecimalNumber(text)
        | Token::HexNumber(text)
        | Token::ScientificNumber(text)
        | Token::UnbasedUnsizedLiteral(text)
        | Token::SystemTfIdentifier(text)
        | Token::SimpleIdentifier(text)
        | Token::EscapedIdentifier(text)
        | Token::InvalidConcatenation(text) => (*text).to_owned(),
        Token::StringLiteral(text) => {
            let mut content: BString = BString::new(b"\"".to_vec());
            content.extend_from_slice(text);
            content.extend_from_slice(b"\"");
            content
        }
        Token::TripleQuoteStringLiteral(text) => {
            let mut content: BString = BString::new(b"\"\"\"".to_vec());
            content.extend_from_slice(text);
            content.extend_from_slice(b"\"\"\"");
            content
        }
        Token::BlockComment(text) => {
            let mut content: BString = BString::new(b"/*".to_vec());
            content.extend_from_slice(text);
            content.extend_from_slice(b"*/");
            content
        }
        Token::OnelineComment(text) => {
            let mut content: BString = BString::new(b"//".to_vec());
            content.extend_from_slice(text);
            content
        }
        Token::TextMacro(text) => {
            let mut content: BString = BString::new(b"`".to_vec());
            content.extend_from_slice(text);
            content
        }
        Token::Newline => BString::new(b"\n".to_vec()),
        Token::Comma => BString::new(b",".to_vec()),
        _ => token.as_str().into(),
    }
}

fn get_text_macro_args<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    define_span: &Span<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<(Vec<Vec<SpannedToken<'s>>>, Span<'s>), PreprocessorError<'s>> {
    let paren_span = loop {
        match src.next() {
            Some(SpannedToken(Token::Paren, paren_span)) => {
                break paren_span;
            }
            Some(SpannedToken(Token::Newline, _)) => (),
            _ => {
                return Err(PreprocessorError::NoMacroArguments {
                    macro_name: text_macro.0,
                    define_span: define_span.clone(),
                    use_span: text_macro.1,
                });
            }
        }
    };
    let mut arg_vec: Vec<Vec<SpannedToken<'s>>> = vec![];
    let end_span =
        loop {
            let mut new_arg: Vec<SpannedToken<'s>> = vec![];
            let prev_in_text_macro_arg =
                state.enter_text_macro_arg(text_macro.0.into());
            let result = preprocess_helper(src, &mut new_arg, state, cache);
            state.exit_text_macro_arg(prev_in_text_macro_arg);
            match result {
                Ok(()) => {
                    return Err(PreprocessorError::IncompleteDirective {
                        directive_span: paren_span,
                    });
                }
                Err(PreprocessorError::EndOfFunctionArgument(
                    SpannedToken(Token::EParen, eparen_span),
                )) => {
                    arg_vec.push(new_arg);
                    break eparen_span;
                }
                Err(PreprocessorError::EndOfFunctionArgument(
                    SpannedToken(Token::Comma, _),
                )) => {
                    arg_vec.push(new_arg);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        };
    let mut overall_span = text_macro.1;
    overall_span.bytes.end = end_span.bytes.end;
    Ok((arg_vec, overall_span))
}

fn resolve_text_macro_args<'s>(
    specified_args: Vec<Vec<SpannedToken<'s>>>,
    original_args: Vec<(SpannedString<'s>, Option<Vec<SpannedToken<'s>>>)>,
    define_span: &Span<'s>,
    text_macro: &(&'s str, Span<'s>),
) -> Result<
    HashMap<&'s BStr, (Span<'s>, Vec<SpannedToken<'s>>)>,
    PreprocessorError<'s>,
> {
    if specified_args.len() > original_args.len() {
        return Err(PreprocessorError::TooManyMacroArguments {
            macro_name: text_macro.0,
            define_span: define_span.clone(),
            use_span: text_macro.1.clone(),
            expected: original_args.len(),
            found: specified_args.len(),
        });
    }
    let mut specified_args_iter = specified_args.into_iter();
    let mut resolved_args: HashMap<
        &'s BStr,
        (Span<'s>, Vec<SpannedToken<'s>>),
    > = HashMap::new();
    for (arg_name, arg_tokens) in original_args.into_iter() {
        match specified_args_iter.next() {
            Some(specified_tokens) => {
                if specified_tokens.len() > 0 {
                    resolved_args.insert(
                        arg_name.0.into(),
                        (arg_name.1, specified_tokens),
                    );
                } else {
                    match arg_tokens {
                        Some(default_tokens) => {
                            resolved_args.insert(
                                arg_name.0.into(),
                                (arg_name.1, default_tokens),
                            );
                        }
                        None => {
                            resolved_args.insert(
                                arg_name.0.into(),
                                (arg_name.1, vec![]),
                            );
                        }
                    }
                }
            }
            None => match arg_tokens {
                Some(default_tokens) => {
                    resolved_args.insert(
                        arg_name.0.into(),
                        (arg_name.1, default_tokens),
                    );
                }
                None => {
                    return Err(PreprocessorError::MissingMacroArgument {
                        define_span: define_span.clone(),
                        use_span: text_macro.1.clone(),
                        param_name: arg_name.0,
                    });
                }
            },
        }
    }
    Ok(resolved_args)
}

struct SpanReplacer<'a> {
    text_macro_name: &'a str,
    text_macro_span: Span<'a>,
    tokens: IntoIter<SpannedToken<'a>>,
    cache: &'a PreprocessorCache<'a>,
}

impl<'a> Iterator for SpanReplacer<'a> {
    type Item = SpannedToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.next() {
            Some(token) => Some(self.update_span(token)),
            None => None,
        }
    }
}

impl<'a> DoubleEndedIterator for SpanReplacer<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.tokens.next_back() {
            Some(token) => Some(self.update_span(token)),
            None => None,
        }
    }
}

/// Create a clone of a span and all its expansions, inserting the new one
fn insert_base_expansion<'a>(
    cache: &'a PreprocessorCache<'a>,
    span: &'a Span<'a>,
    expanded_ref: (&'a str, &'a Span<'a>),
) -> &'a Span<'a> {
    let mut new_span = span.clone();
    match new_span.expanded_from {
        None => {
            new_span.expanded_from = Some(expanded_ref);
        }
        Some((macro_name, nested_expansion)) => {
            new_span.expanded_from = Some((
                macro_name,
                insert_base_expansion(cache, nested_expansion, expanded_ref),
            ));
        }
    };
    cache.retain_span(new_span)
}

impl<'a> SpanReplacer<'a> {
    fn new(
        text_macro_name: &'a str,
        text_macro_span: Span<'a>,
        tokens: IntoIter<SpannedToken<'a>>,
        cache: &'a PreprocessorCache<'a>,
    ) -> Self {
        Self {
            text_macro_name,
            text_macro_span,
            tokens,
            cache,
        }
    }

    fn update_span(&self, mut token: SpannedToken<'a>) -> SpannedToken<'a> {
        let original_span = std::mem::take(&mut token.1);
        token.1 = if original_span.file == "" {
            self.text_macro_span.clone()
        } else {
            // Check for nested macros
            let original_span_ref = match self.text_macro_span.expanded_from {
                Some((prev_macro_name, prev_expansion)) => (
                    prev_macro_name,
                    insert_base_expansion(
                        self.cache,
                        prev_expansion,
                        (
                            self.text_macro_name,
                            self.cache.retain_span(original_span),
                        ),
                    ),
                ),
                None => (
                    self.text_macro_name,
                    self.cache.retain_span(original_span),
                ),
            };
            Span {
                expanded_from: Some(original_span_ref),
                ..self.text_macro_span.clone()
            }
        };
        token
    }
}

impl<'a> ExactSizeIterator for SpanReplacer<'a> {
    fn len(&self) -> usize {
        self.tokens.len()
    }
}

fn recursive_definition<'s>(
    text_macro_name: &str,
    text_macro_span: &Span<'s>,
) -> bool {
    match text_macro_span.expanded_from {
        None => false,
        Some((macro_name, expanded_span)) => {
            if macro_name == text_macro_name {
                true
            } else {
                recursive_definition(text_macro_name, expanded_span)
            }
        }
    }
}

pub fn preprocess_macro<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<(), PreprocessorError<'s>> {
    match state.get_macro_tokens(text_macro.0) {
        Some((define_span, (token_vec, define_args))) => {
            let mut macro_span = text_macro.1.clone();
            let arguments = if let Some(define_args) = define_args {
                let (function_args, function_macro_span) = get_text_macro_args(
                    src,
                    state,
                    cache,
                    &define_span,
                    text_macro.clone(),
                )?;
                macro_span = function_macro_span;
                resolve_text_macro_args(
                    function_args,
                    define_args,
                    &define_span,
                    &text_macro,
                )?
            } else {
                HashMap::new()
            };
            if recursive_definition(text_macro.0, &text_macro.1) {
                return Err(PreprocessorError::RecursiveMacro {
                    macro_name: text_macro.0,
                    define_span: state.get_define_decl(text_macro.0).unwrap(),
                    use_span: text_macro.1,
                });
            }
            let token_result_vec = TokenGluer::new(
                Stringify::new(
                    ArgReplace::new(token_vec.into_iter(), &arguments),
                    &state,
                    cache,
                    vec![text_macro.0.into()],
                    &arguments,
                ),
                cache,
            )
            .collect::<Vec<_>>();
            let new_token_vec = token_result_vec
                .into_iter()
                .filter_map(|a| match a {
                    Ok(spanned_token) => Some(spanned_token),
                    Err(err) => {
                        state.err(err);
                        None
                    }
                })
                .collect::<Vec<_>>();
            let token_iter = SpanReplacer::new(
                text_macro.0,
                macro_span,
                new_token_vec.into_iter(),
                cache,
            );
            src.prepend_tokens(token_iter);
            Ok(())
        }
        None => Err(PreprocessorError::UndefinedMacro {
            undefined_name: text_macro.0,
            undefined_span: text_macro.1,
        }),
    }
}

#[test]
fn basic() {
    check_preprocessor!(
        "`define TEST_MACRO 1
        `TEST_MACRO `TEST_MACRO `TEST_MACRO",
        vec![
            Token::UnsignedNumber("1".into()),
            Token::UnsignedNumber("1".into()),
            Token::UnsignedNumber("1".into())
        ]
    )
}

#[test]
fn string_replacement() {
    check_preprocessor!(
        "`define TEST 1
        `define TARGET `\"`TEST`\"
        `undef TEST
        `define TEST 2
        `TARGET",
        vec![Token::StringLiteral("2".into())]
    );
    check_preprocessor!(
        "`define TEST whoops
        `define TARGET `\"\"\"This test looks `TEST`\"\"\"
        `undef TEST
        `define TEST correct
        `TARGET",
        vec![Token::TripleQuoteStringLiteral(
            "This test looks correct".into()
        )]
    )
}

#[test]
#[should_panic(expected = "UndefinedMacro")]
fn undefined() {
    check_preprocessor!("`UNDEFINED_MACRO", Vec::<Token<'_>>::new())
}

#[test]
fn function() {
    check_preprocessor!(
        "`define TEST(a, b) a + b
        `TEST(1, 2)
        `TEST(3, 4)",
        vec![
            Token::UnsignedNumber("1".into()),
            Token::Plus,
            Token::UnsignedNumber("2".into()),
            Token::UnsignedNumber("3".into()),
            Token::Plus,
            Token::UnsignedNumber("4".into()),
        ]
    )
}

#[test]
fn nested_function() {
    check_preprocessor!(
        "`define TOP(a,b) a + b
        `TOP( `TOP(b,1), `TOP(42,a) )",
        vec![
            Token::SimpleIdentifier("b".into()),
            Token::Plus,
            Token::UnsignedNumber("1".into()),
            Token::Plus,
            Token::UnsignedNumber("42".into()),
            Token::Plus,
            Token::SimpleIdentifier("a".into())
        ]
    )
}

#[test]
fn function_string_replacement() {
    check_preprocessor!(
        "`define TEST 1
        `define TARGET(a) `\"a = `TEST`\"
        `undef TEST
        `define TEST 2
        `TARGET(2)",
        vec![Token::StringLiteral("2 = 2".into())]
    );
    check_preprocessor!(
        "`define TEST whoops
        `define TARGET(test_name) `\"\"\"This test_name looks `TEST`\"\"\"
        `undef TEST
        `define TEST correct
        `TARGET(basic_test)",
        vec![Token::TripleQuoteStringLiteral(
            "This basic_test looks correct".into()
        )]
    )
}

#[test]
fn default_function() {
    check_preprocessor!(
        "`define TEST(a, b = 2) a + b
        `TEST(6, 7)
        `TEST(32)",
        vec![
            Token::UnsignedNumber("6".into()),
            Token::Plus,
            Token::UnsignedNumber("7".into()),
            Token::UnsignedNumber("32".into()),
            Token::Plus,
            Token::UnsignedNumber("2".into())
        ]
    )
}

#[test]
fn default_complex() {
    check_preprocessor!(
        "`define MACRO2(a=5, b, c=\"C\") a + b - c
        `MACRO2 (1, , 3)",
        vec![
            Token::UnsignedNumber("1".into()),
            Token::Plus,
            // Empty `b`
            Token::Minus,
            Token::UnsignedNumber("3".into())
        ]
    );
    check_preprocessor!(
        "`define MACRO2(a=5, b, c=\"C\") a + b - c
        `MACRO2 (, 2, )",
        vec![
            Token::UnsignedNumber("5".into()),
            Token::Plus,
            Token::UnsignedNumber("2".into()),
            Token::Minus,
            Token::StringLiteral("C".into())
        ]
    );
    check_preprocessor!(
        "`define MACRO2(a=5, b, c=\"C\") a + b - c
        `MACRO2 (, 2)",
        vec![
            Token::UnsignedNumber("5".into()),
            Token::Plus,
            Token::UnsignedNumber("2".into()),
            Token::Minus,
            Token::StringLiteral("C".into())
        ]
    )
}

#[test]
#[should_panic(expected = "NoMacroArguments")]
fn funtion_no_args() {
    check_preprocessor!(
        "`define TEST(a, b) a + b
        `TEST",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic(expected = "MissingMacroArgument")]
fn function_fewer_args() {
    check_preprocessor!(
        "`define TEST(a, b) a + b
        `TEST(42)",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic(expected = "TooManyMacroArguments")]
fn function_more_args() {
    check_preprocessor!(
        "`define TEST(a, b) a + b
        `TEST(42, 97, 33)",
        Vec::<Token<'_>>::new()
    )
}

#[cfg(test)]
mod stringify_with_macros {
    use super::*;
    #[test]
    fn basic() {
        check_preprocessor!(
            "`define TEST my_test
            `define STRINGIFY(a) `\"a`\"
            `STRINGIFY(`TEST)",
            vec![Token::StringLiteral("my_test".into())]
        )
    }

    #[test]
    fn empty() {
        check_preprocessor!(
            "`define TEST
            `define STRINGIFY(a) `\"a`\"
            `STRINGIFY(`TEST)",
            vec![Token::StringLiteral("".into())]
        )
    }

    #[test]
    fn function() {
        check_preprocessor!(
            "`define TEST(a, b) b``a
            `define STRINGIFY(a) `\"a`\"
            `STRINGIFY(`TEST(literal, my_))",
            vec![Token::StringLiteral("my_literal".into())]
        )
    }

    #[test]
    fn function_args_in_string() {
        check_preprocessor!(
            "`define TEST(a, b) b``a
            `define STRINGIFY(a) `\"a( _test, in_string )`\"
            `STRINGIFY(`TEST)",
            vec![Token::StringLiteral("in_string_test".into())]
        )
    }
}

#[cfg(test)]
mod recursive {
    use super::*;
    #[test]
    #[should_panic(expected = "RecursiveMacro")]
    fn recursive_use() {
        check_preprocessor!(
            "`define TEST1 `TEST2
            `define TEST2 `TEST1
            `TEST1",
            Vec::<Token<'_>>::new()
        )
    }

    #[test]
    fn recursive_redefine() {
        check_preprocessor!(
            "`define TEST1 `TEST2
            `define TEST2 `TEST1
            `define TEST1 2
            `TEST2",
            vec![Token::UnsignedNumber("2".into())],
            false
        )
    }

    #[test]
    fn recursive_stringify() {
        check_preprocessor!(
            "`define TEST `\"`TEST`\"
            `TEST",
            vec![Token::StringLiteral("`TEST".into())]
        )
    }

    #[test]
    fn recursive_stringify_multilevel() {
        check_preprocessor!(
            "`define TEST(a) `\"a`\"
            `define SUM(b) `VAL + b
            `define VAL `SUM(2)
            `TEST(`VAL)",
            vec![Token::StringLiteral("`VAL + 2".into())]
        )
    }
}
