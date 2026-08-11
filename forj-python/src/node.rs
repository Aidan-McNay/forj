// =======================================================================
// node.rs
// =======================================================================
//! A wrapper around [`forj_syntax::Node`]

use pyo3::{exceptions::PyIOError, prelude::*};
use forj_parser::PreprocessorCache;
use forj_syntax::*;
use std::io::{Read, Seek};
use std::{fs::File, ops::Range};

/// A wrapper around [`std::ops::Range<usize>`]
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct Bytes {
    /// The start of the byte-span (inclusive)
    #[pyo3(get, set)]
    pub start: usize,
    /// The end of the byte-span (exclusive)
    #[pyo3(get, set)]
    pub end: usize,
}

impl<T> From<Range<T>> for Bytes
where
    T: Into<usize>,
{
    fn from(value: Range<T>) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl<T> From<Bytes> for Range<T>
where
    T: From<usize>,
{
    fn from(value: Bytes) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

/// A wrapper around [`forj_syntax::Span`], providing a location in the
/// source code where a [`Node`] was found
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct Span {
    /// The source file
    #[pyo3(get, set)]
    pub file: String,
    /// The byte-span within the source file
    #[pyo3(get, set)]
    pub bytes: Bytes,
    /// The [`Span`] of the original token, if this is a text macro
    pub expanded_from: Option<Box<Span>>,
    /// The [`Span`] of the `` `include `` directive that produced this
    /// one, if any
    pub included_from: Option<Box<Span>>,
}

impl<'a> From<forj_syntax::Span<'a>> for Span {
    fn from(value: forj_syntax::Span<'a>) -> Self {
        Self {
            file: value.file.to_string(),
            bytes: value.bytes.into(),
            expanded_from: match value.expanded_from {
                Some(expanded_from_ref) => {
                    Some(Box::new(expanded_from_ref.clone().into()))
                }
                None => None,
            },
            included_from: match value.included_from {
                Some(included_from_ref) => {
                    Some(Box::new(included_from_ref.clone().into()))
                }
                None => None,
            },
        }
    }
}

impl<'a> Span {
    /// Turn a [`Span`] into a [`forj_syntax::Span`]
    pub(crate) fn to_span(
        &'a self,
        cache: &'a PreprocessorCache<'a>,
    ) -> forj_syntax::Span<'a> {
        forj_syntax::Span {
            file: &self.file,
            bytes: self.bytes.clone().into(),
            expanded_from: match &self.expanded_from {
                Some(expanded_from_box) => {
                    Some(expanded_from_box.as_ref().to_span_ref(cache))
                }
                None => None,
            },
            included_from: match &self.included_from {
                Some(included_from_box) => {
                    Some(included_from_box.as_ref().to_span_ref(cache))
                }
                None => None,
            },
        }
    }
}

#[pymethods]
impl Span {
    // Get the corresponding text from a [`Span`]
    #[getter]
    pub fn text(&self) -> PyResult<String> {
        let mut file = File::open(self.file.as_str())
            .map_err(|err| PyIOError::new_err(err.to_string()))?;
        file.seek(std::io::SeekFrom::Start(self.bytes.start as u64))
            .map_err(|err| PyIOError::new_err(err.to_string()))?;
        let mut byte_buf = vec![0_u8; self.bytes.end - self.bytes.start];
        file.read_exact(&mut byte_buf)
            .map_err(|err| PyIOError::new_err(err.to_string()))?;
        String::from_utf8(byte_buf)
            .map_err(|err| PyIOError::new_err(err.to_string()))
    }
}

impl<'a> Span {
    /// Turn a [`Span`] into a [`&forj_syntax::Span`]
    pub(crate) fn to_span_ref(
        &'a self,
        cache: &'a PreprocessorCache<'a>,
    ) -> &'a forj_syntax::Span<'a> {
        let new_span = self.to_span(cache);
        cache.retain_span(new_span)
    }
}

/// A wrapper around [`forj_syntax::Node`], providing a single CST node
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct Node {
    /// The name of the [`Node`] (see [`forj_syntax::Node::name`])
    #[pyo3(get, set)]
    pub name: String,
    /// The span of the [`Node`] (see [`forj_syntax::Node::span`])
    #[pyo3(get, set)]
    pub span: Span,
    /// All direct children of the [`Node`] in the CST (see [`forj_syntax::Node::children`])
    #[pyo3(get, set)]
    pub children: Vec<Node>,
}

impl<'a, 'b> From<forj_syntax::Node<'a, 'b>> for Node {
    fn from(value: forj_syntax::Node<'a, 'b>) -> Self {
        Self {
            name: value.name().to_string(),
            span: value.span().into(),
            children: value
                .children()
                .into_iter()
                .map(|rust_node: forj_syntax::Node<'_, '_>| rust_node.into())
                .collect(),
        }
    }
}

#[pymethods]
impl Node {
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<NodeIterator>> {
        Py::new(slf.py(), NodeIterator::new(slf.clone()))
    }
    // Get the corresponding text from a [`Node`]
    #[getter]
    pub fn text(&self) -> PyResult<String> {
        self.span.text()
    }
}

/// An iterator over [`Node`]s
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct NodeIterator {
    nodes: Vec<Node>,
}

impl NodeIterator {
    pub fn new(root: Node) -> Self {
        Self { nodes: vec![root] }
    }
}

#[pymethods]
impl NodeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Node> {
        match slf.nodes.pop() {
            Some(node) => {
                slf.nodes.append(
                    &mut node
                        .children
                        .clone()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>(),
                );
                Some(node)
            }
            None => None,
        }
    }
}
