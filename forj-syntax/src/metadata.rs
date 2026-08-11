// =======================================================================
// metadata.rs
// =======================================================================
//! Extra metadata attached to leaf nodes to encode a CST

use crate::*;
use core::ops::Range;

/// The start and end bytes of a particular portion of a source file
pub type ByteSpan = Range<usize>;

/// A representation of a unique location in a source file
///
/// If the file was included from another file (using the `` `include ``
/// directive), [`Span::included_from`] will reference the [`Span`]
/// of the include directive
///
/// If the [`Span`] is part of a `` `define `` directive, each expanded
/// text macro will have the original [`Span`] of the (now expanded) macro,
/// with [`Span::expanded_from`] referencing the original token (in the
/// `` `define `` directive) before expansion
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Span<'a> {
    pub file: &'a str,
    pub bytes: ByteSpan,
    pub expanded_from: Option<&'a Span<'a>>,
    pub included_from: Option<&'a Span<'a>>,
}

/// A relationship between two [`Span`]s, ordering them relative to
/// each other in declaration order
#[derive(Debug, PartialEq)]
pub enum SpanRelation {
    Earlier,
    Later,
    Same,
}

impl<'a> Span<'a> {
    fn include_indeces(&self) -> Vec<usize> {
        match &self.included_from {
            None => {
                vec![self.bytes.start]
            }
            Some(inner_span) => {
                let mut nested_byte_indeces = inner_span.include_indeces();
                nested_byte_indeces.push(self.bytes.start);
                nested_byte_indeces
            }
        }
    }
    fn indeces_to_compare(&self) -> Vec<Vec<usize>> {
        let mut indeces = vec![self.include_indeces()];
        match self.expanded_from {
            Some(expanded_span) => {
                indeces.extend(expanded_span.indeces_to_compare())
            }
            None => (),
        };
        indeces
    }
    /// Compare two [`Span`]s, returning the relationship of the first relative to the second
    ///
    /// ```rust
    /// # use forj_syntax::*;
    /// let span1 = Span {
    ///     file: "test",
    ///     bytes: (0..2),
    ///     expanded_from: None,
    ///     included_from: None
    /// };
    /// let span2 = Span {
    ///     file: "test",
    ///     bytes: (6..8),
    ///     expanded_from: None,
    ///     included_from: None
    /// };
    /// assert_eq!(span1.compare(&span2), SpanRelation::Earlier)
    /// ```
    ///
    /// Expanded [`Span`]s will be compared starting at their expansion
    /// point, and working backwards through `#define`s. Included [`Span`]s
    /// will be compared starting at their first `#include` and working
    /// through the include hierarchy to their final token. If a [`Span`]
    /// is included with an `` `include `` directive **AND** expanded from
    /// a `` `define `` directive, the former is used before the latter.
    pub fn compare(&self, other: &Self) -> SpanRelation {
        let mut idx = 0;
        let self_include_byte_indeces = self.indeces_to_compare();
        let other_include_byte_indeces = other.indeces_to_compare();
        loop {
            match (
                self_include_byte_indeces.get(idx),
                other_include_byte_indeces.get(idx),
            ) {
                (Some(self_idxs), Some(other_idxs)) => {
                    let mut nested_idx = 0;
                    'match_define: loop {
                        match (
                            self_idxs.get(nested_idx),
                            other_idxs.get(nested_idx),
                        ) {
                            (Some(self_idx), Some(other_idx)) => {
                                if self_idx < other_idx {
                                    return SpanRelation::Earlier;
                                } else if self_idx > other_idx {
                                    return SpanRelation::Later;
                                } else {
                                    nested_idx += 1;
                                }
                            }
                            (None, None) => {
                                break 'match_define;
                            }
                            _ => {
                                panic!(
                                    "Internal error comparing spans {:?} and {:?}",
                                    self, other
                                )
                            }
                        }
                    }
                    idx += 1;
                }
                (None, None) => {
                    break SpanRelation::Same;
                }
                _ => {
                    panic!(
                        "Internal error comparing spans {:?} and {:?}",
                        self, other
                    )
                }
            }
        }
    }

    /// How many preprocessor macros this [`Span`] was expanded from
    pub const fn expansion_depth(&self) -> usize {
        let mut curr_span: &Span = self;
        let mut depth = 0;
        loop {
            if let Some(expanded_from_span) = curr_span.expanded_from {
                curr_span = expanded_from_span;
                depth += 1;
            } else {
                break;
            }
        }
        depth
    }

    /// How many `` `include `` macros this [`Span`] was included with
    pub const fn inclusion_depth(&self) -> usize {
        let mut curr_span: &Span = self;
        let mut depth = 0;
        loop {
            if let Some(included_from_span) = curr_span.included_from {
                curr_span = included_from_span;
                depth += 1;
            } else {
                break;
            }
        }
        depth
    }
}

