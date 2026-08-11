// =======================================================================
// lib.rs
// =======================================================================
//! Reporting errors from parsing

use crate::*;
pub use ariadne::{Cache, ReportKind};

/// A cache of sources that can be used for printing reports
///
/// Currently, this is implemented for iterators over
/// `(file_name, file_content)` tuples; this should be the common use case,
/// with others implemented at their own risk
pub trait Sources {
    fn sources(self) -> impl ariadne::Cache<String>;
}

impl<I, S> Sources for I
where
    I: IntoIterator<Item = (String, S)>,
    S: AsRef<str>,
{
    fn sources(self) -> impl ariadne::Cache<String> {
        ariadne::sources(self)
    }
}

/// A printable error report
///
/// These include detailed information about the location of the error,
/// and are printed with file snippets to assist the user
pub struct Report {
    builder: ariadne::ReportBuilder<'static, (String, std::ops::Range<usize>)>,
}

fn get_expansion_string(expansion_depth: usize, is_first: bool) -> String {
    if expansion_depth == 0 {
        "Original token".to_string()
    } else if (expansion_depth == 1) && is_first {
        "Expanded here".to_string()
    } else {
        let suffix = match expansion_depth % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        format!("Expanded here {}{}", expansion_depth, suffix)
    }
}

impl<'a> Report {
    /// Create a new [`Report`] indicating a particular location with a message
    ///
    /// This does not label/print the location; to do so, use [`Report::with_label`]
    pub fn new<C, M>(
        kind: ariadne::ReportKind<'static>,
        span: &Span<'a>,
        code: C,
        msg: M,
    ) -> Self
    where
        C: std::fmt::Display,
        M: ToString,
    {
        Self {
            builder: ariadne::Report::build(
                kind,
                (span.file.to_string(), span.bytes.clone()),
            )
            .with_config(
                ariadne::Config::new()
                    .with_index_type(ariadne::IndexType::Byte),
            )
            .with_code(code)
            .with_message(msg),
        }
    }

    /// Adds a label to the [`Report`]
    ///
    /// A label includes a [`Span`] to highlight, as well as a message to
    /// attach at that location
    pub fn with_label<M>(
        mut self,
        span: &Span<'a>,
        kind: ariadne::ReportKind<'static>,
        msg: M,
    ) -> Self
    where
        M: ToString,
    {
        let mut curr_span: &Span<'a> = span;
        let mut expansion_depth: usize = curr_span.expansion_depth();
        let mut expanded = false;
        let color = match kind {
            ReportKind::Error => Color::Red,
            ReportKind::Warning => Color::Yellow,
            ReportKind::Advice => Color::Fixed(147),
            ReportKind::Custom(_, color) => color.clone(),
        };
        loop {
            if let Some(expanded_span) = curr_span.expanded_from {
                self.builder = self.builder.with_label(
                    Label::new((
                        curr_span.file.to_string(),
                        curr_span.bytes.clone(),
                    ))
                    .with_message(get_expansion_string(
                        expansion_depth,
                        expanded == false,
                    ))
                    .with_color(Color::BrightGreen),
                );
                curr_span = expanded_span;
                expansion_depth -= 1;
                expanded = true;
            } else {
                break;
            }
        }
        if expanded {
            // Also label expansion in original spot
            self.builder = self.builder.with_label(
                Label::new((
                    curr_span.file.to_string(),
                    curr_span.bytes.clone(),
                ))
                .with_message(get_expansion_string(expansion_depth, false))
                .with_color(Color::BrightGreen),
            );
        }
        curr_span = &span;
        self.builder = self.builder.with_label(
            Label::new((curr_span.file.to_string(), curr_span.bytes.clone()))
                .with_message(msg)
                .with_color(color)
                .with_priority(1),
        );
        let mut note = "".to_string();
        let mut note_pad = "".to_string();
        let total_inclusion_depth = curr_span.inclusion_depth();
        let mut curr_inclusion_depth = 0;
        // Only display a max of 7 includes
        loop {
            if (curr_inclusion_depth == 3) & (total_inclusion_depth > 7) {
                for _ in 7..=total_inclusion_depth {
                    curr_span = curr_span.included_from.unwrap();
                }
                note = format!("{}\n{}╰- ...", note, note_pad);
                note_pad += "  ";
            }
            if let Some(included_span) = curr_span.included_from {
                curr_inclusion_depth += 1;
                curr_span = included_span;
                if note.is_empty() {
                    note = format!("Included from {}", curr_span.file);
                } else {
                    note = format!(
                        "{}\n{}╰-Included from {}",
                        note, note_pad, curr_span.file
                    );
                    note_pad += "  ";
                }
            } else {
                break;
            }
        }
        debug_assert!(curr_span.included_from.is_none());
        if !note.is_empty() {
            self.builder = self.builder.with_note(note);
        }
        self
    }

    /// Print the report to `stdout`
    pub fn print<C>(self, source_cache: &mut C) -> std::io::Result<()>
    where
        C: ariadne::Cache<String>,
    {
        let report = self.builder.finish();
        report.print(source_cache)
    }

    /// Print the report to `stderr`
    pub fn eprint<C>(self, source_cache: &mut C) -> std::io::Result<()>
    where
        C: ariadne::Cache<String>,
    {
        let report = self.builder.finish();
        report.eprint(source_cache)
    }

    /// Writes the report (without color) to a target with the [`std::io::Write`] trait
    pub fn write<C, W>(
        self,
        source_cache: &mut C,
        target: W,
    ) -> std::io::Result<()>
    where
        C: ariadne::Cache<String>,
        W: std::io::Write,
    {
        let report = self.builder.finish();
        report.write(source_cache, target)
    }
}
