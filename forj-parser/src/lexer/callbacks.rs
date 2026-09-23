// =======================================================================
// callbacks.rs
// =======================================================================
// The callbacks used to lex a SystemVerilog source

use crate::*;
use bstr::BStr;
use logos::{Lexer, Logos};

#[derive(Logos)]
#[logos(utf8 = false)]
enum StringToken {
    #[token(r#"""#)]
    Delimeter,
    #[token(r#"\""#, priority = 4)]
    EscapedDelimeter,
    #[token("\n")]
    #[token("\r")]
    #[token("\r\n")]
    #[token("\u{0085}")]
    #[token("\u{2028}")]
    #[token("\u{2029}")]
    Newline,
    #[token("\\\n")]
    #[token("\\\r")]
    #[token("\\\r\n")]
    #[token("\\\u{0085}")]
    #[token("\\\u{2028}")]
    #[token("\\\u{2029}")]
    EscapedNewline,
    #[regex(br#"[^"\r\n\\]"#)]
    #[regex(br#"\\([ -~]|[0-7]{1,3})"#, priority = 2)]
    #[regex(br#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
#[logos(utf8 = false)]
enum MultilineStringToken {
    #[token(r#"""""#)]
    Delimeter,
    #[regex(br#"[^\\]"#)]
    #[regex(br#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(br#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
#[logos(utf8 = false)]
enum PreprocessorStringToken {
    #[token(r#"`""#)]
    Delimeter,
    #[token(r#"`\`""#)]
    EscapedDelimeter,
    #[token("\n")]
    #[token("\r")]
    #[token("\r\n")]
    #[token("\u{0085}")]
    #[token("\u{2028}")]
    #[token("\u{2029}")]
    Newline,
    #[token("\\\n")]
    #[token("\\\r")]
    #[token("\\\r\n")]
    #[token("\\\u{0085}")]
    #[token("\\\u{2028}")]
    #[token("\\\u{2029}")]
    EscapedNewline,
    #[regex(br#"[^\r\n\\]"#)]
    #[regex(br#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(br#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
#[logos(utf8 = false)]
enum PreprocessorMultilineStringToken {
    #[token(r#"`""""#)]
    Delimeter,
    #[token(r#"`\`""#)]
    EscapedDelimeter,
    #[token("\n")]
    #[token("\r")]
    #[token("\r\n")]
    #[token("\u{0085}")]
    #[token("\u{2028}")]
    #[token("\u{2029}")]
    Newline,
    #[token("\\\n")]
    #[token("\\\r")]
    #[token("\\\r\n")]
    #[token("\\\u{0085}")]
    #[token("\\\u{2028}")]
    #[token("\\\u{2029}")]
    EscapedNewline,
    #[regex(br#"[^\n\r\\]"#)]
    #[regex(br#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(br#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
#[logos(utf8 = false)]
enum BlockCommentToken {
    #[token("*/")]
    Delimeter,
    #[regex(br"[\s\S]")]
    Other,
}

pub fn oneline_comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a BStr> {
    lex.slice().strip_prefix(b"//").map(Into::into)
}

pub fn block_comment<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a BStr, String> {
    let start_span = lex.span();
    let mut block_comment_lexer = lex.clone().morph::<BlockCommentToken>();
    while let Some(string_token) = block_comment_lexer.next() {
        match string_token {
            Ok(BlockCommentToken::Delimeter) => {
                let end_span = block_comment_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string.into());
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = block_comment_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err("Unterminated block comment".to_string());
            }
        }
    }
    let end_span = block_comment_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated block comment".to_string())
}

pub fn string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a BStr, String> {
    let start_span = lex.span();
    let mut string_lexer = lex.clone().morph::<StringToken>();
    while let Some(string_token) = string_lexer.next() {
        match string_token {
            Ok(StringToken::Delimeter) => {
                let end_span = string_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string.into());
            }
            Ok(StringToken::Newline) => {
                let end_span = string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err("Unterminated string literal".to_string());
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err("Unterminated string literal".to_string());
            }
        }
    }
    let end_span = string_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated string literal".to_string())
}

pub fn multiline_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a BStr, String> {
    let start_span = lex.span();
    let mut multiline_string_lexer =
        lex.clone().morph::<MultilineStringToken>();
    while let Some(string_token) = multiline_string_lexer.next() {
        match string_token {
            Ok(MultilineStringToken::Delimeter) => {
                let end_span = multiline_string_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string.into());
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = multiline_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err("Unterminated multiline string literal".to_string());
            }
        }
    }
    let end_span = multiline_string_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated multiline string literal".to_string())
}

pub fn preprocessor_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a BStr, String> {
    let start_span = lex.span();
    let mut preprocessor_string_lexer =
        lex.clone().morph::<PreprocessorStringToken>();
    while let Some(string_token) = preprocessor_string_lexer.next() {
        match string_token {
            Ok(PreprocessorStringToken::Delimeter) => {
                let end_span = preprocessor_string_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string.into());
            }
            Ok(PreprocessorStringToken::Newline) => {
                let end_span = preprocessor_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err(
                    "Unterminated preprocessor string literal".to_string()
                );
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = preprocessor_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err(
                    "Unterminated preprocessor string literal".to_string()
                );
            }
        }
    }
    let end_span = preprocessor_string_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated preprocessor string literal".to_string())
}

