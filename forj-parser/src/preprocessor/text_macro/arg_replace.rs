// =======================================================================
// arg_replace.rs
// =======================================================================
// Replace the arguments in a preprocessor macro with their values

use crate::*;
use bstr::BStr;
use std::collections::HashMap;
use std::vec::IntoIter;

/// An iterator to glue together preprocessor macro arguments
pub(crate) struct ArgReplace<'a: 'b, 'b, T> {
    args: &'b HashMap<&'a BStr, (Span<'a>, Vec<SpannedToken<'a>>)>,
    expanded_tokens: IntoIter<SpannedToken<'a>>,
    inner_iter: T,
}

impl<'a, 'b, T> ArgReplace<'a, 'b, T>
where
    T: Iterator<Item = SpannedToken<'a>>,
{
    pub fn new(
        iter: T,
        args: &'b HashMap<&'a BStr, (Span<'a>, Vec<SpannedToken<'a>>)>,
    ) -> Self {
        Self {
            args,
            expanded_tokens: vec![].into_iter(),
            inner_iter: iter,
        }
    }
}

impl<'a, 'b, T> Iterator for ArgReplace<'a, 'b, T>
where
    T: Iterator<Item = SpannedToken<'a>>,
{
    type Item = SpannedToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.expanded_tokens.next() {
                Some(token) => {
                    break Some(token);
                }
                None => match self.inner_iter.next() {
                    Some(SpannedToken(
                        Token::SimpleIdentifier(id),
                        id_span,
                    )) if let Some((_, replacement_tokens)) =
                        self.args.get(id) =>
                    {
                        self.expanded_tokens = replacement_tokens
                            .iter()
                            .map(|spanned_token| {
                                SpannedToken(
                                    spanned_token.0.clone(),
                                    id_span.clone(),
                                )
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                    }
                    other => {
                        break other;
                    }
                },
            }
        }
    }
}

#[cfg(test)]
fn test_token_gluer<'a>(
    src: &[u8],
    args: HashMap<&'a BStr, Vec<Token<'a>>>,
    expected: Vec<Token<'a>>,
) {
    let mut state = PreprocessorState::new(vec![], vec![]);
    let cache = PreprocessorCache::new();
    state.retain_file("test.v".to_string(), src.to_vec(), &cache);
    let tokens = lex(src, "test.v").tokens();
    let args = args
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                (
                    Span::default(),
                    v.into_iter()
                        .map(|t| SpannedToken(t, Span::default()))
                        .collect(),
                ),
            )
        })
        .collect();
    let replaced_args = ArgReplace::new(tokens, &args)
        .into_iter()
        .map(|a| a.0)
        .collect::<Vec<_>>();
    assert_eq!(replaced_args, expected);
}

#[test]
fn basic() {
    let source = b"a + b";
    let args = vec![
        ("a".into(), vec![Token::UnsignedNumber("1".into())]),
        ("b".into(), vec![Token::UnsignedNumber("2".into())]),
    ]
    .into_iter()
    .collect();
    test_token_gluer(
        source,
        args,
        vec![
            Token::UnsignedNumber("1".into()),
            Token::Plus,
            Token::UnsignedNumber("2".into()),
        ],
    )
}

#[test]
fn no_token() {
    let source = b"first second third";
    let args = vec![
        (
            "first".into(),
            vec![Token::SimpleIdentifier("my_first".into())],
        ),
        ("second".into(), vec![]),
        (
            "third".into(),
            vec![Token::SimpleIdentifier("my_third".into())],
        ),
    ]
    .into_iter()
    .collect();
    test_token_gluer(
        source,
        args,
        vec![
            Token::SimpleIdentifier("my_first".into()),
            Token::SimpleIdentifier("my_third".into()),
        ],
    )
}

#[test]
fn multi_token() {
    let source = b"math complex";
    let args = vec![
        (
            "complex".into(),
            vec![
                Token::SimpleIdentifier("this".into()),
                Token::SimpleIdentifier("is".into()),
                Token::SimpleIdentifier("multiple".into()),
            ],
        ),
        (
            "math".into(),
            vec![
                Token::UnsignedNumber("4".into()),
                Token::Slash,
                Token::UnsignedNumber("2".into()),
            ],
        ),
    ]
    .into_iter()
    .collect();
    test_token_gluer(
        source,
        args,
        vec![
            Token::UnsignedNumber("4".into()),
            Token::Slash,
            Token::UnsignedNumber("2".into()),
            Token::SimpleIdentifier("this".into()),
            Token::SimpleIdentifier("is".into()),
            Token::SimpleIdentifier("multiple".into()),
        ],
    )
}