#[test]
fn basic_span_comparison() {
    let span1 = Span {
        file: "",
        bytes: (0..2),
        expanded_from: None,
        included_from: None,
    };
    let span2 = Span {
        file: "",
        bytes: (2..4),
        expanded_from: None,
        included_from: None,
    };
    assert_eq!(span1.compare(&span2), SpanRelation::Earlier)
}

#[test]
fn include_span_comparison() {
    let span1 = Span {
        file: "",
        bytes: (2..4),
        expanded_from: None,
        included_from: None,
    };
    let included_span = Span {
        file: "",
        bytes: (0..2),
        expanded_from: None,
        included_from: None,
    };
    // Included from an earlier point
    let span2 = Span {
        file: "",
        bytes: (4..6),
        expanded_from: None,
        included_from: Some(&included_span),
    };
    assert_eq!(span1.compare(&span2), SpanRelation::Later)
}

#[test]
fn define_span_comparison() {
    let span1 = Span {
        file: "",
        bytes: (2..4),
        expanded_from: None,
        included_from: None,
    };
    let definition_span = Span {
        file: "",
        bytes: (0..2),
        expanded_from: None,
        included_from: None,
    };
    // Defined earlier, but should still occur later
    let span2 = Span {
        file: "",
        bytes: (4..6),
        expanded_from: Some(&definition_span),
        included_from: None,
    };
    assert_eq!(span1.compare(&span2), SpanRelation::Earlier)
}

#[test]
fn mixed_define_span_comparison() {
    let definition_span1 = Span {
        file: "",
        bytes: (102..104),
        expanded_from: None,
        included_from: None,
    };
    let span1 = Span {
        file: "",
        bytes: (0..2),
        expanded_from: Some(&definition_span1),
        included_from: None,
    };
    let definition_span2 = Span {
        file: "",
        bytes: (100..102),
        expanded_from: None,
        included_from: None,
    };
    // Defined earlier, but should still occur later
    let span2 = Span {
        file: "",
        bytes: (2..4),
        expanded_from: Some(&definition_span2),
        included_from: None,
    };
    assert_eq!(span1.compare(&span2), SpanRelation::Earlier)
}

#[test]
fn same_definition_span_comparison() {
    let definition_span1 = Span {
        file: "",
        bytes: (2..4),
        expanded_from: None,
        included_from: None,
    };
    let span1 = Span {
        file: "",
        bytes: (100..104),
        expanded_from: Some(&definition_span1),
        included_from: None,
    };
    let definition_span2 = Span {
        file: "",
        bytes: (0..2),
        expanded_from: None,
        included_from: None,
    };
    let span2 = Span {
        file: "",
        bytes: (100..104),
        expanded_from: Some(&definition_span2),
        included_from: None,
    };
    assert_eq!(span1.compare(&span2), SpanRelation::Later)
}