pub fn preprocessor_multiline_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a BStr, String> {
    let start_span = lex.span();
    let mut preprocessor_string_lexer =
        lex.clone().morph::<PreprocessorMultilineStringToken>();
    while let Some(string_token) = preprocessor_string_lexer.next() {
        match string_token {
            Ok(PreprocessorMultilineStringToken::Delimeter) => {
                let end_span = preprocessor_string_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string.into());
            }
            Ok(PreprocessorMultilineStringToken::Newline) => {
                let end_span = preprocessor_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err(
                    "Unterminated preprocessor multiline string literal"
                        .to_string(),
                );
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = preprocessor_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err(
                    "Unterminated preprocessor multiline string literal"
                        .to_string(),
                );
            }
        }
    }
    let end_span = preprocessor_string_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated preprocessor multiline string literal".to_string())
}

pub fn text_macro<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a BStr> {
    lex.slice().strip_prefix(b"`").map(Into::into)
}

#[test]
fn comments() {
    check_lexer!(
        "// This is a single-line comment
        /* This is a block comment */
        /* Block comments
        can be
        on multiple
        lines */",
        vec![
            Token::OnelineComment(b" This is a single-line comment".into()),
            Token::Newline,
            Token::BlockComment(b" This is a block comment ".into()),
            Token::Newline,
            Token::BlockComment(
                b" Block comments
        can be
        on multiple
        lines "
                    .into()
            )
        ]
    )
}

#[test]
fn string() {
    check_lexer!(
        "\" This is a string \"",
        vec![Token::StringLiteral(b" This is a string ".into())]
    );
    check_lexer!(
        "\" This is a \\\n multiline string \"",
        vec![Token::StringLiteral(
            b" This is a \\\n multiline string ".into()
        )]
    )
}

#[test]
fn multiline_string() {
    check_lexer!(
        "\"\"\"This string
        spans multiple
        lines!\"\"\"",
        vec![Token::TripleQuoteStringLiteral(
            b"This string
        spans multiple
        lines!"
                .into()
        )]
    )
}

#[test]
fn preprocessor_string() {
    check_lexer!(
        "`\" This is a preprocessor string `\"",
        vec![Token::PreprocessorStringLiteral(
            b" This is a preprocessor string ".into()
        )]
    );
    check_lexer!(
        "`\" This is a \\\n multiline preprocessor string `\"",
        vec![Token::PreprocessorStringLiteral(
            b" This is a \\\n multiline preprocessor string ".into()
        )]
    )
}

#[test]
fn preprocessor_multiline_string() {
    check_lexer!(
        "`\"\"\"This string \\\n spans multiple \\\n lines!`\"\"\"",
        vec![Token::PreprocessorTripleQuoteStringLiteral(
            b"This string \\\n spans multiple \\\n lines!".into()
        )]
    )
}

#[test]
fn text_macros() {
    check_lexer!(
        "`TEST_MACRO
        `TEST_FUNCTION(ARGA, ARGB)",
        vec![
            Token::TextMacro(b"TEST_MACRO".into()),
            Token::Newline,
            Token::TextMacro(b"TEST_FUNCTION".into()),
            Token::Paren,
            Token::SimpleIdentifier(b"ARGA".into()),
            Token::Comma,
            Token::SimpleIdentifier(b"ARGB".into()),
            Token::EParen
        ]
    )
}
