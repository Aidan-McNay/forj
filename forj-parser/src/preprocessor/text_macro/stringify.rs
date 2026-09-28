// =======================================================================
// stringify.rs
// =======================================================================
// Replace preprocessor strings with arguments and macro expansions

use crate::{
    text_macro::{ArgReplace, TokenGluer, resolve_text_macro_args},
    *,
};
use bstr::{BStr, BString, ByteSlice};
use std::collections::HashMap;

pub(crate) struct Stringify<'a: 'b, 'b, T> {
    args: &'b HashMap<&'a BStr, (Span<'a>, Vec<SpannedToken<'a>>)>,
    state: &'b PreprocessorState<'a>,
    cache: &'a PreprocessorCache<'a>,
    expanding_macros: Vec<&'a BStr>,
    inner_iter: T,
}

fn string_substitute<'a>(
    replacement_tokens: &Vec<SpannedToken<'a>>,
) -> BString {
    if replacement_tokens.is_empty() {
        BString::new(vec![])
    } else {
        // Can't just take source slice, in case concatenation has happened
        let mut prev_token_end =
            replacement_tokens.first().unwrap().1.bytes.start;
        let mut content = BString::new(vec![]);
        for token in replacement_tokens {
            for _ in prev_token_end..token.1.bytes.start {
                content.push(b' ');
            }
            content.extend_from_slice(text_macro::into_text(&token.0).as_ref());
            prev_token_end = token.1.bytes.end;
        }
        content
    }
}

