// =======================================================================
// lib.rs
// =======================================================================
//! A SystemVerilog preprocessor and parser
//!
//! `forj-parser` provides capabilities for transformting a SystemVerilog
//! source file into a CST compliant with IEEE 1800-2023, with an
//! emphasis on informative error messages. It can be used as the
//! front-end for other tools looking to interpret SystemVerilog designs.
//!
//! ## Features
//!
//!  - `lossless`: Equivalent to the `lossless` feature for [`forj_syntax`].
//!    Produces a CST with room for non-trivia nodes, but does not actually
//!    parse any from provided sources
//!  - `parse_lossless`: Extends `lossless` to parse non-trivia tokens.
//!    Due to their arbitrary position in source files, this adds a
//!    measurable performance decrease, and should only be used if
//!    newlines/comments are needed.

mod error;
pub mod lexer;
pub mod parser;
pub mod preprocessor;
pub mod report;
use ariadne::{Color, Label};
pub use error::*;
use lexer::*;
pub use lexer::{LexedSource, Token, lex};
pub use parser::parse;
use parser::*;
pub(crate) use preprocessor::*;
pub use preprocessor::{
    PreprocessorCache, PreprocessorError, PreprocessorState, preprocess,
};
use winnow::Parser;
use winnow::stream::TokenSlice;
#[cfg(test)]
pub mod test;
pub use forj_syntax::Span;
#[cfg(test)]
pub use test::*;

/// A string and its associated [`Span`] in the source files
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpannedString<'a>(pub &'a str, pub Span<'a>);

/// A token and its location in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedToken<'s>(pub Token<'s>, pub Span<'s>);
impl<'s> PartialEq<Token<'s>> for SpannedToken<'s> {
    fn eq(&self, other: &Token) -> bool {
        self.0 == *other
    }
}
impl<'s> From<(Token<'s>, Span<'s>)> for SpannedToken<'s> {
    fn from(item: (Token<'s>, Span<'s>)) -> Self {
        (item.0, item.1).into()
    }
}
