// =======================================================================
// lib.rs
// =======================================================================
// The top-level collection of CST nodes

//! A SystemVerilog (concrete) syntax tree for representing
//! [1800-2023](https://ieeexplore.ieee.org/document/10458102) syntax.
//!
//! Each syntactic category is represented as a struct or enum with the
//! same name as appears in Annex A, with additional types as needed for
//! unnamed alternatives.
//!
//! While the tree can be treated as a normal data structure, it can
//! also be iterated across (depth-first). Iterating produces [`Node`]s,
//! which reference a specific data structure in the tree
//!
//! ```rust
//! # use forj_syntax::*;
//! fn is_always_comb_block<'a, 'b>(node: Node<'a, 'b>) -> bool {
//!   matches!(node, Node::AlwaysConstruct(AlwaysConstruct(AlwaysKeyword::AlwaysComb(_), _)))
//! }
//!
//! fn num_inferred_latches<'a>(source: &SourceText<'a>) -> i32 {
//!   let mut count = 0;
//!   for node in source.find(is_always_comb_block) {
//!       for child_node in node.iter() {
//!           if let Node::BlockingAssignment(_) = child_node {
//!               count += 1;
//!           }
//!       }
//!   }
//!   count
//! }
//! ```
//!
//! Compiler directives are not supported due to their arbitrary
//! location/semantics in a syntax tree.
//!
//! ## Features
//!
//!  - `lossless`: Allows for whitespace/comments to be preserved in [`Metadata`]

include!(concat!(env!("OUT_DIR"), "/nodes.rs"));
include!(concat!(env!("OUT_DIR"), "/id.rs"));

pub mod behavioral_statements;
pub mod declarations;
pub mod expressions;
pub mod general;
pub mod instantiations;
pub mod iter;
pub mod metadata;
pub mod primitive_instances;
pub mod source_text;
pub mod specify_section;
pub mod udp_declaration_and_instantiation;
pub use behavioral_statements::*;
pub use declarations::*;
pub use expressions::*;
pub use general::*;
pub use instantiations::*;
pub use iter::*;
pub use metadata::*;
pub use primitive_instances::*;
pub use source_text::*;
pub use specify_section::*;
pub use udp_declaration_and_instantiation::*;
