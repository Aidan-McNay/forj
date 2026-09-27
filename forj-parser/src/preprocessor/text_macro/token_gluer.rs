// =======================================================================
// token_gluer.rs
// =======================================================================
// Glue together macro tokens delimited by ``

use crate::*;
use bstr::BString;
use logos::Logos;
use std::ops::Range;

/// An iterator to glue together macro tokens delimited by ``
pub(crate) struct TokenGluer<'a, T: std::iter::Iterator> {
    prev_token: Option<SpannedToken<'a>>,
    inner_iter: std::iter::Peekable<T>,
    cache: &'a PreprocessorCache<'a>,
}

/// Whether `span1` comes right before `span2`
///
/// Here, we don't have to worry about expansions/inclusions, as
/// none of that has happened yet
const fn adjacent<'a>(span1: &Span<'a>, span2: &Span<'a>) -> bool {
    span1.bytes.end == span2.bytes.start
}

fn block_comment_error<'a>(
    block_comment_start_span: Span<'a>,
) -> PreprocessorError<'a> {
    PreprocessorError::VerboseError {
        err: VerboseError {
            span: block_comment_start_span,
            found: None,
            reason: VerboseErrorReason::Diagnostic("a block comment end"),
        },
    }
}

impl<'a, T> TokenGluer<'a, T>
where
    T: Iterator<Item = SpannedToken<'a>>,
{
    /// Wrap a token iterator in a [`TokenGluer`]
    pub(crate) fn new(iter: T, cache: &'a PreprocessorCache<'a>) -> Self {
        Self {
            prev_token: None,
            inner_iter: iter.peekable(),
            cache,
        }
    }

    fn take_block_comment(
        &mut self,
        block_comment_start_span: Span<'a>,
    ) -> Result<SpannedToken<'a>, PreprocessorError<'a>> {
        let mut prev_token_end = block_comment_start_span.bytes.end;
        let mut curr_content = BString::new(vec![]);
        let comment_end = 'outer: loop {
            match self.inner_iter.next() {
                None => {
                    return Err(block_comment_error(block_comment_start_span));
                }
                Some(SpannedToken(Token::Star, star_span)) => 'inner: loop {
                    for _ in prev_token_end..star_span.bytes.start {
                        curr_content.push(b' ');
                    }
                    prev_token_end = star_span.bytes.end;
                    match self.inner_iter.next() {
                        Some(SpannedToken(Token::Slash, slash_span)) => {
                            break 'outer slash_span.bytes.end;
                        }
                        Some(SpannedToken(Token::MacroConcatenate, _)) => (),
                        Some(SpannedToken(other_token, other_span)) => {
                            curr_content.push(b'*');
                            for _ in prev_token_end..other_span.bytes.start {
                                curr_content.push(b' ');
                            }
                            curr_content.extend_from_slice(
                                text_macro::into_text(&other_token).as_ref(),
                            );
                            prev_token_end = other_span.bytes.end;
                            break 'inner;
                        }
                        None => {
                            return Err(block_comment_error(
                                block_comment_start_span,
                            ));
                        }
                    }
                },
                Some(SpannedToken(other_token, other_span)) => {
                    for _ in prev_token_end..other_span.bytes.start {
                        curr_content.push(b' ');
                    }
                    curr_content.extend_from_slice(
                        text_macro::into_text(&other_token).as_ref(),
                    );
                    prev_token_end = other_span.bytes.end;
                }
            }
        };
        let span = Span {
            bytes: Range {
                start: block_comment_start_span.bytes.start,
                end: comment_end,
            },
            ..block_comment_start_span
        };
        Ok(SpannedToken(
            Token::BlockComment(
                self.cache.retain_bytes(curr_content.into()).into(),
            ),
            span,
        ))
    }

    fn take_oneline_comment(
        &mut self,
        oneline_comment_start_span: Span<'a>,
    ) -> Result<SpannedToken<'a>, PreprocessorError<'a>> {
        // Ends are trimmed to the last non-newline token
        let mut prev_token_end = oneline_comment_start_span.bytes.end;
        let mut curr_content = BString::new(vec![]);
        let span_end = loop {
            match self.inner_iter.peek() {
                Some(SpannedToken(Token::Newline, _)) | None => {
                    break prev_token_end;
                }
                Some(_) => {
                    let SpannedToken(other_token, other_span) =
                        self.inner_iter.next().unwrap();
                    for _ in prev_token_end..other_span.bytes.start {
                        curr_content.push(b' ');
                    }
                    curr_content.extend_from_slice(
                        text_macro::into_text(&other_token).as_ref(),
                    );
                    prev_token_end = other_span.bytes.end;
                }
            }
        };
        let span = Span {
            bytes: Range {
                start: oneline_comment_start_span.bytes.start,
                end: span_end,
            },
            ..oneline_comment_start_span
        };
        Ok(SpannedToken(
            Token::OnelineComment(
                self.cache.retain_bytes(curr_content.into()).into(),
            ),
            span,
        ))
    }

    /// Consume from the inner iterator until the concatenation stops
    fn take_concatenate(
        &mut self,
        mut curr_span: Span<'a>,
        mut curr_id: BString,
    ) -> Result<SpannedToken<'a>, PreprocessorError<'a>> {
        loop {
            match self.inner_iter.peek() {
                None => {
                    break;
                }
                Some(concat_token) => {
                    if !adjacent(&curr_span, &concat_token.1) {
                        break;
                    }
                    let concat_token = self.inner_iter.next().unwrap();
                    curr_span.bytes.end = concat_token.1.bytes.end;
                    curr_id.extend_from_slice(
                        text_macro::into_text(&concat_token.0).as_ref(),
                    );
                    match self.inner_iter.peek() {
                        None => {
                            break;
                        }
                        Some(next_token) => {
                            if next_token.0 != Token::MacroConcatenate {
                                break;
                            }
                            // Consume concatenation token
                            let next_token = self.inner_iter.next().unwrap();
                            curr_span.bytes.end = next_token.1.bytes.end;
                        }
                    }
                }
            }
        }
        // Re-lex, handling comments explicitly
        if curr_id == b"/*" {
            self.take_block_comment(curr_span)
        } else if curr_id == b"//" {
            self.take_oneline_comment(curr_span)
        } else {
            let bytes = self.cache.retain_bytes(curr_id.into());
            let tokens = Token::lexer(bytes).collect::<Vec<_>>();
            match tokens[..] {
                [Ok(result_token)] => Ok(SpannedToken(result_token, curr_span)),
                _ => Ok(SpannedToken(
                    Token::InvalidConcatenation(bytes.into()),
                    curr_span,
                )),
            }
        }
    }

    /// Perform token concatenation, returning the next token to be yielded
    /// (either the previous token if not part of the concatenation, or the
    /// concatenated token)
    fn concatenate_tokens(
        &mut self,
        curr_token: Option<SpannedToken<'a>>,
        concatenate_span: Span<'a>,
    ) -> Result<SpannedToken<'a>, PreprocessorError<'a>> {
        let mut overall_span = concatenate_span;
        let mut overall_id = BString::new(vec![]);
        if let Some(curr_token) = curr_token {
            if adjacent(&curr_token.1, &overall_span) {
                // The current token is part of the concatenation
                overall_span.bytes.start = curr_token.1.bytes.start;
                overall_id = text_macro::into_text(&curr_token.0);
                self.take_concatenate(overall_span, overall_id)
            } else {
                match self.take_concatenate(overall_span, overall_id) {
                    Ok(next_token) => {
                        self.prev_token = Some(next_token);
                        Ok(curr_token)
                    }
                    Err(err) => {
                        // Keep the curr_token for next time
                        self.prev_token = Some(curr_token);
                        Err(err)
                    }
                }
            }
        } else {
            self.take_concatenate(overall_span, overall_id)
        }
    }
}