#[test]
fn define_include_span_comparison() {
    // Prefer inclusion spots over expansion initially
    let definition_span1 = Span {
        file: "",
        bytes: (2..4),
        expanded_from: None,
        included_from: None,
    };
    let inclusion_span1 = Span {
        file: "",
        bytes: (50..52),
        expanded_from: None,
        included_from: None,
    };
    let span1 = Span {
        file: "",
        bytes: (100..104),
        expanded_from: Some(&definition_span1),
        included_from: Some(&inclusion_span1),
    };
    let definition_span2 = Span {
        file: "",
        bytes: (0..2),
        expanded_from: None,
        included_from: None,
    };
    let inclusion_span2 = Span {
        file: "",
        bytes: (52..54),
        expanded_from: None,
        included_from: None,
    };
    let span2 = Span {
        file: "",
        bytes: (100..104),
        expanded_from: Some(&definition_span2),
        included_from: Some(&inclusion_span2),
    };
    assert_eq!(span1.compare(&span2), SpanRelation::Earlier)
}

#[test]
fn same_define_include_span_comparison() {
    // Use expansion locations if inclusion spots are the same
    let definition_span1 = Span {
        file: "",
        bytes: (2..4),
        expanded_from: None,
        included_from: None,
    };
    let inclusion_span1 = Span {
        file: "",
        bytes: (50..52),
        expanded_from: None,
        included_from: None,
    };
    let span1 = Span {
        file: "",
        bytes: (100..104),
        expanded_from: Some(&definition_span1),
        included_from: Some(&inclusion_span1),
    };
    let definition_span2 = Span {
        file: "",
        bytes: (0..2),
        expanded_from: None,
        included_from: None,
    };
    let inclusion_span2 = Span {
        file: "",
        bytes: (50..52),
        expanded_from: None,
        included_from: None,
    };
    let span2 = Span {
        file: "",
        bytes: (100..104),
        expanded_from: Some(&definition_span2),
        included_from: Some(&inclusion_span2),
    };
    assert_eq!(span1.compare(&span2), SpanRelation::Later)
}

/// Metadata for a given syntax token.
///
/// This includes the [`Span`] of the token. With the `lossless` feature,
/// [`Metadata`] also includes `non_trivia`, which stores non-trivia tokens
/// such as whitespace and comments - see [`NonTriviaToken`]
#[cfg(feature = "lossless")]
#[derive(Default, Clone, Debug)]
pub struct Metadata<'a> {
    pub span: Span<'a>,
    pub non_trivia: Vec<NonTriviaToken<'a>>,
}

/// Metadata for a given syntax token.
///
/// This includes the [`Span`] of the token. With the `lossless` feature,
/// [`Metadata`] also includes `non_trivia`, which stores non-trivia tokens
/// such as whitespace and comments - see [`NonTriviaToken`]
#[cfg(not(feature = "lossless"))]
#[derive(Default, Clone, Debug)]
pub struct Metadata<'a> {
    pub span: Span<'a>,
}

impl<'a> Metadata<'a> {
    /// Construct a new [`Metadata`]. If the `lossless` feature isn't enabled,
    /// `non_trivia` is discarded.
    #[cfg(feature = "lossless")]
    pub fn new(span: Span<'a>, non_trivia: Vec<NonTriviaToken<'a>>) -> Self {
        Self { span, non_trivia }
    }

    /// Construct a new [`Metadata`]. If the `lossless` feature isn't enabled,
    /// `non_trivia` is discarded.
    #[cfg(not(feature = "lossless"))]
    #[allow(unused_variables)]
    pub fn new(span: Span<'a>, non_trivia: Vec<NonTriviaToken<'a>>) -> Self {
        Self { span }
    }
}

impl<'a> PartialEq for Metadata<'a> {
    fn eq(&self, _: &Self) -> bool {
        // Allows checking of overall CST structure without checking
        // exact whitespace
        true
    }
}

// Metadata never returns any Nodes while iterating
impl<'a, 'b> IntoIterator for &'b Metadata<'a> {
    type Item = Node<'a, 'b>;
    type IntoIter = std::iter::Empty<Node<'a, 'b>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl<'a, 'b> IntoIterator for &'b mut Metadata<'a> {
    type Item = Node<'a, 'b>;
    type IntoIter = std::iter::Empty<Node<'a, 'b>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

/// A non-trivia token from the source file
#[derive(Clone, Debug, PartialEq)]
pub enum NonTriviaToken<'a> {
    OnelineComment(&'a str, Span<'a>),
    BlockComment(&'a str, Span<'a>),
    Newline,
}
