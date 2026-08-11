// =======================================================================
// error.rs
// =======================================================================
//! Making Rust errors Python-compatible

use crate::{Span, Token};
use pyo3::prelude::*;

// -----------------------------------------------------------------------
// Verbose
// -----------------------------------------------------------------------

/// An expectation instead of what was found
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub enum Expectation {
    /// A specific token
    Token { token: Token },
    /// A verbose human-readable label
    Label { label: String },
    /// The end of a file
    EOI(),
}

impl<'a> From<forj_parser::Expectation<'a>> for Expectation {
    fn from(value: forj_parser::Expectation<'a>) -> Self {
        match value {
            forj_parser::Expectation::Token(rust_token) => {
                Expectation::Token {
                    token: rust_token.into(),
                }
            }
            forj_parser::Expectation::Label(rust_str) => Expectation::Label {
                label: rust_str.to_string(),
            },
            forj_parser::Expectation::EOI => Expectation::EOI(),
        }
    }
}

impl<'a> std::fmt::Display for Expectation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expectation::Token { token } => {
                let rust_token: forj_parser::Token = token.into();
                rust_token.fmt(f)
            }
            Expectation::Label { label } => write!(f, "{}", label),
            Expectation::EOI() => write!(f, "end of input"),
        }
    }
}

/// A wrapper around [`forj_parser::VerboseError`]
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct VerboseError {
    /// The [`Span`] that the error occurred at
    #[pyo3(get, set)]
    pub span: Span,
    /// What token was found - [`None`] if the end of the file was reached
    #[pyo3(get, set)]
    pub found: Option<Token>,
    /// What was expected instead (listing all possibilities)
    #[pyo3(get, set)]
    pub expected: Vec<Expectation>,
}

impl<'a> From<forj_parser::VerboseError<'a>> for VerboseError {
    fn from(value: forj_parser::VerboseError<'a>) -> Self {
        Self {
            span: value.span.into(),
            found: match value.found {
                Some(rust_token) => Some(rust_token.into()),
                None => None,
            },
            expected: value
                .expected
                .into_iter()
                .map(|rust_expectation| rust_expectation.into())
                .collect(),
        }
    }
}

impl<'a> std::fmt::Display for VerboseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "found ")?;
        match &self.found {
            Some(tok) => tok.fmt(f)?,
            None => write!(f, "end of input")?,
        };
        write!(f, ", expected ")?;
        let mut dedup_expected: Vec<Expectation> = vec![];
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

// -----------------------------------------------------------------------
// Preprocessor
// -----------------------------------------------------------------------