impl<'a, T> Iterator for TokenGluer<'a, T>
where
    T: Iterator<Item = SpannedToken<'a>>,
{
    type Item = Result<SpannedToken<'a>, PreprocessorError<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.prev_token.take(), self.inner_iter.next()) {
                (None, None) => {
                    break None;
                }
                (None, Some(next_token)) => {
                    if next_token.0 != Token::MacroConcatenate {
                        self.prev_token.replace(next_token);
                    } else {
                        break Some(
                            self.concatenate_tokens(None, next_token.1),
                        );
                    }
                }
                (Some(curr_token), None) => break Some(Ok(curr_token)),
                (Some(curr_token), Some(next_token)) => {
                    if next_token.0 != Token::MacroConcatenate {
                        self.prev_token = Some(next_token);
                        break Some(Ok(curr_token));
                    } else {
                        break Some(self.concatenate_tokens(
                            Some(curr_token),
                            next_token.1,
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
fn test_token_gluer<'a>(src: &[u8], expected: Vec<Token<'a>>) {
    let mut state = PreprocessorState::new(vec![], vec![]);
    let cache = PreprocessorCache::new();
    state.retain_file("test.v".to_string(), src.to_vec(), &cache);
    let tokens = lex(src, "test.v").tokens();
    let glued_tokens = TokenGluer::new(tokens, &cache)
        .into_iter()
        .map(|a| a.unwrap().0)
        .collect::<Vec<_>>();
    assert_eq!(glued_tokens, expected);
}

#[test]
fn basic() {
    let source = b"a``_``b";
    test_token_gluer(source, vec![Token::SimpleIdentifier("a_b".into())]);
}

#[test]
fn leading() {
    let source = b"  ``baba``_``is``_you  ";
    test_token_gluer(
        source,
        vec![Token::SimpleIdentifier("baba_is_you".into())],
    );
}

#[test]
fn trailing() {
    let source = b"  hello``_world``  ";
    test_token_gluer(
        source,
        vec![Token::SimpleIdentifier("hello_world".into())],
    );
}

#[test]
fn non_id() {
    let source = b"  `NEW``MACRO  ";
    test_token_gluer(source, vec![Token::TextMacro("NEWMACRO".into())]);
}

#[test]
fn non_concat_tokens() {
    let source = b"  id1 ``id``2`` id3  ";
    test_token_gluer(
        source,
        vec![
            Token::SimpleIdentifier("id1".into()),
            Token::SimpleIdentifier("id2".into()),
            Token::SimpleIdentifier("id3".into()),
        ],
    );
}

#[test]
fn invalid() {
    let source = b"  plus``is``+  ";
    test_token_gluer(
        source,
        vec![Token::InvalidConcatenation("plusis+".into())],
    );
}

#[test]
fn oneline_comment() {
    let source = b"/``/ This is a comment
    // This is a lexed comment";
    test_token_gluer(
        source,
        vec![
            Token::OnelineComment(" This is a comment".into()),
            Token::Newline,
            Token::OnelineComment(" This is a lexed comment".into()),
        ],
    );
}

#[test]
fn oneline_comment_end() {
    let source = b"/``/ This is a comment";
    test_token_gluer(
        source,
        vec![Token::OnelineComment(" This is a comment".into())],
    );
}

#[test]
fn empty_oneline_comment() {
    let source = b"/``/";
    test_token_gluer(source, vec![Token::OnelineComment("".into())]);
}

#[test]
fn block_comment() {
    let source = b"/``* This is a block comment *``/";
    test_token_gluer(
        source,
        vec![Token::BlockComment(" This is a block comment ".into())],
    );
}

#[test]
fn block_comment_normal_end() {
    let source = b"/``* This is a block comment */";
    test_token_gluer(
        source,
        vec![Token::BlockComment(" This is a block comment ".into())],
    );
}

#[test]
fn block_comment_containing_partial_end() {
    let source = b"/``* A termination uses * and / *``/";
    test_token_gluer(
        source,
        vec![Token::BlockComment(" A termination uses * and / ".into())],
    );
}

#[test]
#[should_panic(expected = "a block comment end")]
fn incomplete_block_commennt() {
    let source = b"/``* Incomplete";
    test_token_gluer(source, vec![Token::BlockComment(" Incomplete".into())]);
}

#[test]
#[should_panic(expected = "a block comment end")]
fn partial_incomplete_block_commennt() {
    let source = b"/``* Incomplete *";
    test_token_gluer(source, vec![Token::BlockComment(" Incomplete ".into())]);
}

#[test]
fn macros() {
    let source = b" `PREFIX``_```SUFFIX  ";
    test_token_gluer(
        source,
        vec![Token::InvalidConcatenation("`PREFIX_`SUFFIX".into())],
    );
}
