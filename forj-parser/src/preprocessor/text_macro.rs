// =======================================================================
// text_macro.rs
// =======================================================================
// Preprocessing for macro instantiations

use crate::*;
use std::collections::HashMap;
use std::vec::IntoIter;

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
            let prev_in_text_macro_arg = state.enter_text_macro_arg();
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
    text_macro: (&'s str, Span<'s>),
) -> Result<
    HashMap<&'s str, (Span<'s>, Vec<SpannedToken<'s>>)>,
    PreprocessorError<'s>,
> {
    if specified_args.len() > original_args.len() {
        return Err(PreprocessorError::TooManyMacroArguments {
            macro_name: text_macro.0,
            define_span: define_span.clone(),
            use_span: text_macro.1,
            expected: original_args.len(),
            found: specified_args.len(),
        });
    }
    let mut specified_args_iter = specified_args.into_iter();
    let mut resolved_args = HashMap::new();
    for (arg_name, arg_tokens) in original_args.into_iter() {
        match specified_args_iter.next() {
            Some(specified_tokens) => {
                resolved_args
                    .insert(arg_name.0, (arg_name.1, specified_tokens));
            }
            None => match arg_tokens {
                Some(default_tokens) => {
                    resolved_args
                        .insert(arg_name.0, (arg_name.1, default_tokens));
                }
                None => {
                    return Err(PreprocessorError::MissingMacroArgument {
                        define_span: define_span.clone(),
                        use_span: text_macro.1,
                        param_name: arg_name.0,
                    });
                }
            },
        }
    }
    Ok(resolved_args)
}

fn get_identifier_substitute<'a>(
    arg_name: &'a str,
    replacement_tokens: &Vec<SpannedToken<'a>>,
) -> Result<&'a str, PreprocessorError<'a>> {
    let replacement_tokens_len = replacement_tokens.len();
    if replacement_tokens_len > 1 {
        // Only currently support a max of one token
        let err_span = replacement_tokens.first().unwrap().1.clone();
        return Err(PreprocessorError::InvalidIdentifierFormation {
            param_name: arg_name,
            arg_span: err_span,
        });
    } else if replacement_tokens_len == 1 {
        let replacement_token = replacement_tokens.first().unwrap().clone();
        match replacement_token.0 {
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
            | Token::EscapedIdentifier(text) => Ok(text),
            Token::OnelineComment(_)
            | Token::BlockComment(_)
            | Token::PreprocessorIdentifier(_)
            | Token::TextMacro(_)
            | Token::StringLiteral(_)
            | Token::TripleQuoteStringLiteral(_)
            | Token::Newline => {
                return Err(PreprocessorError::InvalidIdentifierFormation {
                    param_name: arg_name,
                    arg_span: replacement_token.1,
                });
            }
            other => Ok(other.as_str()),
        }
    } else {
        Ok("")
    }
}

fn get_string_substitute<'a>(
    _id_name: &'a str,
    replacement_tokens: &Vec<SpannedToken<'a>>,
    str_to_append: &mut String,
    state: &PreprocessorState<'a>,
) -> Result<(), PreprocessorError<'a>> {
    if !replacement_tokens.is_empty() {
        let start_span = &replacement_tokens.first().unwrap().1;
        let end_span = &replacement_tokens.last().unwrap().1;
        // Guaranteed to be the same file, otherwise they'd split a preprocessor definition
        let slice = &state
            .included_files
            .get(start_span.file)
            .expect("Internal Error: File not parsed yet")
            [start_span.bytes.start..end_span.bytes.end];
        *str_to_append += slice;
    }
    Ok(())
}

fn get_replacement_string<'a>(
    mut initial_string: String,
    state: &PreprocessorState<'a>,
    arguments: &HashMap<&'a str, (Span<'a>, Vec<SpannedToken<'a>>)>,
) -> Result<String, PreprocessorError<'a>> {
    // Replace definitions
    for define in &state.defines {
        let define_id = format! {"`{}", define.name.0};
        if initial_string.contains(&define_id) {
            let replacement_string = match &define.body {
                DefineBody::Empty => "".to_string(),
                DefineBody::Text(tokens) => {
                    let mut token_str = "".to_string();
                    get_string_substitute(
                        define.name.0,
                        tokens,
                        &mut token_str,
                        state,
                    )?;
                    token_str
                }
                DefineBody::Function(_) => {
                    todo!("Error for using functions here")
                }
            };
            initial_string =
                initial_string.replace(&define_id, replacement_string.as_str());
        }
    }
    // Replace arguments
    for argument in arguments.keys() {
        if initial_string.contains(argument) {
            let mut replacement_string = "".to_string();
            get_string_substitute(
                argument,
                &arguments.get(argument).unwrap().1,
                &mut replacement_string,
                state,
            )?;
            initial_string =
                initial_string.replace(argument, replacement_string.as_str())
        }
    }
    Ok(initial_string)
}