/// A wrapper around [`forj_parser::PreprocessorError`]
///
/// See its documentation for details on each variant
#[pyclass(eq, from_py_object, module = "forj_python")]
#[derive(Clone, PartialEq, Eq)]
pub enum PreprocessorError {
    Endif {
        /// The [`Span`] of the `` `endif ``
        endif_span: Span,
    },
    NoEndif {
        /// The conditional token (either [`Token::DirIfdef`], [`Token::DirIfndef`],
        /// [`Token::DirElsif`], or [`Token::DirElse`]) with no matching `` `endif ``
        cond_token: Token,
        /// The [`Span`] of the conditional token
        cond_token_span: Span,
    },
    Elsif {
        /// The [`Span`] of the `` `elsif ``
        elsif_span: Span,
    },
    Else {
        /// The [`Span`] of the `` `else ``
        else_span: Span,
    },
    EndKeywords {
        /// The [`Span`] of the `` `end_keywords ``
        end_keywords_span: Span,
    },
    NoEndKeywords {
        /// The [`Span`] of the unterminated `` `begin_keywords ``
        begin_keywords_span: Span,
    },
    InvalidDefineParameter {
        /// The [`Token`] found instead of the `` `define `` parameter
        other_token: Token,
        /// The [`Span`] of the token found instead
        other_span: Span,
    },
    InvalidDefineArgument {
        /// The [`Token`] found instead of the valid `` `define `` argument
        other_token: Token,
        /// The [`Span`] of the token found instead
        other_span: Span,
    },
    InvalidVersionSpecifier {
        /// The [`Token`] provided instead of a valid version specifier
        ///
        /// If the token is a [`Token::StringLiteral`], the string isn't a version recognized
        /// by 1800-2023
        invalid_version: Token,
        /// The [`Span`] of the invalid version specifier
        invalid_version_span: Span,
    },
    IncompleteDirective {
        /// The [`Span`] of the incomplete preprocessor directive
        ///
        /// This is usually the primary directive, but can be other more indicative
        /// tokens as well, such as an unmatched opening parenthesis
        directive_span: Span,
    },
    IncompleteDefine {
        /// If known, the [`Token`] found instead of a valid function macro argument specification
        ///
        /// In the case that the token wasn't tracked, the opening [`Token::Paren`] is referenced
        /// instead
        other_token: Token,
        /// The [`Span`] of the token found instead
        other_span: Span,
    },
    UndefinedMacro {
        /// The name of the undefined macro
        undefined_name: String,
        /// The [`Span`] of the undefined macro
        undefined_span: Span,
    },
    DuplicateMacroParameter {
        /// The name of the macro for which duplicate parameters were specified
        define_name: String,
        /// The name of the parameter that was specified multiple times
        param_name: String,
        /// The [`Span`] of the duplicate specification
        dup_span: Span,
        /// The [`Span`] of the previous/original specification
        prev_span: Span,
    },
    NoDefaultAfterDefault {
        /// The name of the previously-specified default parameter
        default_param: String,
        /// The [`Span`] of the previously-specified default parameter
        default_param_span: Span,
        /// The name of the non-default parameter
        non_default_param: String,
        /// The [`Span`] of the non-default parameter
        non_default_param_span: Span,
    },
    NoMacroArguments {
        /// The name of the macro
        macro_name: String,
        /// The [`Span`] of the macro definition (with arguments)
        define_span: Span,
        /// The [`Span`] where the macro was used with no arguments
        use_span: Span,
    },
    TooManyMacroArguments {
        /// The name of the macro
        macro_name: String,
        /// The [`Span`] of the macro definition
        define_span: Span,
        /// The [`Span`] where the macro was used with too many arguments
        use_span: Span,
        /// How many arguments were expected
        expected: usize,
        /// How many arguments were found
        found: usize,
    },
    MissingMacroArgument {
        /// The [`Span`] of the macro definition
        define_span: Span,
        /// The [`Span`] where the macro was used with a missing argument
        use_span: Span,
        /// The name of the missing parameter
        param_name: String,
    },
    InvalidIdentifierFormation {
        /// The name of the parameter used in a preprocessor identifier
        param_name: String,
        /// The [`Span`] of the invalid argument
        arg_span: Span,
    },
    InvalidRelativeTimescales {
        /// The [`Span`] of the `` `timescale `` directive
        timescale_span: Span,
    },
    IncompleteMacroWithToken {
        /// The error-causing [`Token`] (either [`Token::EParen`],
        /// [`Token::EBracket`], or [`Token::EBrace`])
        error_token: Token,
        /// The error-causing [`Span`]
        error_span: Span,
    },
    Include {
        /// The path for the `` `include `` directive
        include_path: String,
        /// The [`Span`] of the include path
        include_path_span: Span,
        /// The [`std::io::Error`] raised when attempting to read the file
        read_err: String,
    },
    IncludeDepth {
        /// The [`Span`] of the `` `include `` directive that exceeded the limit
        include_span: Span,
    },
    VerboseError {
        /// The [`VerboseError`] for the preprocessor error
        err: VerboseError,
    },
    NotPreviouslyDefinedMacro {
        /// The name that wasn't previously defined
        macro_name: String,
        /// The [`Span`] where the not-previously-defined name was specified
        macro_span: Span,
    },
    RedefinedMacro {
        /// The name of the macro being redefined
        macro_name: String,
        /// The [`Span`] of the redefinition
        redef_span: Span,
        /// The [`Span`] where the macro was previously defined
        prev_def_span: Span,
    },
}