impl<'a, 'b, T> Stringify<'a, 'b, T>
where
    T: Iterator<Item = SpannedToken<'a>>,
{
    pub fn new(
        iter: T,
        state: &'b PreprocessorState<'a>,
        cache: &'a PreprocessorCache<'a>,
        expanding_macros: Vec<&'a BStr>,
        args: &'b HashMap<&'a BStr, (Span<'a>, Vec<SpannedToken<'a>>)>,
    ) -> Self {
        Self {
            args,
            state,
            cache,
            expanding_macros,
            inner_iter: iter,
        }
    }

    fn replace_args(&self, mut pp_string: BString) -> BString {
        for argument in self.args.keys() {
            if pp_string.contains_str(argument) {
                pp_string = pp_string
                    .replace(
                        argument,
                        string_substitute(&self.args.get(argument).unwrap().1),
                    )
                    .into();
            }
        }
        pp_string
    }

    fn get_stringify_macro_args(
        &self,
        start_idx: usize,
        arg_slice: &[u8],
    ) -> Result<(Vec<Vec<SpannedToken<'a>>>, usize), ()> {
        let mut byte_iter = arg_slice.iter().enumerate();
        let Some((_, b'(')) = byte_iter.next() else {
            return Err(());
        };
        let mut arg_vec: Vec<Vec<SpannedToken<'a>>> = vec![];
        let mut paren_depth: usize = 0;
        let mut arg_start: usize = 1;
        let eparen_idx = loop {
            match byte_iter.next() {
                None => {
                    return Err(());
                }
                Some((_, b'(')) => {
                    paren_depth += 1;
                }
                Some((eparen_idx, b')')) => {
                    if paren_depth == 0 {
                        let sliced_text = &arg_slice[arg_start..eparen_idx];
                        arg_vec.push(vec![SpannedToken(
                            Token::SimpleIdentifier(
                                self.cache
                                    .retain_bytes(sliced_text.trim().to_vec())
                                    .into(),
                            ),
                            Span::default(),
                        )]);
                        break eparen_idx;
                    } else {
                        paren_depth -= 1;
                    }
                }
                Some((comma_idx, b',')) => {
                    let sliced_text = &arg_slice[arg_start..comma_idx];
                    arg_vec.push(vec![SpannedToken(
                        Token::SimpleIdentifier(
                            self.cache
                                .retain_bytes(sliced_text.trim().to_vec())
                                .into(),
                        ),
                        Span::default(),
                    )]);
                    arg_start = comma_idx + 1;
                }
                _ => (),
            }
        };
        Ok((arg_vec, start_idx + eparen_idx + 1))
    }

    // This is gonna get kinda yucky
    // We need to parse the arguments from the string,
    // expand the macro, then re-insert it into the string. Any failure
    // should result in the string not being replaced.
    //
    // We could try to replicate the preprocessor macro expansion here,
    // but it's not great for maintainability. Default [`Span`]s are used
    // here after careful consideration that they're inconsequential (since
    // this is all just ending up in a string at the end, and failures are silent)
    fn replace_macro_function(
        &self,
        define_id: BString,
        func: &DefineFunction<'a>,
        already_replaced: Vec<&'a BStr>,
        mut curr_string: BString,
    ) -> BString {
        let start_idx = curr_string.find(&define_id).unwrap();
        let end_of_id = start_idx + define_id.len();
        let arg_slice = &curr_string[end_of_id..];
        let Ok((macro_args, end_idx)) =
            self.get_stringify_macro_args(end_of_id, arg_slice)
        else {
            return curr_string;
        };
        let Ok(resolved_args) = resolve_text_macro_args(
            macro_args,
            func.args
                .iter()
                .map(|(a, b)| match b {
                    Some((_, tokens)) => (a.clone(), Some(tokens.clone())),
                    None => (a.clone(), None),
                })
                .collect(),
            &Span::default(),
            &("", Span::default()),
        ) else {
            return curr_string;
        };
        let Ok(token_result_vec) = TokenGluer::new(
            Stringify::new(
                ArgReplace::new(
                    match &func.body {
                        Some(tokens) => tokens.clone().into_iter(),
                        None => vec![].into_iter(),
                    },
                    &resolved_args,
                ),
                self.state,
                self.cache,
                already_replaced.clone(),
                &resolved_args,
            ),
            self.cache,
        )
        .collect::<Result<Vec<_>, _>>() else {
            return curr_string;
        };
        let replacement_string = string_substitute(&token_result_vec);
        curr_string.splice(
            start_idx..end_idx,
            Into::<Vec<u8>>::into(replacement_string).into_iter(),
        );
        self.replace_macros(curr_string, already_replaced)
    }

    fn replace_macros(
        &self,
        mut pp_string: BString,
        already_replaced: Vec<&'a BStr>,
    ) -> BString {
        for define in &self.state.defines {
            if already_replaced.contains(&(define.name.0.into())) {
                // Avoid self-referential expansion
                continue;
            }
            let mut define_id = BString::new(b"`".to_vec());
            define_id.extend_from_slice(define.name.0.as_bytes());
            if pp_string.contains_str(&define_id) {
                match &define.body {
                    DefineBody::Empty => {
                        pp_string = pp_string.replace(define_id, b"").into();
                    }
                    DefineBody::Text(tokens) => {
                        let mut new_already_replaced = already_replaced.clone();
                        new_already_replaced.push(define.name.0.into());
                        match TokenGluer::new(
                            Stringify::new(
                                tokens.clone().into_iter(),
                                self.state,
                                self.cache,
                                new_already_replaced.clone(),
                                &HashMap::new(),
                            ),
                            self.cache,
                        )
                        .collect::<Result<Vec<_>, _>>()
                        {
                            Ok(pp_tokens) => {
                                pp_string = pp_string
                                    .replace(
                                        define_id,
                                        string_substitute(&pp_tokens),
                                    )
                                    .into();
                            }
                            Err(_) => {
                                // Errors on comments - avoid substitution for now
                                pp_string = pp_string
                                    .replace(
                                        define_id,
                                        string_substitute(tokens),
                                    )
                                    .into();
                            }
                        };
                        pp_string = self
                            .replace_macros(pp_string, new_already_replaced);
                    }
                    DefineBody::Function(def_function) => {
                        let mut new_already_replaced = already_replaced.clone();
                        new_already_replaced.push(define.name.0.into());
                        pp_string = self.replace_macro_function(
                            define_id,
                            def_function,
                            new_already_replaced,
                            pp_string,
                        );
                    }
                }
            }
        }
        pp_string
    }

    fn replace_escape_sequence(&self, pp_string: BString) -> BString {
        pp_string.replace(b"`\\`\"", b"\\\"").into()
    }

    fn replace_string(
        &self,
        pp_string: &BStr,
        is_triple_quote: bool,
    ) -> &'a BStr {
        let owned_string = pp_string.to_owned();
        let mut replaced_string =
            self.replace_escape_sequence(self.replace_macros(
                self.replace_args(owned_string),
                self.expanding_macros.clone(),
            ));
        if is_triple_quote {
            replaced_string = replaced_string.replace(b"\\\n", b"\n").into();
        }
        self.cache.retain_bytes(replaced_string.into()).into()
    }
}

impl<'a, 'b, T> Iterator for Stringify<'a, 'b, T>
where
    T: Iterator<Item = SpannedToken<'a>>,
{
    type Item = SpannedToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_iter.next() {
            Some(SpannedToken(
                Token::PreprocessorStringLiteral(pp_string),
                pp_string_span,
            )) => Some(SpannedToken(
                Token::StringLiteral(self.replace_string(pp_string, false)),
                pp_string_span,
            )),
            Some(SpannedToken(
                Token::PreprocessorTripleQuoteStringLiteral(pp_string),
                pp_string_span,
            )) => Some(SpannedToken(
                Token::TripleQuoteStringLiteral(
                    self.replace_string(pp_string, true),
                ),
                pp_string_span,
            )),
            other => other,
        }
    }
}
