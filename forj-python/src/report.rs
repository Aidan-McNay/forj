// =======================================================================
// report.rs
// =======================================================================
//! A wrapper around [`forj_parser::report::Report`]

use crate::{PreprocessorError, Span};
use pyo3::prelude::*;
use forj_parser::{PreprocessorCache, report::Sources};
use std::fs;
use yansi::Color;

const NOTE_COLOR: Color = Color::Fixed(81);
const NOTE_KIND: forj_parser::report::ReportKind<'static> =
    forj_parser::report::ReportKind::Custom("note", NOTE_COLOR);

/// The type of report being generated, used for coloring output
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ReportKind {
    Error(),
    Warning(),
    Advice(),
    Note(),
}

impl From<ReportKind> for forj_parser::report::ReportKind<'static> {
    fn from(value: ReportKind) -> Self {
        match value {
            ReportKind::Error() => forj_parser::report::ReportKind::Error,
            ReportKind::Warning() => forj_parser::report::ReportKind::Warning,
            ReportKind::Advice() => forj_parser::report::ReportKind::Advice,
            ReportKind::Note() => NOTE_KIND,
        }
    }
}

/// The base of a [`Report`]
#[derive(Clone, PartialEq, Eq)]
struct ReportBase {
    kind: ReportKind,
    span: Span,
    code: String,
    msg: String,
}

/// A label in a [`Report`]
#[derive(Clone, PartialEq, Eq)]
struct ReportLabel {
    span: Span,
    kind: ReportKind,
    msg: String,
}

/// A wrapper around [`forj_parser::report::Report`]
///
/// Here, since we can't pass in a cache from Python, we track which
/// files are added in labels, so that we can re-read them in during
/// printing
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct Report {
    /// The base of the [`Report`]
    base: ReportBase,
    /// The labels in the [`Report`]
    labels: Vec<ReportLabel>,
    /// Any additional source files that are needed for printing
    additional_sources: Vec<(String, String)>,
}

impl Report {
    /// Generates the [`forj_parser::report::Report`]
    fn report(&self) -> forj_parser::report::Report {
        let cache = PreprocessorCache::new();
        let mut report = forj_parser::report::Report::new(
            self.base.kind.clone().into(),
            self.base.span.to_span_ref(&cache),
            &self.base.code,
            &self.base.msg,
        );
        for label in &self.labels {
            report = report.with_label(
                label.span.to_span_ref(&cache),
                label.kind.clone().into(),
                &label.msg,
            )
        }
        report
    }

    /// Reads in the sources needed to print the [`Report`]
    fn sources(&self) -> PyResult<impl forj_parser::report::Cache<String>> {
        let mut name_content_pairs = self.additional_sources.clone();
        // Base content
        if !name_content_pairs
            .iter()
            .any(|(file_name, _)| *file_name == self.base.span.file)
        {
            let base_content = fs::read_to_string(&self.base.span.file)
                .map_err(|err| {
                    pyo3::exceptions::PyIOError::new_err(err.to_string())
                })?;
            name_content_pairs
                .push((self.base.span.file.clone(), base_content));
        }
        // Label content
        for label in &self.labels {
            if !name_content_pairs
                .iter()
                .any(|(file_name, _)| *file_name == label.span.file)
            {
                let label_content = fs::read_to_string(&label.span.file)
                    .map_err(|err| {
                        pyo3::exceptions::PyIOError::new_err(err.to_string())
                    })?;
                name_content_pairs
                    .push((label.span.file.clone(), label_content));
            }
        }
        Ok(name_content_pairs.sources())
    }
}

#[pymethods]
impl Report {
    /// Create a new [`Report`] indicating a particular location with a message
    ///
    /// This does not label/print the location; to do so, use [`Report::label`]
    #[new]
    pub fn new(
        kind: ReportKind,
        span: Span,
        code: String,
        msg: String,
    ) -> Self {
        Self {
            base: ReportBase {
                kind: kind.clone(),
                span,
                code,
                msg,
            },
            labels: vec![],
            additional_sources: vec![],
        }
    }

    /// Include additional sources to use when printing
    pub fn include(&mut self, file_name: String, file_content: String) {
        self.additional_sources.push((file_name, file_content))
    }

