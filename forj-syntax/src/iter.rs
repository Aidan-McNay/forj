// =======================================================================
// iter.rs
// =======================================================================
//! Iterating over a CST

use crate::*;
use std::cmp::{max, min};
use std::ops::Add;

/// An iterator over nodes in a syntax tree
///
/// Nodes are iterated across in a depth-first fashion, in the same order
/// they appear in source files
///
/// ```rust
/// # use forj_syntax::*;
/// /// The number of non-blocking assignments within an `always` block
/// fn num_nonblock_assign(block: &AlwaysConstruct) -> i32 {
///   let always_construct_node: Node = block.into();
///   let always_construct_iter: NodeIter = always_construct_node.into();
///   let mut count = 0;
///   for node in always_construct_iter {
///     if let Node::NonblockingAssignment(_) = node {
///       count += 1;
///     }
///   }
///   count
/// }
/// ```
pub struct NodeIter<'a, 'b> {
    nodes: Vec<Node<'a, 'b>>,
}

impl<'a: 'b, 'b> NodeIter<'a, 'b> {
    /// Get the underlying nodes in the iterator
    ///
    /// Note that this may not contain *all* nodes
    /// represented by the iterator, as nodes are added
    /// during iteration to minimize storage
    pub(crate) fn raw(self) -> Vec<Node<'a, 'b>> {
        self.nodes
    }
}

impl<'a: 'b, 'b> Default for NodeIter<'a, 'b> {
    fn default() -> Self {
        NodeIter { nodes: Vec::new() }
    }
}

impl<'a: 'b, 'b> From<Node<'a, 'b>> for NodeIter<'a, 'b> {
    fn from(value: Node<'a, 'b>) -> Self {
        NodeIter { nodes: vec![value] }
    }
}

impl<'a: 'b, 'b> Add for NodeIter<'a, 'b> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.nodes.extend(rhs.nodes);
        self
    }
}

impl<'a: 'b, 'b> Iterator for NodeIter<'a, 'b> {
    type Item = Node<'a, 'b>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.nodes.pop();
        if let Some(node) = next.clone() {
            let mut new_nodes: Vec<Node<'a, 'b>> = node.children();
            new_nodes.reverse();
            self.nodes.append(&mut new_nodes);
        };
        next
    }
}

/// An object that can be represented as a collection of CST nodes
///
/// This is implemented by all individual CST nodes (see the associated
/// data for [`Node`] variants) to represent its subtree.
pub trait Nodes<'a: 'b, 'b> {
    /// The nodes of the object
    ///
    /// This includes the object itself, as well as any/all children nodes,
    /// provided depth-first.
    fn nodes(&'b self) -> NodeIter<'a, 'b>;

    /// The overall [`Span`] of the nodes.
    ///
    /// The resulting [`Span`] is for the overall node; if the node happes
    /// to go across files, the [`Span`] will only be for the first child node
    fn span(&'b self) -> Span<'a>;

    /// Add all children nodes satisfying the given predicate to
    /// the provided [`Vec`]
    ///
    /// This is faster than iterating across the entire tree, as the tree
    /// can be evaluated completely rather than being itermediately
    /// stored
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    );

    /// Find all children nodes satisfying the given predicate
    ///
    /// This is faster than iterating across the entire tree, as the tree
    /// can be evaluated completely rather than being itermediately
    /// stored
    fn find(&'b self, pred: fn(Node<'a, 'b>) -> bool) -> Vec<Node<'a, 'b>> {
        let mut result = Vec::new();
        self.add_nodes(&mut result, pred);
        result
    }
}

pub(crate) fn merge_spans<'a>(span1: Span<'a>, span2: Span<'a>) -> Span<'a> {
    if span1.file == "" {
        span2
    } else if span2.file == "" {
        span1
    } else if span1.file == span2.file {
        Span {
            bytes: (min(span1.bytes.start, span2.bytes.start)
                ..max(span1.bytes.end, span2.bytes.end)),
            ..span1
        }
    } else {
        span1
    }
}

impl<'a: 'b, 'b, T> Nodes<'a, 'b> for Box<T>
where
    T: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.as_ref().nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.as_ref().add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        self.as_ref().span()
    }
}

