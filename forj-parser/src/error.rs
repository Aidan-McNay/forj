// =======================================================================
// error.rs
// =======================================================================
//! Errors used in parsing

use crate::report::Report;
use crate::*;
use core::ops::Range;
use lexer::Token;
use forj_syntax::*;
use std::fmt;
use std::fs;
use winnow::{
    error::{AddContext, ParserError},
    stream::Stream,
};

/// A trait for displaying a short representation of an object as a [`String`]
pub(crate) trait DisplayShort {
    fn to_short_string(&self) -> String;
}

/// Something the parser expected to find instead of what was found,
/// in the case of an error
#[derive(Debug, Clone, PartialEq)]
pub enum Expectation<'s> {
    /// A particular lexed token
    Token(Token<'s>),
    /// A human-readable expectation
    Label(&'s str),
    /// The end of a file
    EOI,
}

impl<'a> std::fmt::Display for Expectation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expectation::Token(token) => token.fmt(f),
            Expectation::Label(label) => write!(f, "{}", label),
            Expectation::EOI => write!(f, "end of input"),
        }
    }
}

/// A verbose error message describing the error location, what was
/// found, and what was expected instead
#[derive(Debug, Clone, PartialEq)]
pub struct VerboseError<'s> {
    /// The [`Span`] where the error occurred
    pub span: Span<'s>,
    /// What token was found (if any - [`None`] indicates the end of a file)
    pub found: Option<Token<'s>>,
    /// What was expected instead of what was found
    pub expected: Vec<Expectation<'s>>,
}

impl<'s> ParserError<Tokens<'s>> for VerboseError<'s> {
    type Inner = Self;
    fn from_input(input: &Tokens<'s>) -> Self {
        match input.peek_token() {
            Some(token) => VerboseError {
                span: token.1.clone(),
                found: Some(token.0),
                expected: vec![],
            },
            None => {
                // Use the last token instead, indicate EOF
                match input.previous_tokens().next() {
                    Some(token) => {
                        let mut curr_span: &Span = &token.1;
                        let root_file = loop {
                            if let Some(included_from_span) =
                                curr_span.included_from
                            {
                                curr_span = included_from_span;
                            } else {
                                break curr_span.file;
                            }
                        };
                        VerboseError {
                            span: Span {
                                file: root_file,
                                bytes: Range {
                                    start: token.1.bytes.end,
                                    end: token.1.bytes.end,
                                },
                                expanded_from: None,
                                included_from: None,
                            },
                            found: None,
                            expected: vec![],
                        }
                    }
                    None => {
                        // No tokens ever present in input - use defaults
                        VerboseError {
                            span: Span::default(),
                            found: None,
                            expected: vec![],
                        }
                    }
                }
            }
        }
    }
    fn into_inner(self) -> winnow::Result<Self::Inner, Self> {
        Ok(self)
    }
    fn or(mut self, mut other: Self) -> Self {
        // Prefer errors that got to the end of the input
        match (self.found, other.found) {
            (None, Some(_)) => self,
            (Some(_), None) => other,
            (None, None) => {
                self.expected.append(&mut other.expected);
                self
            }
            (Some(_), Some(_)) => {
                // Prefer the one with a later span (a.k.a. got farther)
                match self.span.compare(&other.span) {
                    SpanRelation::Later => self,
                    SpanRelation::Earlier => other,
                    SpanRelation::Same => {
                        self.expected.append(&mut other.expected);
                        self
                    }
                }
            }
        }
    }
}

impl<'s> VerboseError<'s> {
    /// Similar to [`VerboseError::or`], but modifies an existing
    /// error instead of creating a new one
    pub(crate) fn or_in_place(&mut self, mut other: Self) {
        // Prefer errors that got to the end of the input
        match (self.found, other.found) {
            (None, Some(_)) => (),
            (Some(_), None) => *self = other,
            (None, None) => {
                self.expected.append(&mut other.expected);
            }
            (Some(_), Some(_)) => {
                // Prefer the one with a later span (a.k.a. got farther)
                match self.span.compare(&other.span) {
                    SpanRelation::Later => (),
                    SpanRelation::Earlier => *self = other,
                    SpanRelation::Same => {
                        self.expected.append(&mut other.expected);
                    }
                }
            }
        }
    }
}

impl<'s> AddContext<Tokens<'s>, Token<'s>> for VerboseError<'s> {
    fn add_context(
        mut self,
        _input: &Tokens<'s>,
        _token_start: &<Tokens<'s> as Stream>::Checkpoint,
        _context: Token<'s>,
    ) -> Self {
        self.expected.push(Expectation::Token(_context));
        self
    }
}
impl<'s> AddContext<Tokens<'s>, &'s str> for VerboseError<'s> {
    fn add_context(
        mut self,
        _input: &Tokens<'s>,
        _token_start: &<Tokens<'s> as Stream>::Checkpoint,
        _context: &'s str,
    ) -> Self {
        self.expected.push(Expectation::Label(_context));
        self
    }
}

impl<'a> fmt::Display for VerboseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "found ")?;
        match self.found {
            Some(tok) => tok.fmt(f)?,
            None => write!(f, "end of input")?,
        };
        write!(f, ", expected ")?;
        let mut dedup_expected: Vec<Expectation<'a>> = vec![];
        for expected in self.expected.iter() {
            if !dedup_expected.contains(expected) {
                dedup_expected.push(expected.clone());
            }
        }
        match &dedup_expected[..] {
            [] => write!(f, "something else"),
            [expected] => expected.fmt(f),
            _ => {
                for expected in &dedup_expected[..dedup_expected.len() - 1] {
                    expected.fmt(f)?;
                    write!(f, ", ")?;
                }
                write!(f, "or ")?;
                dedup_expected.last().unwrap().fmt(f)
            }
        }
    }
}

impl<'a> DisplayShort for VerboseError<'a> {
    fn to_short_string(&self) -> String {
        match self.found {
            Some(tok) => format!("Didn't expect {}", tok),
            None => "Didn't expect end of input".to_owned(),
        }
    }
}

impl<'s> VerboseError<'s> {
    /// Generate an error report for the [`VerboseError`]
    pub fn report<C>(&self, code: C) -> Report
    where
        C: fmt::Display,
    {
        let error_span = if self.found.is_none() {
            let file_len = fs::metadata(self.span.file)
                .expect("TODO: Handle file read error")
                .len();
            let byte_span = Range {
                start: file_len as usize,
                end: file_len as usize,
            };
            Span {
                file: self.span.file,
                bytes: byte_span,
                expanded_from: None,
                included_from: self.span.included_from,
            }
        } else {
            self.span.clone()
        };
        Report::new(
            report::ReportKind::Error,
            &error_span,
            code,
            self.to_string(),
        )
        .with_label(
            &error_span,
            report::ReportKind::Error,
            self.to_short_string(),
        )
    }
}
