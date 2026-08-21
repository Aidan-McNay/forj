// =======================================================================
// error.rs
// =======================================================================
//! Warnings/errors that are thrown by the preprocessor

use crate::{preprocessor::include::MAX_INCLUDE_DEPTH, report::Report, *};
use std::io;

const NOTE_COLOR: Color = Color::Fixed(81);
const NOTE_KIND: ariadne::ReportKind<'static> =
    crate::report::ReportKind::Custom("note", NOTE_COLOR);

/// An error encountered during preprocessing
///
/// As preprocessing can affect the interpretation of later
/// source code, these errors are often irrecoverable
///
/// Errors marked with **INTERNAL** are meant for use inside the
/// preprocessor for passing information, and should not be returned
#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessorError<'a> {
    /// An `` `endif `` encountered outside a conditional preprocessor block
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `endif
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(),&cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::Endif{ .. })));
    /// ```
    Endif {
        /// The [`Span`] of the `` `endif ``
        endif_span: Span<'a>,
    },
    /// No terminating `` `endif `` for a conditional preprocessor block
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `ifdef TEST
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::NoEndif{
    ///     cond_token: Token::DirIfdef,
    ///     ..
    /// })));
    /// ```
    NoEndif {
        /// The conditional token (either [`Token::DirIfdef`], [`Token::DirIfndef`],
        /// [`Token::DirElsif`], or [`Token::DirElse`]) with no matching `` `endif ``
        cond_token: Token<'a>,
        /// The [`Span`] of the conditional token
        cond_token_span: Span<'a>,
    },
    /// An `` `elsif `` encountered outside a conditional preprocessor block
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `elsif
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::Elsif{ .. })));
    /// ```
    Elsif {
        /// The [`Span`] of the `` `elsif ``
        elsif_span: Span<'a>,
    },
    /// An `` `else `` encountered outside a conditional preprocessor block
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `else
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::Else{ .. })));
    /// ```
    Else {
        /// The [`Span`] of the `` `else ``
        else_span: Span<'a>,
    },
    /// An `` `end_keywords `` encountered outside a `` `begin_keywords `` block
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `end_keywords
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::EndKeywords{ .. })));
    /// ```
    EndKeywords {
        /// The [`Span`] of the `` `end_keywords ``
        end_keywords_span: Span<'a>,
    },
    /// No terminating `` `end_keywords `` for a `` `begin_keywords `` block
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `begin_keywords \"1800-2009\"
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::NoEndKeywords{ .. })));
    /// ```
    NoEndKeywords {
        /// The [`Span`] of the unterminated `` `begin_keywords ``
        begin_keywords_span: Span<'a>,
    },
    /// A `` `define `` attempted to override a compiler directive
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define ifdef 'whoops'
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::RedefinedDirective{
    ///     directive_name: "ifdef",
    ///     ..
    /// })));
    /// ```
    RedefinedDirective {
        /// The compiler directive that was being overriden
        directive_name: &'a str,
        /// The [`Span`] of the name specification
        directive_span: Span<'a>,
    },
    /// A missing parameter in a `` `define `` function declaration where one is expected
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST()
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::InvalidDefineParameter{
    ///     other_token: Token::EParen,
    ///     ..
    /// })));
    /// ```
    InvalidDefineParameter {
        /// The [`Token`] found instead of the `` `define `` parameter
        other_token: Token<'a>,
        /// The [`Span`] of the token found instead
        other_span: Span<'a>,
    },
    /// A missing or invalid argument specification in a `` `define `` function
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b c)
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::InvalidDefineArgument{
    ///     other_token: Token::SimpleIdentifier("c"),
    ///     ..
    /// })));
    /// ```
    InvalidDefineArgument {
        /// The [`Token`] found instead of the valid `` `define `` argument
        other_token: Token<'a>,
        /// The [`Span`] of the token found instead
        other_span: Span<'a>,
    },
    /// An invalid version specifier for a `` `begin_keywords `` directive
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `begin_keywords \"MyVersion\"
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::InvalidVersionSpecifier{
    ///     invalid_version: Token::StringLiteral("MyVersion"),
    ///     ..
    /// })));
    /// ```
    InvalidVersionSpecifier {
        /// The [`Token`] provided instead of a valid version specifier
        ///
        /// If the token is a [`Token::StringLiteral`], the string isn't a version recognized
        /// by 1800-2023
        invalid_version: Token<'a>,
        /// The [`Span`] of the invalid version specifier
        invalid_version_span: Span<'a>,
    },
    /// A directive that doesn't have all of the required components
    ///
    /// In general, [`PreprocessorError::VerboseError`] is preferred, but may
    /// not be suitable due to a lack of subsequent tokens
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "`line";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::IncompleteDirective{ .. })));
    /// ```
    IncompleteDirective {
        /// The [`Span`] of the incomplete preprocessor directive
        ///
        /// This is usually the primary directive, but can be other more indicative
        /// tokens as well, such as an unmatched opening parenthesis
        directive_span: Span<'a>,
    },
    /// An incomplete preprocessor definition, specifically with function macro arguments
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::IncompleteDefine{
    ///     other_token: Token::Paren,
    ///     ..
    /// })));
    /// ```
    IncompleteDefine {
        /// If known, the [`Token`] found instead of a valid function macro argument specification
        ///
        /// In the case that the token wasn't tracked, the opening [`Token::Paren`] is referenced
        /// instead
        other_token: Token<'a>,
        /// The [`Span`] of the token found instead
        other_span: Span<'a>,
    },
    /// Use of a text macro that wasn't previously defined
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `TEST
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::UndefinedMacro{
    ///     undefined_name: "TEST",
    ///     ..
    /// })));
    /// ```
    UndefinedMacro {
        /// The name of the undefined macro
        undefined_name: &'a str,
        /// The [`Span`] of the undefined macro
        undefined_span: Span<'a>,
    },
    /// Specifying a macro parameter that was already specified
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b, a) a + b
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::DuplicateMacroParameter{
    ///     define_name: "TEST",
    ///     param_name: "a",
    ///     ..
    /// })));
    /// ```
    DuplicateMacroParameter {
        /// The name of the macro for which duplicate parameters were specified
        define_name: &'a str,
        /// The name of the parameter that was specified multiple times
        param_name: &'a str,
        /// The [`Span`] of the duplicate specification
        dup_span: Span<'a>,
        /// The [`Span`] of the previous/original specification
        prev_span: Span<'a>,
    },
    /// Attempting to have a macro parameter with no default value after one that does
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a = 1, b) a + b
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::NoDefaultAfterDefault{
    ///     default_param: "a",
    ///     non_default_param: "b",
    ///     ..
    /// })));
    /// ```
    NoDefaultAfterDefault {
        /// The name of the previously-specified default parameter
        default_param: &'a str,
        /// The [`Span`] of the previously-specified default parameter
        default_param_span: Span<'a>,
        /// The name of the non-default parameter
        non_default_param: &'a str,
        /// The [`Span`] of the non-default parameter
        non_default_param_span: Span<'a>,
    },
    /// Specifying no arguments for a macro function that takes arguments
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a + b
    /// `TEST
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::NoMacroArguments{
    ///     macro_name: "TEST",
    ///     ..
    /// })));
    /// ```
    NoMacroArguments {
        /// The name of the macro
        macro_name: &'a str,
        /// The [`Span`] of the macro definition (with arguments)
        define_span: Span<'a>,
        /// The [`Span`] where the macro was used with no arguments
        use_span: Span<'a>,
    },
    /// Specifying too many arguments for a macro function
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a + b
    /// `TEST(1, 2, 3)
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::TooManyMacroArguments{
    ///     macro_name: "TEST",
    ///     expected: 2,
    ///     found: 3,
    ///     ..
    /// })));
    /// ```
    TooManyMacroArguments {
        /// The name of the macro
        macro_name: &'a str,
        /// The [`Span`] of the macro definition
        define_span: Span<'a>,
        /// The [`Span`] where the macro was used with too many arguments
        use_span: Span<'a>,
        /// How many arguments were expected
        expected: usize,
        /// How many arguments were found
        found: usize,
    },
    /// Missing an argument in a macro function use
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a + b
    /// `TEST(1)
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::MissingMacroArgument{
    ///     param_name: "b",
    ///     ..
    /// })));
    /// ```
    MissingMacroArgument {
        /// The [`Span`] of the macro definition
        define_span: Span<'a>,
        /// The [`Span`] where the macro was used with a missing argument
        use_span: Span<'a>,
        /// The name of the missing parameter
        param_name: &'a str,
    },
    /// An invalid preprocessor identifier specification
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a``_with_``b
    /// `TEST(\"one\", \"two\")
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::InvalidIdentifierFormation{
    ///     param_name: "a",
    ///     ..
    /// })));
    /// ```
    InvalidIdentifierFormation {
        /// The name of the parameter used in a preprocessor identifier
        param_name: &'a str,
        /// The [`Span`] of the invalid argument
        arg_span: Span<'a>,
    },
    /// A precision that is less precise than the unit in a `` `timescale `` directive
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `timescale 100 fs / 1 s
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::InvalidRelativeTimescales{ .. })));
    /// ```
    InvalidRelativeTimescales {
        /// The [`Span`] of the `` `timescale `` directive
        timescale_span: Span<'a>,
    },
    /// An incomplete macro due to mismatching grouping tokens (`[]`, `()`, or `{}`)
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a + b
    /// `TEST(a = 1, b = 2])
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::IncompleteMacroWithToken{
    ///     error_token: Token::EBracket,
    ///     ..
    /// })));
    /// ```
    IncompleteMacroWithToken {
        /// The error-causing [`Token`] (either [`Token::EParen`],
        /// [`Token::EBracket`], or [`Token::EBrace`])
        error_token: Token<'a>,
        /// The error-causing [`Span`]
        error_span: Span<'a>,
    },
    /// An error reading a file specified by an  `` `include `` macro
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `include \"other.v\"
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::Include{
    ///     include_path: "other.v",
    ///     ..
    /// })));
    /// ```
    Include {
        /// The path for the `` `include `` directive
        include_path: &'a str,
        /// The [`Span`] of the include path
        include_path_span: Span<'a>,
        /// The [`io::ErrorKind`] raised when attempting to read the file
        read_err: io::ErrorKind,
    },
    /// The maximum include depth was hit, likely as a result of a self-referential
    /// `` `include `` sequence
    ///
    /// ```no_run
    /// # // No test.v in the file system
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `include \"test.v\"
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::IncludeDepth{ .. })));
    /// ```
    IncludeDepth {
        /// The [`Span`] of the `` `include `` directive that exceeded the limit
        include_span: Span<'a>,
    },
    /// A [`VerboseError`] detailing the expected and found tokens, for a case not covered above
    ///
    /// This is most commonly used when we can provide the user with a bit more context
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `line
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// // Expects a line number
    /// assert!(preprocess_result.is_err());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::VerboseError{
    ///     err: VerboseError{
    ///         found: Some(Token::Newline),
    ///         ..
    ///     }
    /// })));
    /// ```
    VerboseError {
        /// The [`VerboseError`] for the preprocessor error
        err: VerboseError<'a>,
    },
    /// Attempted to `` `undef `` a macro that had no previous definition
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `undef TEST
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_ok());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::NotPreviouslyDefinedMacro{
    ///     macro_name: "TEST",
    ///     ..
    /// })))
    /// ```
    NotPreviouslyDefinedMacro {
        /// The name that wasn't previously defined
        macro_name: &'a str,
        /// The [`Span`] where the not-previously-defined name was specified
        macro_span: Span<'a>,
    },
    /// A redefinition of a text macro that was previously defined
    ///
    /// ```rust
    /// # use forj_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST definition_one
    /// `define TEST definition_two
    /// ";
    /// state.retain_file("test.v".to_string(), source.to_string(), &cache);
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     input,
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(preprocess_result.is_ok());
    /// assert!(matches!(state.errors.first(), Some(PreprocessorError::RedefinedMacro{
    ///     macro_name: "TEST",
    ///     ..
    /// })))
    /// ```
    RedefinedMacro {
        /// The name of the macro being redefined
        macro_name: &'a str,
        /// The [`Span`] of the redefinition
        redef_span: Span<'a>,
        /// The [`Span`] where the macro was previously defined
        prev_def_span: Span<'a>,
    },
    // Internal "errors" used for communication
    // - Should not be exposed outside of main preprocess function
    /// **INTERNAL**: A newline encountered in a `` `define `` directive
    NewlineInDefine(Span<'a>),
    /// **INTERNAL**: The end of a function argument was encountered
    EndOfFunctionArgument(SpannedToken<'a>),
}

impl<'a> PreprocessorError<'a> {
    /// Whether the given [`PreprocessorError`] is just a warning
    ///
    /// Warnings reflect an irregularity in the source code, but are
    /// still well-defined and allow preprocessing to continue
    pub fn is_warning(&self) -> bool {
        match self {
            PreprocessorError::NotPreviouslyDefinedMacro { .. }
            | PreprocessorError::RedefinedMacro { .. } => true,
            _ => false,
        }
    }
}

impl<'s> From<&PreprocessorError<'s>> for Report {
    fn from(s: &PreprocessorError<'s>) -> Self {
        match s {
            PreprocessorError::Endif { endif_span } => Report::new(
                report::ReportKind::Error,
                &endif_span,
                "PP1",
                "Unexpected `endif",
            )
            .with_label(
                &endif_span,
                report::ReportKind::Error,
                "Unexpected `endif",
            ),
            PreprocessorError::NoEndif {
                cond_token,
                cond_token_span,
            } => Report::new(
                report::ReportKind::Error,
                &cond_token_span,
                "PP2",
                format!("No matching `endif for {cond_token}"),
            )
            .with_label(
                &cond_token_span,
                report::ReportKind::Error,
                "No matching `endif",
            ),
            PreprocessorError::Elsif { elsif_span } => Report::new(
                report::ReportKind::Error,
                &elsif_span,
                "PP3",
                "Unexpected `elsif",
            )
            .with_label(
                &elsif_span,
                report::ReportKind::Error,
                "Unexpected `elsif",
            ),
            PreprocessorError::Else { else_span } => Report::new(
                report::ReportKind::Error,
                &else_span,
                "PP4",
                "Unexpected `else",
            )
            .with_label(
                &else_span,
                report::ReportKind::Error,
                "Unexpected `else",
            ),
            PreprocessorError::EndKeywords { end_keywords_span } => {
                Report::new(
                    report::ReportKind::Error,
                    &end_keywords_span,
                    "PP5",
                    "`end_keywords with no previous `begin_keywords",
                )
                .with_label(
                    &end_keywords_span,
                    report::ReportKind::Error,
                    "No matching `begin_keywords",
                )
            }
            PreprocessorError::NoEndKeywords {
                begin_keywords_span,
            } => Report::new(
                report::ReportKind::Error,
                &begin_keywords_span,
                "PP6",
                "`begin_keywords with no matching `end_keywords",
            )
            .with_label(
                &begin_keywords_span,
                report::ReportKind::Error,
                "No matching `end_keywords",
            ),
            PreprocessorError::RedefinedDirective {
                directive_name,
                directive_span,
            } => Report::new(
                report::ReportKind::Error,
                &directive_span,
                "PP7",
                format!(
                    "Attempted to redefine compiler directive '{}'",
                    directive_name
                ),
            )
            .with_label(
                &directive_span,
                report::ReportKind::Error,
                format!("Redefining {}", directive_name),
            ),
            PreprocessorError::InvalidDefineParameter {
                other_token,
                other_span,
            } => Report::new(
                report::ReportKind::Error,
                &other_span,
                "PP8",
                format!(
                    concat!(
                        "Found {}, expected a preprocessor ",
                        "macro parameter/identifier"
                    ),
                    other_token
                ),
            )
            .with_label(
                &other_span,
                report::ReportKind::Error,
                format!("Unexpected {}", other_token),
            ),
            PreprocessorError::InvalidDefineArgument {
                other_token,
                other_span,
            } => Report::new(
                report::ReportKind::Error,
                &other_span,
                "PP9",
                format!(
                    concat!(
                        "Found {}, expected a comma, ), ",
                        "or a preprocessor macro argument"
                    ),
                    other_token
                ),
            )
            .with_label(
                &other_span,
                report::ReportKind::Error,
                format!("Unexpected {}", other_token),
            ),
            PreprocessorError::InvalidVersionSpecifier {
                invalid_version,
                invalid_version_span,
            } => Report::new(
                report::ReportKind::Error,
                &invalid_version_span,
                "PP10",
                match invalid_version {
                    Token::StringLiteral(invalid_version_str) => format!(
                        "{} is not a valid version specifier",
                        invalid_version_str
                    ),
                    _ => {
                        format!("{} isn't a version specifier", invalid_version)
                    }
                },
            )
            .with_label(
                &invalid_version_span,
                report::ReportKind::Error,
                "Invalid version specifier",
            ),
            PreprocessorError::IncompleteDirective { directive_span } => {
                Report::new(
                    report::ReportKind::Error,
                    &directive_span,
                    "PP11",
                    "Incomplete directive",
                )
                .with_label(
                    &directive_span,
                    report::ReportKind::Error,
                    "Expected a complete directive",
                )
            }
            PreprocessorError::IncompleteDefine {
                other_token,
                other_span,
            } => Report::new(
                report::ReportKind::Error,
                &other_span,
                "PP12",
                format!(
                    "Found {}, expected more in the preprocessor definition",
                    other_token
                ),
            )
            .with_label(
                &other_span,
                report::ReportKind::Error,
                "Expected more after",
            ),
            PreprocessorError::UndefinedMacro {
                undefined_name,
                undefined_span,
            } => Report::new(
                report::ReportKind::Error,
                &undefined_span,
                "PP13",
                format!("{undefined_name} has not been previously defined"),
            )
            .with_label(
                &undefined_span,
                report::ReportKind::Error,
                "Not previously defined",
            ),
            PreprocessorError::RedefinedMacro {
                macro_name,
                redef_span,
                prev_def_span,
            } => Report::new(
                report::ReportKind::Warning,
                &redef_span,
                "PP14",
                format!("Redefining {macro_name}"),
            )
            .with_label(&prev_def_span, NOTE_KIND, "Previously defined here")
            .with_label(
                &redef_span,
                report::ReportKind::Warning,
                "Redefined here",
            ),
            PreprocessorError::NotPreviouslyDefinedMacro {
                macro_name,
                macro_span,
            } => Report::new(
                report::ReportKind::Warning,
                &macro_span,
                "PP15",
                format!(
                    "Undefining {}, which has not been previously defined",
                    macro_name
                ),
            )
            .with_label(
                &macro_span,
                report::ReportKind::Warning,
                "Not previously defined",
            ),
            PreprocessorError::DuplicateMacroParameter {
                define_name,
                param_name,
                dup_span,
                prev_span,
            } => Report::new(
                report::ReportKind::Error,
                &dup_span,
                "PP16",
                format!(
                    "'{}' was already declared as a macro parameter for {}",
                    param_name, define_name
                ),
            )
            .with_label(&prev_span, NOTE_KIND, "Previously declared here")
            .with_label(
                &dup_span,
                report::ReportKind::Error,
                "Duplicate parameter declaration",
            ),
            PreprocessorError::NoDefaultAfterDefault {
                default_param,
                default_param_span,
                non_default_param,
                non_default_param_span,
            } => Report::new(
                report::ReportKind::Error,
                &non_default_param_span,
                "PP17",
                "No default specified for argument after one with a default",
            )
            .with_label(
                &default_param_span,
                NOTE_KIND,
                format!("{} had a default specified", default_param),
            )
            .with_label(
                &non_default_param_span,
                report::ReportKind::Error,
                format!("No default specified for {}", non_default_param),
            ),
            PreprocessorError::NoMacroArguments {
                macro_name,
                define_span,
                use_span,
            } => Report::new(
                report::ReportKind::Error,
                &use_span,
                "PP18",
                format!("Expected arguments when using {macro_name}"),
            )
            .with_label(&define_span, NOTE_KIND, "Macro defined here")
            .with_label(
                &use_span,
                report::ReportKind::Error,
                "Expected arguments not present",
            ),
            PreprocessorError::TooManyMacroArguments {
                macro_name,
                define_span,
                use_span,
                expected,
                found,
            } => Report::new(
                report::ReportKind::Error,
                &use_span,
                "PP19",
                format!(
                    "{} expected {} arguments, but {} were provided",
                    macro_name, expected, found
                ),
            )
            .with_label(
                &define_span,
                NOTE_KIND,
                format!("Macro definition expects {expected} arguments"),
            )
            .with_label(
                &use_span,
                report::ReportKind::Error,
                format!("{found} arguments provided"),
            ),
            PreprocessorError::MissingMacroArgument {
                define_span,
                use_span,
                param_name,
            } => Report::new(
                report::ReportKind::Error,
                &use_span,
                "PP20",
                format!("'{param_name}' wasn't specified and has no default"),
            )
            .with_label(&define_span, NOTE_KIND, "Macro defined here")
            .with_label(
                &use_span,
                report::ReportKind::Error,
                "Missing argument",
            ),
            PreprocessorError::InvalidIdentifierFormation {
                param_name,
                arg_span,
            } => Report::new(
                report::ReportKind::Error,
                &arg_span,
                "PP21",
                format!(
                    concat!(
                        "The argument for '{}' cannot be ",
                        "concatenated into an identifier"
                    ),
                    param_name
                ),
            )
            .with_label(
                &arg_span,
                report::ReportKind::Error,
                "No valid conversion to identifier",
            ),
            PreprocessorError::InvalidRelativeTimescales { timescale_span } => {
                Report::new(
                    report::ReportKind::Error,
                    &timescale_span,
                    "PP22",
                    "Time precision is larger than the time unit",
                )
                .with_label(
                    &timescale_span,
                    report::ReportKind::Error,
                    "Cannot have delay unit be smaller than precision",
                )
            }
            PreprocessorError::IncompleteMacroWithToken {
                error_token,
                error_span,
            } => Report::new(
                report::ReportKind::Error,
                &error_span,
                "PP23",
                format!(
                    "Usage of {} resulted in an incomplete macro",
                    error_token
                ),
            )
            .with_label(
                &error_span,
                report::ReportKind::Error,
                "Expected a complete macro argument or escaped newline after",
            ),
            PreprocessorError::Include {
                include_path,
                include_path_span,
                read_err,
            } => Report::new(
                report::ReportKind::Error,
                &include_path_span,
                "PP24",
                format!("Error when reading {}", include_path),
            )
            .with_label(
                &include_path_span,
                report::ReportKind::Error,
                read_err.to_string(),
            ),
            PreprocessorError::IncludeDepth { include_span } => Report::new(
                report::ReportKind::Error,
                &include_span,
                "PP25",
                format!("Max include depth of {} reached", MAX_INCLUDE_DEPTH),
            )
            .with_label(
                &include_span,
                report::ReportKind::Error,
                "Check for an `include loop",
            ),
            PreprocessorError::VerboseError { err } => err.report("PP26"),
            PreprocessorError::NewlineInDefine(newline_span) => Report::new(
                report::ReportKind::Error,
                &newline_span,
                "PPX",
                "(Internal Error) Newline in define not handled correctly",
            )
            .with_label(
                &newline_span,
                report::ReportKind::Error,
                "(Internal Error) Newline in define not handled correctly",
            ),
            PreprocessorError::EndOfFunctionArgument(err_spanned_token) => {
                Report::new(
                    report::ReportKind::Error,
                    &err_spanned_token.1,
                    "PPX",
                    concat!(
                        "(Internal Error) End of function argument ",
                        "not handled correctly"
                    ),
                )
                .with_label(
                    &err_spanned_token.1,
                    report::ReportKind::Error,
                    concat!(
                        "(Internal Error) End of function argument ",
                        "not handled correctly"
                    ),
                )
            }
        }
    }
}