fn replace_macro_tokens<'a>(
    original_stream: IntoIter<SpannedToken<'a>>,
    arguments: HashMap<&'a str, (Span<'a>, Vec<SpannedToken<'a>>)>,
    state: &mut PreprocessorState<'a>,
    cache: &'a PreprocessorCache<'a>,
) -> Result<Vec<SpannedToken<'a>>, PreprocessorError<'a>> {
    let mut result_vec = vec![];
    for token in original_stream {
        match token {
            SpannedToken(Token::SimpleIdentifier(id), id_span)
                if arguments.contains_key(id) =>
            {
                let (_, replacement_tokens) = arguments.get(id).unwrap();
                result_vec.extend(replacement_tokens.iter().map(
                    |spanned_token| {
                        SpannedToken(spanned_token.0.clone(), id_span.clone())
                    },
                ));
            }
            SpannedToken(Token::PreprocessorIdentifier(id), span) => {
                let components = id.split("``");
                let mut resulting_identifier = "".to_string();
                for component in components.into_iter() {
                    match arguments.get(component) {
                        Some((_, replacement_tokens)) => {
                            resulting_identifier += get_identifier_substitute(
                                component,
                                replacement_tokens,
                            )?;
                        }
                        None => {
                            resulting_identifier += component;
                        }
                    }
                }
                result_vec.push(SpannedToken(
                    Token::SimpleIdentifier(
                        state.retain_string(resulting_identifier, cache),
                    ),
                    span,
                ));
            }
            SpannedToken(Token::ConcatenatedTextMacro(id), span) => {
                let components = id.split("``");
                let mut resulting_identifier = "".to_string();
                for component in components.into_iter() {
                    match arguments.get(component) {
                        Some((_, replacement_tokens)) => {
                            resulting_identifier += get_identifier_substitute(
                                component,
                                replacement_tokens,
                            )?;
                        }
                        None => {
                            resulting_identifier += component;
                        }
                    }
                }
                result_vec.push(SpannedToken(
                    Token::TextMacro(
                        state.retain_string(resulting_identifier, cache),
                    ),
                    span,
                ));
            }
            SpannedToken(Token::PreprocessorStringLiteral(id), span) => {
                result_vec.push(SpannedToken(
                    Token::StringLiteral(state.retain_string(
                        get_replacement_string(
                            id.to_string(),
                            state,
                            &arguments,
                        )?,
                        cache,
                    )),
                    span,
                ));
            }
            SpannedToken(
                Token::PreprocessorTripleQuoteStringLiteral(id),
                span,
            ) => {
                result_vec.push(SpannedToken(
                    Token::TripleQuoteStringLiteral(
                        state.retain_string(
                            get_replacement_string(
                                id.to_string(),
                                state,
                                &arguments,
                            )?
                            .replace("\\\n", "\n"),
                            cache,
                        ),
                    ),
                    span,
                ));
            }
            other_token => {
                result_vec.push(other_token);
            }
        }
    }
    Ok(result_vec)
}

struct SpanReplacer<'a> {
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

/// Create a clone of a span and all is
fn insert_base_expansion<'a>(
    cache: &'a PreprocessorCache<'a>,
    span: &'a Span<'a>,
    expanded_ref: &'a Span<'a>,
) -> &'a Span<'a> {
    let mut new_span = span.clone();
    match new_span.expanded_from {
        None => {
            new_span.expanded_from = Some(expanded_ref);
        }
        Some(nested_expansion) => {
            new_span.expanded_from = Some(insert_base_expansion(
                cache,
                nested_expansion,
                expanded_ref,
            ));
        }
    };
    cache.retain_span(new_span)
}

impl<'a> SpanReplacer<'a> {
    fn new(
        text_macro_span: Span<'a>,
        tokens: IntoIter<SpannedToken<'a>>,
        cache: &'a PreprocessorCache<'a>,
    ) -> Self {
        Self {
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
                Some(prev_expansion) => insert_base_expansion(
                    self.cache,
                    prev_expansion,
                    self.cache.retain_span(original_span),
                ),
                None => self.cache.retain_span(original_span),
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

pub fn preprocess_macro<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<(), PreprocessorError<'s>> {
    match state.get_macro_tokens(text_macro.0) {
        Some((define_span, (mut token_vec, define_args))) => {
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
                    text_macro,
                )?
            } else {
                HashMap::new()
            };
            token_vec = replace_macro_tokens(
                token_vec.into_iter(),
                arguments,
                state,
                cache,
            )?;
            let token_iter =
                SpanReplacer::new(macro_span, token_vec.into_iter(), cache);
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
            Token::UnsignedNumber("1"),
            Token::UnsignedNumber("1"),
            Token::UnsignedNumber("1")
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
        vec![Token::StringLiteral("2")]
    );
    check_preprocessor!(
        "`define TEST whoops
        `define TARGET `\"\"\"This test looks `TEST`\"\"\"
        `undef TEST
        `define TEST correct
        `TARGET",
        vec![Token::TripleQuoteStringLiteral("This test looks correct")]
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
            Token::UnsignedNumber("1"),
            Token::Plus,
            Token::UnsignedNumber("2"),
            Token::UnsignedNumber("3"),
            Token::Plus,
            Token::UnsignedNumber("4"),
        ]
    )
}

#[test]
fn nested_function() {
    check_preprocessor!(
        "`define TOP(a,b) a + b
        `TOP( `TOP(b,1), `TOP(42,a) )",
        vec![
            Token::SimpleIdentifier("b"),
            Token::Plus,
            Token::UnsignedNumber("1"),
            Token::Plus,
            Token::UnsignedNumber("42"),
            Token::Plus,
            Token::SimpleIdentifier("a")
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
        vec![Token::StringLiteral("2 = 2")]
    );
    check_preprocessor!(
        "`define TEST whoops
        `define TARGET(test_name) `\"\"\"This test_name looks `TEST`\"\"\"
        `undef TEST
        `define TEST correct
        `TARGET(basic_test)",
        vec![Token::TripleQuoteStringLiteral(
            "This basic_test looks correct"
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
            Token::UnsignedNumber("6"),
            Token::Plus,
            Token::UnsignedNumber("7"),
            Token::UnsignedNumber("32"),
            Token::Plus,
            Token::UnsignedNumber("2")
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