impl<'a: 'b, 'b, T> Nodes<'a, 'b> for Option<T>
where
    T: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        match self {
            Some(data) => data.nodes(),
            None => NodeIter::default(),
        }
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        match self {
            Some(data) => data.add_nodes(dest, pred),
            None => (),
        }
    }
    fn span(&'b self) -> Span<'a> {
        match self {
            Some(data) => data.span(),
            None => Span::default(),
        }
    }
}

impl<'a: 'b, 'b, T> Nodes<'a, 'b> for Vec<T>
where
    T: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        let mut ret: NodeIter<'a, 'b> = NodeIter::default();
        for member in self {
            ret = ret + member.nodes().into();
        }
        ret
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        for member in self {
            member.add_nodes(dest, pred);
        }
    }
    fn span(&'b self) -> Span<'a> {
        self.iter()
            .map(|child_node| child_node.span())
            .reduce(merge_spans)
            .unwrap_or(Span::default())
    }
}

impl<'a: 'b, 'b, T0, T1> Nodes<'a, 'b> for (T0, T1)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes() + self.1.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        merge_spans(self.0.span(), self.1.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2> Nodes<'a, 'b> for (T0, T1, T2)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes() + self.1.nodes() + self.2.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        merge_spans(merge_span, self.2.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3: 'a> Nodes<'a, 'b> for (T0, T1, T2, T3)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes() + self.1.nodes() + self.2.nodes() + self.3.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        merge_spans(merge_span, self.3.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4: 'a> Nodes<'a, 'b> for (T0, T1, T2, T3, T4)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
        self.4.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        let merge_span = merge_spans(merge_span, self.3.span());
        merge_spans(merge_span, self.4.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5: 'a> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
        self.4.add_nodes(dest, pred);
        self.5.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        let merge_span = merge_spans(merge_span, self.3.span());
        let merge_span = merge_spans(merge_span, self.4.span());
        merge_spans(merge_span, self.5.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6: 'a> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
        self.4.add_nodes(dest, pred);
        self.5.add_nodes(dest, pred);
        self.6.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        let merge_span = merge_spans(merge_span, self.3.span());
        let merge_span = merge_spans(merge_span, self.4.span());
        let merge_span = merge_spans(merge_span, self.5.span());
        merge_spans(merge_span, self.6.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7: 'a> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
        self.4.add_nodes(dest, pred);
        self.5.add_nodes(dest, pred);
        self.6.add_nodes(dest, pred);
        self.7.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        let merge_span = merge_spans(merge_span, self.3.span());
        let merge_span = merge_spans(merge_span, self.4.span());
        let merge_span = merge_spans(merge_span, self.5.span());
        let merge_span = merge_spans(merge_span, self.6.span());
        merge_spans(merge_span, self.7.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7, T8> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
    T8: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
            + self.8.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
        self.4.add_nodes(dest, pred);
        self.5.add_nodes(dest, pred);
        self.6.add_nodes(dest, pred);
        self.7.add_nodes(dest, pred);
        self.8.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        let merge_span = merge_spans(merge_span, self.3.span());
        let merge_span = merge_spans(merge_span, self.4.span());
        let merge_span = merge_spans(merge_span, self.5.span());
        let merge_span = merge_spans(merge_span, self.6.span());
        let merge_span = merge_spans(merge_span, self.7.span());
        merge_spans(merge_span, self.8.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
    T8: Nodes<'a, 'b>,
    T9: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
            + self.8.nodes()
            + self.9.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
        self.4.add_nodes(dest, pred);
        self.5.add_nodes(dest, pred);
        self.6.add_nodes(dest, pred);
        self.7.add_nodes(dest, pred);
        self.8.add_nodes(dest, pred);
        self.9.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        let merge_span = merge_spans(merge_span, self.3.span());
        let merge_span = merge_spans(merge_span, self.4.span());
        let merge_span = merge_spans(merge_span, self.5.span());
        let merge_span = merge_spans(merge_span, self.6.span());
        let merge_span = merge_spans(merge_span, self.7.span());
        let merge_span = merge_spans(merge_span, self.8.span());
        merge_spans(merge_span, self.9.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
    T8: Nodes<'a, 'b>,
    T9: Nodes<'a, 'b>,
    T10: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
            + self.8.nodes()
            + self.9.nodes()
            + self.10.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
        self.4.add_nodes(dest, pred);
        self.5.add_nodes(dest, pred);
        self.6.add_nodes(dest, pred);
        self.7.add_nodes(dest, pred);
        self.8.add_nodes(dest, pred);
        self.9.add_nodes(dest, pred);
        self.10.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        let merge_span = merge_spans(merge_span, self.3.span());
        let merge_span = merge_spans(merge_span, self.4.span());
        let merge_span = merge_spans(merge_span, self.5.span());
        let merge_span = merge_spans(merge_span, self.6.span());
        let merge_span = merge_spans(merge_span, self.7.span());
        let merge_span = merge_spans(merge_span, self.8.span());
        let merge_span = merge_spans(merge_span, self.9.span());
        merge_spans(merge_span, self.10.span())
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
    T8: Nodes<'a, 'b>,
    T9: Nodes<'a, 'b>,
    T10: Nodes<'a, 'b>,
    T11: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
            + self.8.nodes()
            + self.9.nodes()
            + self.10.nodes()
            + self.11.nodes()
    }
    fn add_nodes(
        &'b self,
        dest: &mut Vec<Node<'a, 'b>>,
        pred: fn(Node<'a, 'b>) -> bool,
    ) {
        self.0.add_nodes(dest, pred);
        self.1.add_nodes(dest, pred);
        self.2.add_nodes(dest, pred);
        self.3.add_nodes(dest, pred);
        self.4.add_nodes(dest, pred);
        self.5.add_nodes(dest, pred);
        self.6.add_nodes(dest, pred);
        self.7.add_nodes(dest, pred);
        self.8.add_nodes(dest, pred);
        self.9.add_nodes(dest, pred);
        self.10.add_nodes(dest, pred);
        self.11.add_nodes(dest, pred);
    }
    fn span(&'b self) -> Span<'a> {
        let merge_span = merge_spans(self.0.span(), self.1.span());
        let merge_span = merge_spans(merge_span, self.2.span());
        let merge_span = merge_spans(merge_span, self.3.span());
        let merge_span = merge_spans(merge_span, self.4.span());
        let merge_span = merge_spans(merge_span, self.5.span());
        let merge_span = merge_spans(merge_span, self.6.span());
        let merge_span = merge_spans(merge_span, self.7.span());
        let merge_span = merge_spans(merge_span, self.8.span());
        let merge_span = merge_spans(merge_span, self.9.span());
        let merge_span = merge_spans(merge_span, self.10.span());
        merge_spans(merge_span, self.11.span())
    }
}

impl<'a: 'b, 'b> Nodes<'a, 'b> for Metadata<'a> {
    fn nodes(&self) -> NodeIter<'a, 'b> {
        NodeIter::default()
    }
    fn add_nodes(
        &'b self,
        _: &mut Vec<Node<'a, 'b>>,
        _: fn(Node<'a, 'b>) -> bool,
    ) {
    }
    fn span(&'b self) -> Span<'a> {
        self.span.clone()
    }
}

impl<'a: 'b, 'b> Nodes<'a, 'b> for NonTriviaToken<'a> {
    fn nodes(&self) -> NodeIter<'a, 'b> {
        NodeIter::default()
    }
    fn add_nodes(
        &'b self,
        _: &mut Vec<Node<'a, 'b>>,
        _: fn(Node<'a, 'b>) -> bool,
    ) {
    }
    fn span(&'b self) -> Span<'a> {
        Span::default()
    }
}

impl<'a: 'b, 'b> Nodes<'a, 'b> for &'a str {
    fn nodes(&self) -> NodeIter<'a, 'b> {
        NodeIter::default()
    }
    fn add_nodes(
        &'b self,
        _: &mut Vec<Node<'a, 'b>>,
        _: fn(Node<'a, 'b>) -> bool,
    ) {
    }
    fn span(&'b self) -> Span<'a> {
        Span::default()
    }
}

#[cfg(test)]
fn example_source() -> SourceText<'static> {
    SourceText(
        vec![],
        None,
        vec![Description::ModuleDeclaration(Box::new(
            ModuleDeclaration::Ansi(Box::new(ModuleDeclarationAnsi(
                ModuleAnsiHeader(
                    vec![],
                    ModuleKeyword::Module(Metadata::default()),
                    None,
                    ModuleIdentifier(Identifier::SimpleIdentifier((
                        "test_module",
                        Metadata::default(),
                    ))),
                    vec![],
                    None,
                    None,
                    Metadata::default(),
                ),
                None,
                vec![
                    NonPortModuleItem::ModuleOrGenerate(Box::new(
                        ModuleOrGenerateItem::ModuleCommon(
                            Box::new((
                                vec![],
                                ModuleCommonItem::ModuleOrGenerateDeclaration(Box::new(
                                    ModuleOrGenerateItemDeclaration::PackageOrGenerate(Box::new(
                                        PackageOrGenerateItemDeclaration::Data(
                                            Box::new(
                                                DataDeclaration::Variable(
                                                    Box::new((
                                                        None,
                                                        None,
                                                        None,
                                                        DataTypeOrImplicit::DataType(DataType::Vector(Box::new((
                                                            IntegerVectorType::Logic(Metadata::default()),
                                                            None,
                                                            vec![]
                                                        )))),
                                                        ListOfVariableDeclAssignments(
                                                            VariableDeclAssignment::Variable(Box::new((
                                                                VariableIdentifier(Identifier::SimpleIdentifier(("my_signal", Metadata::default()))),
                                                                vec![],
                                                                None)
                                                            )),
                                                            vec![]
                                                        ),
                                                        Metadata::default()
                                                    ))
                                                )
                                            )
                                        )
                                    ))
                                ))
                            ))
                        )
                    ))
                ],
                Metadata::default(),
                None,
            ))),
        ))],
    )
}

#[test]
fn iterate() {
    let source = example_source();
    let mut nodes = source.nodes();
    assert!(matches!(nodes.next().unwrap(), Node::SourceText(_)));
    assert!(matches!(nodes.next().unwrap(), Node::Description(_)));
    assert!(matches!(nodes.next().unwrap(), Node::ModuleDeclaration(_)));
    assert!(matches!(
        nodes.next().unwrap(),
        Node::ModuleDeclarationAnsi(_)
    ));
    assert!(matches!(nodes.next().unwrap(), Node::ModuleAnsiHeader(_)));
    assert!(matches!(nodes.next().unwrap(), Node::ModuleKeyword(_)));
    assert!(matches!(nodes.next().unwrap(), Node::ModuleIdentifier(_)));
    assert!(matches!(nodes.next().unwrap(), Node::Identifier(_)));
    assert!(matches!(nodes.next().unwrap(), Node::NonPortModuleItem(_)));
    assert!(matches!(
        nodes.next().unwrap(),
        Node::ModuleOrGenerateItem(_)
    ));
    assert!(matches!(nodes.next().unwrap(), Node::ModuleCommonItem(_)));
    assert!(matches!(
        nodes.next().unwrap(),
        Node::ModuleOrGenerateItemDeclaration(_)
    ));
    assert!(matches!(
        nodes.next().unwrap(),
        Node::PackageOrGenerateItemDeclaration(_)
    ));
    assert!(matches!(nodes.next().unwrap(), Node::DataDeclaration(_)));
    assert!(matches!(nodes.next().unwrap(), Node::DataTypeOrImplicit(_)));
    assert!(matches!(nodes.next().unwrap(), Node::DataType(_)));
    assert!(matches!(nodes.next().unwrap(), Node::IntegerVectorType(_)));
    assert!(matches!(
        nodes.next().unwrap(),
        Node::ListOfVariableDeclAssignments(_)
    ));
    assert!(matches!(
        nodes.next().unwrap(),
        Node::VariableDeclAssignment(_)
    ));
    assert!(matches!(nodes.next().unwrap(), Node::VariableIdentifier(_)));
    assert!(matches!(nodes.next().unwrap(), Node::Identifier(_)));
    assert!(nodes.next().is_none());
}

#[cfg(test)]
fn is_identifier<'a, 'b>(node: Node<'a, 'b>) -> bool {
    matches!(node, Node::Identifier(_))
}

#[test]
fn find() {
    let source = example_source();
    let source_node: Node<'_, '_> = (&source).into();
    let identifiers: Vec<Node<'_, '_>> = source_node.find(is_identifier);
    assert_eq!(identifiers.len(), 2);
    let mut identifier_iter = identifiers.iter();
    assert!(matches!(
        identifier_iter.next().unwrap(),
        Node::Identifier(Identifier::SimpleIdentifier(("test_module", _)))
    ));
    assert!(matches!(
        identifier_iter.next().unwrap(),
        Node::Identifier(Identifier::SimpleIdentifier(("my_signal", _)))
    ));
}
