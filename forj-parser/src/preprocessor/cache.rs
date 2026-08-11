// =======================================================================
// cache.rs
// =======================================================================
//! A cache for storing data generated during preprocessing

use crate::*;
use elsa::FrozenVec;

/// A cache for storing generated [`Span`]s and strings.
///
/// These objects are only meant to be used internal to the
/// preprocessor. Since the CST only stores references to copy as
/// little as possible, the [`PreprocessorCache`] stores new
/// [`Span`]s and strings created as part of preprocessing (such
/// as when referencing `` `include `` directives and elaborating
/// preprocessor identifiers, respectively). Since the CST stores
/// references to these, a [`PreprocessorCache`] used as part
/// of preprocessing must be kept alive as long as the CST itself.
pub struct PreprocessorCache<'a> {
    spans: FrozenVec<Box<Span<'a>>>,
    strings: FrozenVec<Box<str>>,
}

impl<'a> PreprocessorCache<'a> {
    /// Store a [`Span`] for the cache's lifetime
    pub fn retain_span(&self, span: Span<'a>) -> &Span<'a> {
        self.spans.push_get(Box::new(span))
    }
    /// Store a [`String`] for the cache's lifetime
    pub fn retain_string(&self, string: String) -> &str {
        self.strings.push_get(string.into_boxed_str())
    }
    /// Create a new cache for storing preprocessor results
    pub fn new() -> Self {
        Self {
            spans: FrozenVec::new(),
            strings: FrozenVec::new(),
        }
    }
}