    /// Adds a label to the [`Report`]
    ///
    /// A label includes a [`Span`] to highlight, as well as a message to
    /// attach at that location
    pub fn label(&mut self, span: Span, kind: ReportKind, msg: String) {
        self.labels.push(ReportLabel { span, kind, msg })
    }

    /// Print the report to `stdout`
    pub fn print(&self) -> PyResult<()> {
        let report = self.report();
        report.print(&mut self.sources()?).map_err(|err| {
            pyo3::exceptions::PyIOError::new_err(err.to_string())
        })
    }

    /// Print the report to `stderr`
    pub fn eprint(&self) -> PyResult<()> {
        let report = self.report();
        report.eprint(&mut self.sources()?).map_err(|err| {
            pyo3::exceptions::PyIOError::new_err(err.to_string())
        })
    }
}

// -----------------------------------------------------------------------
// VerboseError
// -----------------------------------------------------------------------

impl crate::VerboseError {
    /// Generate an error report for the [`VerboseError`]
    pub(crate) fn report_with_code(&self, code: String) -> PyResult<Report> {
        let error_span = if self.found.is_none() {
            let file_len = std::fs::metadata(&self.span.file)
                .map_err(|err| {
                    pyo3::exceptions::PyIOError::new_err(err.to_string())
                })?
                .len();
            let byte_span = std::ops::Range {
                start: file_len as usize,
                end: file_len as usize,
            };
            Span {
                file: self.span.file.clone(),
                bytes: byte_span.into(),
                expanded_from: self.span.expanded_from.clone(),
                included_from: self.span.included_from.clone(),
            }
        } else {
            self.span.clone()
        };
        let mut report = Report::new(
            ReportKind::Error(),
            error_span.clone(),
            code,
            self.to_string(),
        );
        report.label(
            error_span,
            ReportKind::Error(),
            match &self.found {
                Some(tok) => format!("Didn't expect {}", tok),
                None => "Didn't expect end of input".to_owned(),
            },
        );
        Ok(report)
    }
}

#[pymethods]
impl crate::VerboseError {
    /// Generate an error report for the [`VerboseError`]
    fn report(&self) -> PyResult<Report> {
        self.report_with_code("P1".to_owned())
    }
}

// -----------------------------------------------------------------------
// PreprocessorError
// -----------------------------------------------------------------------

