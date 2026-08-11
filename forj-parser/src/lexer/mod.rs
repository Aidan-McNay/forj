// =======================================================================
// mod.rs
// =======================================================================
//! Lexing a source file into semantic tokens

pub(crate) mod callbacks;
pub(crate) mod keywords;
pub(crate) mod tokens;
use crate::SpannedToken;
use crate::report::{Report, ReportKind};
pub use keywords::StandardVersion;
use logos::Logos;
use logos::Span as ByteSpan;
use forj_syntax::Span;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;
pub use tokens::Token;

/// A single result from the lexer
///
/// This is either a valid token, or a (possible) explanation
/// of what went wrong, if identifiable, along with the associated
/// [`Span`]
pub type LexerResult<'a> = (Result<Token<'a>, String>, Span<'a>);

impl<'a> TryFrom<&LexerResult<'a>> for Report {
    type Error = ();
    fn try_from(
        value: &(Result<Token<'a>, String>, Span<'a>),
    ) -> Result<Self, Self::Error> {
        let (result, span) = value;
        let Err(text) = result else {
            return Err(());
        };
        if text.len() == 0 {
            Ok(
                Report::new(
                    ReportKind::Error,
                    span,
                    "L1",
                    "Unrecognized token",
                )
                .with_label(
                    &span,
                    ReportKind::Error,
                    "Unrecognized token",
                ),
            )
        } else {
            Ok(Report::new(ReportKind::Error, span, "L2", text).with_label(
                &span,
                ReportKind::Error,
                text,
            ))
        }
    }
}

fn map_lex_result<'a>(lex_result: LexerResult<'a>) -> SpannedToken<'a> {
    match lex_result.0 {
        Ok(tok) => SpannedToken(tok, lex_result.1),
        Err(_) => SpannedToken(Token::Error, lex_result.1),
    }
}

/// An iterator over syntactical tokens for a SystemVerilog source
pub trait LexedSource<'a>: Iterator<Item = LexerResult<'a>> + Clone {
    /// Generate error reports for any errors encountered in lexing
    fn report_errors(&self) -> impl Iterator<Item = Report> {
        self.clone().into_iter().filter_map(|result| {
            let report_result: Result<Report, ()> = Report::try_from(&result);
            report_result.ok()
        })
    }

    /// Dump a representation of the lexed source to a file, for debugging
    fn dump(&self, file_path: &Path) -> io::Result<()> {
        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);
        for (result, span) in self.clone() {
            let dump_str = format!(
                "[{:>2}:{:>2}] {}\n",
                span.bytes.start,
                span.bytes.end,
                match result {
                    Ok(token) => token,
                    Err(_) => Token::Error,
                }
            );
            writer.write_all(dump_str.as_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Process the lexing of the source, storing the result
    ///
    /// While this does incur memory overhead, it avoid processing
    /// the source multiple times if cloned.
    fn process(self) -> std::vec::IntoIter<LexerResult<'a>> {
        self.collect::<Vec<_>>().into_iter()
    }

    /// Convert lexer results into tokens, turning errors into [`Token::Error`]
    fn tokens(self) -> impl Iterator<Item = SpannedToken<'a>> {
        self.into_iter().map(map_lex_result)
    }
}
impl<'a, T> LexedSource<'a> for T where
    T: Iterator<Item = LexerResult<'a>> + Clone
{
}

fn token_span_mapper<'a>(
    file_name: &'a str,
    included_from: Option<&'a Span<'a>>,
) -> impl Fn((Result<Token<'a>, String>, ByteSpan)) -> LexerResult<'a> + Clone {
    move |(token_result, byte_span)| {
        (
            token_result,
            Span {
                file: file_name,
                bytes: byte_span,
                expanded_from: None,
                included_from,
            },
        )
    }
}

pub(crate) fn lex_helper<'a>(
    src: &'a str,
    file_name: &'a str,
    included_from: Option<&'a Span<'a>>,
) -> impl LexedSource<'a> {
    let span_mapper = token_span_mapper(file_name, included_from);
    Token::lexer(src).spanned().map(span_mapper)
}

/// Separate a source file into syntactic tokens
///
/// ```rust
/// # use forj_parser::*;
/// let file_contents = "module test_module; endmodule";
/// let mut tokens = lex(file_contents, "test_file.v");
/// assert!(matches!(tokens.next().unwrap(), (Ok(Token::Module), _)));
/// assert!(matches!(tokens.next().unwrap(), (Ok(Token::SimpleIdentifier("test_module")), _)));
/// assert!(matches!(tokens.next().unwrap(), (Ok(Token::SColon), _)));
/// assert!(matches!(tokens.next().unwrap(), (Ok(Token::Endmodule), _)));
/// assert!(tokens.next().is_none());
/// ```
///
/// If the lexer encounters an error, the resulting `Err(string)` may contain
/// more information if possible to discern.
pub fn lex<'a>(src: &'a str, file_name: &'a str) -> impl LexedSource<'a> {
    lex_helper(src, file_name, None)
}