impl<'a> From<forj_parser::PreprocessorError<'a>> for PreprocessorError {
    fn from(value: forj_parser::PreprocessorError<'a>) -> Self {
        match value {
            forj_parser::PreprocessorError::Endif { endif_span } => {
                PreprocessorError::Endif {
                    endif_span: endif_span.into(),
                }
            }
            forj_parser::PreprocessorError::NoEndif {
                cond_token,
                cond_token_span,
            } => PreprocessorError::NoEndif {
                cond_token: cond_token.into(),
                cond_token_span: cond_token_span.into(),
            },
            forj_parser::PreprocessorError::Elsif { elsif_span } => {
                PreprocessorError::Elsif {
                    elsif_span: elsif_span.into(),
                }
            }
            forj_parser::PreprocessorError::Else { else_span } => {
                PreprocessorError::Else {
                    else_span: else_span.into(),
                }
            }
            forj_parser::PreprocessorError::EndKeywords {
                end_keywords_span,
            } => PreprocessorError::EndKeywords {
                end_keywords_span: end_keywords_span.into(),
            },
            forj_parser::PreprocessorError::NoEndKeywords {
                begin_keywords_span,
            } => PreprocessorError::NoEndKeywords {
                begin_keywords_span: begin_keywords_span.into(),
            },
            forj_parser::PreprocessorError::InvalidDefineParameter {
                other_token,
                other_span,
            } => PreprocessorError::InvalidDefineParameter {
                other_token: other_token.into(),
                other_span: other_span.into(),
            },
            forj_parser::PreprocessorError::InvalidDefineArgument {
                other_token,
                other_span,
            } => PreprocessorError::InvalidDefineArgument {
                other_token: other_token.into(),
                other_span: other_span.into(),
            },
            forj_parser::PreprocessorError::InvalidVersionSpecifier {
                invalid_version,
                invalid_version_span,
            } => PreprocessorError::InvalidVersionSpecifier {
                invalid_version: invalid_version.into(),
                invalid_version_span: invalid_version_span.into(),
            },
            forj_parser::PreprocessorError::IncompleteDirective {
                directive_span,
            } => PreprocessorError::IncompleteDirective {
                directive_span: directive_span.into(),
            },
            forj_parser::PreprocessorError::IncompleteDefine {
                other_token,
                other_span,
            } => PreprocessorError::IncompleteDefine {
                other_token: other_token.into(),
                other_span: other_span.into(),
            },
            forj_parser::PreprocessorError::UndefinedMacro {
                undefined_name,
                undefined_span,
            } => PreprocessorError::UndefinedMacro {
                undefined_name: undefined_name.to_string(),
                undefined_span: undefined_span.into(),
            },
            forj_parser::PreprocessorError::DuplicateMacroParameter {
                define_name,
                param_name,
                dup_span,
                prev_span,
            } => PreprocessorError::DuplicateMacroParameter {
                define_name: define_name.to_string(),
                param_name: param_name.to_string(),
                dup_span: dup_span.into(),
                prev_span: prev_span.into(),
            },
            forj_parser::PreprocessorError::NoDefaultAfterDefault {
                default_param,
                default_param_span,
                non_default_param,
                non_default_param_span,
            } => PreprocessorError::NoDefaultAfterDefault {
                default_param: default_param.to_string(),
                default_param_span: default_param_span.into(),
                non_default_param: non_default_param.to_string(),
                non_default_param_span: non_default_param_span.into(),
            },
            forj_parser::PreprocessorError::NoMacroArguments {
                macro_name,
                define_span,
                use_span,
            } => PreprocessorError::NoMacroArguments {
                macro_name: macro_name.to_string(),
                define_span: define_span.into(),
                use_span: use_span.into(),
            },
            forj_parser::PreprocessorError::TooManyMacroArguments {
                macro_name,
                define_span,
                use_span,
                expected,
                found,
            } => PreprocessorError::TooManyMacroArguments {
                macro_name: macro_name.to_string(),
                define_span: define_span.into(),
                use_span: use_span.into(),
                expected,
                found,
            },
            forj_parser::PreprocessorError::MissingMacroArgument {
                define_span,
                use_span,
                param_name,
            } => PreprocessorError::MissingMacroArgument {
                define_span: define_span.into(),
                use_span: use_span.into(),
                param_name: param_name.to_string(),
            },
            forj_parser::PreprocessorError::InvalidIdentifierFormation {
                param_name,
                arg_span,
            } => PreprocessorError::InvalidIdentifierFormation {
                param_name: param_name.to_string(),
                arg_span: arg_span.into(),
            },
            forj_parser::PreprocessorError::InvalidRelativeTimescales {
                timescale_span,
            } => PreprocessorError::InvalidRelativeTimescales {
                timescale_span: timescale_span.into(),
            },
            forj_parser::PreprocessorError::IncompleteMacroWithToken {
                error_token,
                error_span,
            } => PreprocessorError::IncompleteMacroWithToken {
                error_token: error_token.into(),
                error_span: error_span.into(),
            },
            forj_parser::PreprocessorError::Include {
                include_path,
                include_path_span,
                read_err,
            } => PreprocessorError::Include {
                include_path: include_path.to_string(),
                include_path_span: include_path_span.into(),
                read_err: read_err.to_string(),
            },
            forj_parser::PreprocessorError::IncludeDepth { include_span } => {
                PreprocessorError::IncludeDepth {
                    include_span: include_span.into(),
                }
            }
            forj_parser::PreprocessorError::VerboseError { err } => {
                PreprocessorError::VerboseError { err: err.into() }
            }
            forj_parser::PreprocessorError::RedefinedMacro {
                macro_name,
                redef_span,
                prev_def_span,
            } => PreprocessorError::RedefinedMacro {
                macro_name: macro_name.to_string(),
                redef_span: redef_span.into(),
                prev_def_span: prev_def_span.into(),
            },
            forj_parser::PreprocessorError::NotPreviouslyDefinedMacro {
                macro_name,
                macro_span,
            } => PreprocessorError::NotPreviouslyDefinedMacro {
                macro_name: macro_name.to_string(),
                macro_span: macro_span.into(),
            },
            forj_parser::PreprocessorError::EndOfFunctionArgument(_)
            | forj_parser::PreprocessorError::NewlineInDefine(_) => panic!(
                "Unexpected internal error encountered during preprocessing"
            ),
        }
    }
}