#[pymethods]
impl PreprocessorError {
    pub fn report(&self) -> PyResult<Report> {
        let report = match self {
            PreprocessorError::Endif { endif_span } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    endif_span.clone(),
                    "PP1".to_owned(),
                    "Unexpected `endif".to_owned(),
                );
                report.label(
                    endif_span.clone(),
                    ReportKind::Error(),
                    "Unexpected `endif".to_owned(),
                );
                report
            }
            PreprocessorError::NoEndif {
                cond_token,
                cond_token_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    cond_token_span.clone(),
                    "PP2".to_owned(),
                    format!("No matching `endif for {cond_token}"),
                );
                report.label(
                    cond_token_span.clone(),
                    ReportKind::Error(),
                    "No matching `endif".to_owned(),
                );
                report
            }
            PreprocessorError::Elsif { elsif_span } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    elsif_span.clone(),
                    "PP3".to_owned(),
                    "Unexpected `elsif".to_owned(),
                );
                report.label(
                    elsif_span.clone(),
                    ReportKind::Error(),
                    "Unexpected `elsif".to_owned(),
                );
                report
            }
            PreprocessorError::Else { else_span } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    else_span.clone(),
                    "PP4".to_owned(),
                    "Unexpected `else".to_owned(),
                );
                report.label(
                    else_span.clone(),
                    ReportKind::Error(),
                    "Unexpected `else".to_owned(),
                );
                report
            }
            PreprocessorError::EndKeywords { end_keywords_span } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    end_keywords_span.clone(),
                    "PP5".to_owned(),
                    "`end_keywords with no previous `begin_keywords".to_owned(),
                );
                report.label(
                    end_keywords_span.clone(),
                    ReportKind::Error(),
                    "No matching `begin_keywords".to_owned(),
                );
                report
            }
            PreprocessorError::NoEndKeywords {
                begin_keywords_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    begin_keywords_span.clone(),
                    "PP6".to_owned(),
                    "`begin_keywords with no matching `end_keywords".to_owned(),
                );
                report.label(
                    begin_keywords_span.clone(),
                    ReportKind::Error(),
                    "No matching `end_keywords".to_owned(),
                );
                report
            }
            PreprocessorError::InvalidDefineParameter {
                other_token,
                other_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    other_span.clone(),
                    "PP7".to_owned(),
                    format!(
                        concat!(
                            "Found {}, expected a preprocessor ",
                            "macro parameter/identifier"
                        ),
                        other_token
                    ),
                );
                report.label(
                    other_span.clone(),
                    ReportKind::Error(),
                    format!("Unexpected {}", other_token),
                );
                report
            }
            PreprocessorError::InvalidDefineArgument {
                other_token,
                other_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    other_span.clone(),
                    "PP7".to_owned(),
                    format!(
                        concat!(
                            "Found {}, expected a comma, ), ",
                            "or a preprocessor macro argument"
                        ),
                        other_token
                    ),
                );
                report.label(
                    other_span.clone(),
                    ReportKind::Error(),
                    format!("Unexpected {}", other_token),
                );
                report
            }
            PreprocessorError::InvalidVersionSpecifier {
                invalid_version,
                invalid_version_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    invalid_version_span.clone(),
                    "PP8".to_owned(),
                    match invalid_version {
                        crate::Token::StringLiteral { text } => {
                            format!("{} is not a valid version specifier", text)
                        }
                        _ => {
                            format!(
                                "{} isn't a version specifier",
                                invalid_version
                            )
                        }
                    },
                );
                report.label(
                    invalid_version_span.clone(),
                    ReportKind::Error(),
                    "Invalid version specifier".to_owned(),
                );
                report
            }
            PreprocessorError::IncompleteDirective { directive_span } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    directive_span.clone(),
                    "PP9".to_owned(),
                    "Incomplete directive".to_owned(),
                );
                report.label(
                    directive_span.clone(),
                    ReportKind::Error(),
                    "Expected a complete directive".to_owned(),
                );
                report
            }
            PreprocessorError::IncompleteDefine {
                other_token,
                other_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    other_span.clone(),
                    "PP10".to_owned(),
                    format!(
                        "Found {}, expected more in the preprocessor definition",
                        other_token
                    ),
                );
                report.label(
                    other_span.clone(),
                    ReportKind::Error(),
                    "Expected more after".to_owned(),
                );
                report
            }
            PreprocessorError::UndefinedMacro {
                undefined_name,
                undefined_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    undefined_span.clone(),
                    "PP11".to_owned(),
                    format!("{undefined_name} has not been previously defined"),
                );
                report.label(
                    undefined_span.clone(),
                    ReportKind::Error(),
                    "Not previously defined".to_owned(),
                );
                report
            }
            PreprocessorError::RedefinedMacro {
                macro_name,
                redef_span,
                prev_def_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Warning(),
                    redef_span.clone(),
                    "PP12".to_owned(),
                    format!("Redefining {macro_name}"),
                );
                report.label(
                    prev_def_span.clone(),
                    ReportKind::Note(),
                    "Previously defined here".to_owned(),
                );
                report.label(
                    redef_span.clone(),
                    ReportKind::Warning(),
                    "Redefined here".to_owned(),
                );
                report
            }
            PreprocessorError::NotPreviouslyDefinedMacro {
                macro_name,
                macro_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Warning(),
                    macro_span.clone(),
                    "PP13".to_owned(),
                    format!(
                        "Undefining {}, which has not been previously defined",
                        macro_name
                    ),
                );
                report.label(
                    macro_span.clone(),
                    ReportKind::Warning(),
                    "Not previously defined".to_owned(),
                );
                report
            }
            PreprocessorError::DuplicateMacroParameter {
                define_name,
                param_name,
                dup_span,
                prev_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    dup_span.clone(),
                    "PP14".to_owned(),
                    format!(
                        "'{}' was already declared as a macro parameter for {}",
                        param_name, define_name
                    ),
                );
                report.label(
                    prev_span.clone(),
                    ReportKind::Note(),
                    "Previously declared here".to_owned(),
                );
                report.label(
                    dup_span.clone(),
                    ReportKind::Error(),
                    "Duplicate parameter declaration".to_owned(),
                );
                report
            }
            PreprocessorError::NoDefaultAfterDefault {
                default_param,
                default_param_span,
                non_default_param,
                non_default_param_span,
            } => {
                let mut report = Report::new(
                ReportKind::Error(),
                non_default_param_span.clone(),
                "PP15".to_owned(),
                "No default specified for argument after one with a default".to_owned(),
            );
                report.label(
                    default_param_span.clone(),
                    ReportKind::Note(),
                    format!("{} had a default specified", default_param),
                );
                report.label(
                    non_default_param_span.clone(),
                    ReportKind::Error(),
                    format!("No default specified for {}", non_default_param),
                );
                report
            }
            PreprocessorError::NoMacroArguments {
                macro_name,
                define_span,
                use_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    use_span.clone(),
                    "PP16".to_owned(),
                    format!("Expected arguments when using {macro_name}"),
                );
                report.label(
                    define_span.clone(),
                    ReportKind::Note(),
                    "Macro defined here".to_owned(),
                );
                report.label(
                    use_span.clone(),
                    ReportKind::Error(),
                    "Expected arguments not present".to_owned(),
                );
                report
            }
            PreprocessorError::TooManyMacroArguments {
                macro_name,
                define_span,
                use_span,
                expected,
                found,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    use_span.clone(),
                    "PP17".to_owned(),
                    format!(
                        "{} expected {} arguments, but {} were provided",
                        macro_name, expected, found
                    ),
                );
                report.label(
                    define_span.clone(),
                    ReportKind::Note(),
                    format!("Macro definition expects {expected} arguments"),
                );
                report.label(
                    use_span.clone(),
                    ReportKind::Error(),
                    format!("{found} arguments provided"),
                );
                report
            }
            PreprocessorError::MissingMacroArgument {
                define_span,
                use_span,
                param_name,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    use_span.clone(),
                    "PP18".to_owned(),
                    format!(
                        "'{param_name}' wasn't specified and has no default"
                    ),
                );
                report.label(
                    define_span.clone(),
                    ReportKind::Note(),
                    "Macro defined here".to_owned(),
                );
                report.label(
                    use_span.clone(),
                    ReportKind::Error(),
                    "Missing argument".to_owned(),
                );
                report
            }
            PreprocessorError::InvalidIdentifierFormation {
                param_name,
                arg_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    arg_span.clone(),
                    "PP19".to_owned(),
                    format!(
                        concat!(
                            "The argument for '{}' cannot be ",
                            "concatenated into an identifier"
                        ),
                        param_name
                    ),
                );
                report.label(
                    arg_span.clone(),
                    ReportKind::Error(),
                    "No valid conversion to identifier".to_owned(),
                );
                report
            }
            PreprocessorError::InvalidRelativeTimescales { timescale_span } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    timescale_span.clone(),
                    "PP20".to_owned(),
                    "Time precision is larger than the time unit".to_owned(),
                );
                report.label(
                    timescale_span.clone(),
                    ReportKind::Error(),
                    "Cannot have delay unit be smaller than precision"
                        .to_owned(),
                );
                report
            }
            PreprocessorError::IncompleteMacroWithToken {
                error_token,
                error_span,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    error_span.clone(),
                    "PP21".to_owned(),
                    format!(
                        "Usage of {} resulted in an incomplete macro",
                        error_token
                    ),
                );
                report.label(
                    error_span.clone(),
                    ReportKind::Error(),
                    concat!(
                        "Expected a complete macro argument or ",
                        "escaped newline after"
                    )
                    .to_owned(),
                );
                report
            }
            PreprocessorError::Include {
                include_path,
                include_path_span,
                read_err,
            } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    include_path_span.clone(),
                    "PP22".to_owned(),
                    format!("Error when reading {}", include_path),
                );
                report.label(
                    include_path_span.clone(),
                    ReportKind::Error(),
                    read_err.to_string(),
                );
                report
            }
            PreprocessorError::IncludeDepth { include_span } => {
                let mut report = Report::new(
                    ReportKind::Error(),
                    include_span.clone(),
                    "PP23".to_owned(),
                    "Max include depth reached".to_owned(),
                );
                report.label(
                    include_span.clone(),
                    ReportKind::Error(),
                    "Check for an `include loop".to_owned(),
                );
                report
            }
            PreprocessorError::VerboseError { err } => {
                return err.report_with_code("PP24".to_owned());
            }
        };
        Ok(report)
    }
}
