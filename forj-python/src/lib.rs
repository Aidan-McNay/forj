// =======================================================================
// lib.rs
// =======================================================================
//! Python bindings for the `forj` SystemVerilog tools
//!
//! When crossing the FFI boundary, Rust's borrow checker can no longer
//! provide lifetime guarantees; as such, many data structures must be
//! cloned, and have associated [`From`]/[`Into`] implementations for
//! their reference-based Rust counterparts. This quickly becomes the
//! dominant factor in runtime; if speed/space usage becomes a concern,
//! native Rust applications should be considered instead.

mod define;
mod error;
mod node;
mod report;
mod token;
use std::path::PathBuf;

pub use define::*;
pub use error::*;
pub use node::*;
use pyo3::prelude::*;
pub use report::*;
use forj_parser::{LexedSource, PreprocessorCache};
pub use token::*;

/// The top-level Python module
#[pymodule]
pub mod forj_python {
    #[pymodule_export]
    pub use super::lex;
    #[pymodule_export]
    pub use super::{
        Bytes, Expectation, Node, NodeIterator, Report, ReportKind, Span,
        SpannedToken, Token, VerboseError,
    };
    #[pymodule_export]
    pub use super::{Define, define_empty, define_text};
    #[pymodule_export]
    pub use super::{ParserResult, parse, parse_from_preprocess};
    #[pymodule_export]
    pub use super::{
        PreprocessorError, PreprocessorResult, preprocess, preprocess_from_lex,
    };
}

// -----------------------------------------------------------------------
// Lexing
// -----------------------------------------------------------------------

/// Separate a source file into syntactic tokens
#[pyfunction]
pub fn lex(src: String, file_name: String) -> Vec<SpannedToken> {
    forj_parser::lex(&src, &file_name)
        .tokens()
        .map(|rust_spanned_token| rust_spanned_token.into())
        .collect()
}

// -----------------------------------------------------------------------
// Preprocessing
// -----------------------------------------------------------------------

/// The result of preprocessing a SystemVerilog source
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub enum PreprocessorResult {
    Ok { tokens: Vec<SpannedToken> },
    Err { errors: Vec<PreprocessorError> },
}

/// Same as [`preprocess`], but operates on the output of [`lex`]
///
/// Comparitively, this incurs overhead from copying data between
/// Rust and Python's ownership models. Only use if you need to
/// modify the output of [`lex`] before preprocessing
#[pyfunction]
pub fn preprocess_from_lex(
    tokens: Vec<SpannedToken>,
    include_paths: Vec<PathBuf>,
    defines: Vec<crate::Define>,
) -> PreprocessorResult {
    let cache = PreprocessorCache::new();
    let rust_tokens = tokens
        .iter()
        .map(|python_token| python_token.to_rust(&cache));
    let mut state = forj_parser::PreprocessorState::new(
        include_paths
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect(),
        defines
            .iter()
            .map(|python_define| python_define.to_rust(&cache))
            .collect(),
    );
    match forj_parser::preprocess(rust_tokens, &mut state, &cache) {
        Ok(tokens) => PreprocessorResult::Ok {
            tokens: tokens
                .into_iter()
                .map(|rust_token| rust_token.into())
                .collect(),
        },
        Err(()) => PreprocessorResult::Err {
            errors: state.errors.into_iter().map(|err| err.into()).collect(),
        },
    }
}

/// Preprocess a token stream, elaborating compiler directives
#[pyfunction]
pub fn preprocess(
    src: String,
    file_name: String,
    include_paths: Vec<PathBuf>,
    defines: Vec<crate::Define>,
) -> PreprocessorResult {
    let cache = PreprocessorCache::new();
    let tokens = forj_parser::lex(&src, &file_name).tokens();
    let mut state = forj_parser::PreprocessorState::new(
        include_paths
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect(),
        defines
            .iter()
            .map(|python_define| python_define.to_rust(&cache))
            .collect(),
    );
    match forj_parser::preprocess(tokens, &mut state, &cache) {
        Ok(tokens) => PreprocessorResult::Ok {
            tokens: tokens
                .into_iter()
                .map(|rust_token| rust_token.into())
                .collect(),
        },
        Err(()) => PreprocessorResult::Err {
            errors: state.errors.into_iter().map(|err| err.into()).collect(),
        },
    }
}

// -----------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------

/// The result of parsing a SystemVerilog source
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub enum ParserResult {
    Ok { root: Node },
    ParserErr { error: VerboseError },
    PreprocessorErr { errors: Vec<PreprocessorError> },
}

/// Same as [`parse`], but operates on the output of [`preprocess`]
///
/// Comparitively, this incurs overhead from copying data between
/// Rust and Python's ownership models. Only use if you need to
/// modify the output of [`preprocess`] before parsing
#[pyfunction]
pub fn parse_from_preprocess(tokens: Vec<SpannedToken>) -> ParserResult {
    let cache = PreprocessorCache::new();
    let rust_tokens = tokens
        .iter()
        .map(|python_token| python_token.to_rust(&cache))
        .collect::<Vec<_>>();
    match forj_parser::parse(&rust_tokens) {
        Ok(result) => {
            let node: forj_syntax::Node<'_, '_> = (&result).into();
            ParserResult::Ok { root: node.into() }
        }
        Err(err) => ParserResult::ParserErr { error: err.into() },
    }
}

/// Parse the token stream into a concrete syntax tree
#[pyfunction]
pub fn parse(
    src: String,
    file_name: String,
    include_paths: Vec<PathBuf>,
    defines: Vec<crate::Define>,
) -> ParserResult {
    let cache = PreprocessorCache::new();
    let tokens = forj_parser::lex(&src, &file_name).tokens();
    let mut state = forj_parser::PreprocessorState::new(
        include_paths
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect(),
        defines
            .iter()
            .map(|python_define| python_define.to_rust(&cache))
            .collect(),
    );
    let tokens = match forj_parser::preprocess(tokens, &mut state, &cache) {
        Ok(tokens) => tokens,
        Err(()) => {
            return ParserResult::PreprocessorErr {
                errors: state
                    .errors
                    .into_iter()
                    .map(|err| err.into())
                    .collect(),
            };
        }
    };
    match forj_parser::parse(&tokens) {
        Ok(result) => {
            let node: forj_syntax::Node<'_, '_> = (&result).into();
            ParserResult::Ok { root: node.into() }
        }
        Err(err) => ParserResult::ParserErr { error: err.into() },
    }
}
