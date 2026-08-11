// =======================================================================
// state.rs
// =======================================================================
//! The state and setup of the preprocessor

use crate::*;
use forj_syntax::SpanRelation;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_TIMESCALE: Timescale = Timescale::new_default(
    (TimescaleValue::One, TimescaleUnit::NS),
    (TimescaleValue::One, TimescaleUnit::NS),
);

const DEFAULT_NETTYPE: DefaultNettype = DefaultNettype::Wire;

const DEFAULT_UNCONNECTED_DRIVE: UnconnectedDrive =
    UnconnectedDrive::NoUnconnected;

/// A preprocessor definition
#[derive(Clone, Debug)]
pub struct Define<'a> {
    pub name: SpannedString<'a>,
    pub body: DefineBody<'a>,
}

impl<'a> Define<'a> {
    /// Whether the define is from the command line
    ///
    /// CLI defines have an empty `file` for their [`Span`]
    pub fn is_from_command_line(&self) -> bool {
        self.name.1.file == ""
    }
}

impl<'a> From<&'a str> for Define<'a> {
    fn from(value: &'a str) -> Self {
        let mut components = value.splitn(2, "=");
        let name = SpannedString(components.next().unwrap(), Span::default());
        let body = match components.next() {
            Some(text) => DefineBody::Text(lex(text, "").tokens().collect()),
            None => DefineBody::Empty,
        };
        Self { name, body }
    }
}

/// The body of a preprocessor definition
///
/// Defines can either be
/// - Empty (no associated text)
/// - Some sequence of tokens
/// - A function (see [`DefineFunction`])
#[derive(Clone, Debug)]
pub enum DefineBody<'a> {
    Empty,
    Text(Vec<SpannedToken<'a>>),
    Function(DefineFunction<'a>),
}

/// A preprocessor text macro function
#[derive(Clone, Debug)]
pub struct DefineFunction<'a> {
    /// A list of arguments (possibly with defaults), with the
    /// [`Span`] being that of the `=`
    pub args: Vec<(
        SpannedString<'a>,
        Option<(
            Span<'a>, // =
            Vec<SpannedToken<'a>>,
        )>,
    )>,
    /// The body of the function
    pub body: Option<Vec<SpannedToken<'a>>>,
}

impl<'a> DefineBody<'a> {
    /// The tokens associated with a definition
    ///
    /// This returns `(tokens, args)`, where `args` is
    /// some function arguments if the definition is a function,
    /// and [`None`] if not
    pub fn get_tokens(
        &self,
    ) -> (
        Vec<SpannedToken<'a>>,
        Option<Vec<(SpannedString<'a>, Option<Vec<SpannedToken<'a>>>)>>,
    ) {
        match self {
            DefineBody::Empty => (vec![], None),
            DefineBody::Text(token_vec) => (token_vec.clone(), None),
            DefineBody::Function(def_func) => {
                let function_args = def_func
                    .args
                    .iter()
                    .map(|(a, b)| match b {
                        Some((_, tokens)) => (a.clone(), Some(tokens.clone())),
                        None => (a.clone(), None),
                    })
                    .collect();
                match &def_func.body {
                    Some(token_vec) => (token_vec.clone(), Some(function_args)),
                    None => (vec![], Some(function_args)),
                }
            }
        }
    }
}

/// A line directive found during preprocessing
#[derive(Clone)]
pub struct LineDirective<'a> {
    /// The file name indicated in the directive
    pub directive_file_name: &'a str,
    /// The line number indicated in the directive
    pub directive_line_number: usize,
    /// The span the directive was found at
    pub original_span: Span<'a>,
    /// The line number the directive was found at
    pub original_line_num: usize,
}

/// The current state of the preprocessor
///
/// The preprocessor has to keep track of various directives and
/// how they affect later preprocessing. A [`PreprocessorState`]
/// encapsulates all of this.
///
/// This is primarily meant to be used in the preprocessor, but
/// can be examined afterwards to glean extra information, such
/// as which files were included.
#[derive(Clone)]
pub struct PreprocessorState<'a> {
    /// The include paths to search for included files
    pub includes: Vec<&'a Path>,
    /// The preprocessor definitions
    pub defines: Vec<Define<'a>>,
    /// Any timescales declared with `` `timescale ``
    pub timescales: Vec<Timescale<'a>>,
    /// Default nettypes declared with `` `default_nettype ``
    pub default_nettypes: Vec<(DefaultNettype, Span<'a>)>,
    /// Unconnected drives declared with `` `unconnected_drive ``
    /// or `` `nounconnected_drive ``
    pub unconnected_drives: Vec<(UnconnectedDrive, Span<'a>)>,
    /// Cell definitions from `` `celldefine `` and `` `endcelldefine ``
    pub cell_defines: Vec<(bool, Span<'a>)>,
    /// Line directives declared with `` `line ``
    pub line_directives: Vec<LineDirective<'a>>,
    /// The contents of included files (`file_name` -> `content`)
    pub included_files: HashMap<&'a str, &'a str>,
    /// The current standard for reserved keywords
    pub curr_standard: StandardVersion,
    /// Any errors encountered so far
    pub errors: Vec<PreprocessorError<'a>>,
    pub(crate) in_define: bool,
    pub(crate) in_define_arg: bool,
    pub(crate) in_text_macro_arg: bool,
    pub(crate) include_depth: usize,
}

impl<'a> PreprocessorState<'a> {
    /// Create a new [`PreprocessorState`]
    pub fn new(includes: Vec<&'a Path>, defines: Vec<Define<'a>>) -> Self {
        Self {
            includes,
            defines,
            timescales: vec![],
            default_nettypes: vec![],
            unconnected_drives: vec![],
            cell_defines: vec![],
            line_directives: vec![],
            included_files: HashMap::new(),
            curr_standard: StandardVersion::default(),
            errors: vec![],
            in_define: false,
            in_define_arg: false,
            in_text_macro_arg: false,
            include_depth: 0,
        }
    }
    /// Make the [`PreprocessorState`] fresh, as though it had just started preprocessing
    ///
    /// This retains all files that were read in, avoiding reading in their contents again
    pub fn make_fresh(&mut self, defines: Vec<Define<'a>>) {
        self.defines = defines;
        self.timescales = vec![];
        self.default_nettypes = vec![];
        self.unconnected_drives = vec![];
        self.cell_defines = vec![];
        self.line_directives = vec![];
        self.curr_standard = StandardVersion::default();
        self.errors = vec![];
        self.in_define = false;
        self.in_define_arg = false;
        self.include_depth = 0;
    }
    /// Reset all resetable configs
    ///
    /// This is called when a `` `resetall `` is encountered
    pub fn reset_all(&mut self, reset_all_span: Span<'a>) {
        self.add_timescale(
            Timescale::new(
                reset_all_span.clone(),
                DEFAULT_TIMESCALE.unit,
                DEFAULT_TIMESCALE.precision,
            )
            .unwrap(),
        );
        self.add_default_nettype(reset_all_span.clone(), DEFAULT_NETTYPE);
        self.add_unconnected_drive(
            reset_all_span.clone(),
            DEFAULT_UNCONNECTED_DRIVE,
        );
        self.add_cell_define(false, reset_all_span);
    }

    /// Check whether the given macro is defined
    pub fn is_defined(&self, macro_name: &'a str) -> bool {
        self.defines.iter().any(|d| d.name.0 == macro_name)
    }

    /// Get the definition span for a macro, if it's been defined
    pub fn get_define_decl(&self, macro_name: &'a str) -> Option<Span<'a>> {
        match self.defines.iter().find(|d| d.name.0 == macro_name) {
            None => None,
            Some(define) => Some(define.name.1.clone()),
        }
    }

    /// Called when starting to preprocess a definition
    ///
    /// Each call to [`PreprocessorState::enter_define`] should be
    /// paired with a later call to [`PreprocessorState::exit_define`]
    #[inline]
    pub(crate) fn enter_define(&mut self) -> bool {
        let prev_in_define = self.in_define;
        self.in_define = true;
        prev_in_define
    }

    /// Called when stopping preprocessing of a definition
    ///
    /// Each call to [`PreprocessorState::enter_define`] should be
    /// paired with a later call to [`PreprocessorState::exit_define`]
    #[inline]
    pub(crate) fn exit_define(&mut self, prev_in_define: bool) {
        self.in_define = prev_in_define;
    }

    /// Whether we're currently preprocessing a definition
    #[inline]
    pub fn in_define(&self) -> bool {
        self.in_define
    }

    /// Called when starting to preprocess a definition argument
    ///
    /// Each call to [`PreprocessorState::enter_define_arg`] should be
    /// paired with a later call to [`PreprocessorState::exit_define_arg`]
    #[inline]
    pub(crate) fn enter_define_arg(&mut self) -> bool {
        let prev_in_define_arg = self.in_define_arg;
        self.in_define_arg = true;
        prev_in_define_arg
    }

    /// Called when stopping preprocessing of a definition argument
    ///
    /// Each call to [`PreprocessorState::enter_define_arg`] should be
    /// paired with a later call to [`PreprocessorState::exit_define_arg`]
    #[inline]
    pub(crate) fn exit_define_arg(&mut self, prev_in_define_arg: bool) {
        self.in_define_arg = prev_in_define_arg;
    }

    /// Whether we're currently preprocessing a definition argument
    #[inline]
    pub fn in_define_arg(&self) -> bool {
        self.in_define_arg
    }

    /// Called when starting to preprocess a text macro argument
    ///
    /// Each call to [`PreprocessorState::enter_text_macro_arg`] should be
    /// paired with a later call to [`PreprocessorState::exit_text_macro_arg`]
    #[inline]
    pub(crate) fn enter_text_macro_arg(&mut self) -> bool {
        let prev_in_text_macro_arg = self.in_text_macro_arg;
        self.in_text_macro_arg = true;
        prev_in_text_macro_arg
    }

    /// Called when stopping preprocessing of a text macro argument
    ///
    /// Each call to [`PreprocessorState::enter_text_macro_arg`] should be
    /// paired with a later call to [`PreprocessorState::exit_text_macro_arg`]
    #[inline]
    pub(crate) fn exit_text_macro_arg(&mut self, prev_in_text_macro_arg: bool) {
        self.in_text_macro_arg = prev_in_text_macro_arg;
    }

    /// Whether we're currently preprocessing a text macro argument
    #[inline]
    pub fn in_text_macro_arg(&self) -> bool {
        self.in_text_macro_arg
    }

    /// Remove a given macro, evaluating to whether a macro was removed
    ///
    /// This is called when a `` `undef `` is encountered
    pub fn undefine(&mut self, macro_name: &'a str) -> bool {
        let prev_len = self.defines.len();
        self.defines.retain(|d| d.name.0 != macro_name);
        prev_len != self.defines.len()
    }

    /// Define a new macro
    ///
    /// This is called when a `` `define `` is encountered, and assumes that
    /// any previous definitions were removed with [`PreprocessorState::undefine`]
    pub fn define(
        &mut self,
        macro_name: &'a str,
        macro_span: Span<'a>,
        macro_body: DefineBody<'a>,
    ) {
        self.defines.push(Define {
            name: SpannedString(macro_name, macro_span),
            body: macro_body,
        });
    }

    /// Define a new macro from the command line
    ///
    /// This is similar to [`PreprocessorState::define`], but
    /// uses a [`Span`] with no file name
    pub fn command_line_define(
        &mut self,
        macro_name: &'a str,
        macro_text: Option<Vec<SpannedToken<'a>>>,
    ) {
        self.define(
            macro_name,
            Span::default(),
            match macro_text {
                None => DefineBody::Empty,
                Some(token_vec) => DefineBody::Text(token_vec),
            },
        )
    }

    /// Undefine all macros
    ///
    /// This is called when a `` `undefineall `` is encountered
    pub fn undefineall(&mut self) {
        self.defines = vec![];
    }

    /// Get the ([`Span`], [`DefineBody::get_tokens`]) for a text macro,
    /// if it exists
    pub fn get_macro_tokens(
        &self,
        macro_name: &'a str,
    ) -> Option<(
        Span<'a>,
        (
            Vec<SpannedToken<'a>>,
            Option<Vec<(SpannedString<'a>, Option<Vec<SpannedToken<'a>>>)>>,
        ),
    )> {
        for define in &self.defines {
            if define.name.0 == macro_name {
                return Some((define.name.1.clone(), define.body.get_tokens()));
            }
        }
        None
    }

    /// Get the full path from an `` `include `` statement
    pub fn get_file_path(&self, include_path: &str) -> Option<PathBuf> {
        for dir_path in &self.includes {
            let full_path = Path::new(dir_path).join(include_path);
            if full_path.exists() {
                return Some(full_path);
            }
        }
        None
    }

    /// Add a compiler directive timescale
    ///
    /// This is called when a `` `timescale `` is encountered
    pub fn add_timescale(&mut self, timescale: Timescale<'a>) {
        self.timescales.push(timescale);
    }

    /// Get the correct compiler timescale, based on the [`Span`]
    /// where a delay is encountered
    pub fn get_timescale(&self, span: &Span<'a>) -> &Timescale<'a> {
        for timescale in self.timescales.iter().rev() {
            if timescale.is_valid(span) {
                return timescale;
            }
        }
        // Default timescale
        &DEFAULT_TIMESCALE
    }

    /// Add a compiler directive default nettype
    ///
    /// This is called when a `` `default_nettype `` is encountered
    pub fn add_default_nettype(
        &mut self,
        def_span: Span<'a>,
        default_nettype: DefaultNettype,
    ) {
        self.default_nettypes.push((default_nettype, def_span));
    }

    /// Get the correct compiler default nettype, based on the [`Span`]
    /// where an implicit nettype is needed
    pub fn get_default_nettype(&self, span: &Span<'a>) -> &DefaultNettype {
        for default_nettype in self.default_nettypes.iter().rev() {
            if default_nettype.1.compare(span) == SpanRelation::Earlier {
                return &default_nettype.0;
            }
        }
        &DEFAULT_NETTYPE
    }

    /// Retain the contents of a file found from an `` `include `` statement
    ///
    /// Produces `(&path, &file_contents)`
    ///
    /// This differs from [`PreprocessorState::retain_file`] by only reading
    /// in the file if necessary, and using the include paths to find the
    /// correct file to read in.
    pub(crate) fn retain_include_file(
        &mut self,
        include_path: &'a str,
        include_path_span: Span<'a>,
        cache: &'a PreprocessorCache<'a>,
    ) -> Result<(&'a str, &'a str), PreprocessorError<'a>> {
        let include_path_buf =
            self.get_file_path(include_path).ok_or_else(|| {
                PreprocessorError::Include {
                    include_path,
                    include_path_span: include_path_span.clone(),
                    read_err: io::ErrorKind::NotFound,
                }
            })?;
        match self
            .included_files
            .get_key_value::<str>(include_path_buf.to_str().unwrap())
        {
            Some((path, contents)) => Ok((*path, *contents)),
            None => {
                let cached_path = cache.retain_string(
                    include_path_buf.to_str().unwrap().to_owned(),
                );
                let file_contents = std::fs::read_to_string(&cached_path)
                    .map_err(|err| PreprocessorError::Include {
                        include_path,
                        include_path_span,
                        read_err: err.kind(),
                    })?;
                let cached_contents = cache.retain_string(file_contents);
                self.included_files.insert(cached_path, cached_contents);
                Ok((cached_path, cached_contents))
            }
        }
    }

    /// Retain the contents of a file
    ///
    /// Produces `(&file_path, &file_contents)` as references to
    /// the cached passed arguments
    pub fn retain_file(
        &mut self,
        file_path: String,
        file_contents: String,
        cache: &'a PreprocessorCache<'a>,
    ) -> (&'a str, &'a str) {
        match self.included_files.get_key_value::<str>(file_path.as_ref()) {
            Some((path, contents)) => (*path, *contents),
            None => {
                let path = cache.retain_string(file_path);
                let contents = cache.retain_string(file_contents);
                self.included_files.insert(path, contents);
                (path, contents)
            }
        }
    }

    /// Get the included files as a [`Vec`] of (name, content) tuples
    pub fn included_files(&self) -> Vec<(String, String)> {
        self.included_files
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect()
    }

    /// Add an unconnected drive
    ///
    /// This is called when a `` `unconnected_drive `` or a
    /// `` `nounconnected_drive `` is encountered
    pub fn add_unconnected_drive(
        &mut self,
        unconnected_drive_span: Span<'a>,
        unconnected_drive: UnconnectedDrive,
    ) {
        self.unconnected_drives
            .push((unconnected_drive, unconnected_drive_span));
    }

    /// Get the unconnected drive based on the [`Span`] where an
    /// unconnected net is encountered
    pub fn get_unconnected_drive(&self, span: &Span<'a>) -> &UnconnectedDrive {
        for unconnected_drive in self.unconnected_drives.iter().rev() {
            if unconnected_drive.1.compare(span) == SpanRelation::Earlier {
                return &unconnected_drive.0;
            }
        }
        &DEFAULT_UNCONNECTED_DRIVE
    }

    /// Add a cell define declaration
    ///
    /// This is called when a `` `celldefine `` is encountered
    pub fn add_cell_define(&mut self, is_cell_define: bool, span: Span<'a>) {
        self.cell_defines.push((is_cell_define, span));
    }

    /// Determine whether a module is a cell module, based on the [`Span`]
    /// of the module declaration
    pub fn is_cell_module(&self, declaration_span: &Span<'a>) -> bool {
        for cell_define in self.cell_defines.iter().rev() {
            if cell_define.1.compare(declaration_span) == SpanRelation::Earlier
            {
                return cell_define.0;
            }
        }
        false
    }

    /// Add a line directive
    ///
    /// This is called when a `` `line `` is encountered
    pub fn add_line_directive(
        &mut self,
        file_name: &'a str,
        line_number: &'a str,
        dir_span: Span<'a>,
    ) {
        let offset = dir_span.bytes.end;
        let file_contents: &str =
            self.included_files.get(dir_span.file).unwrap();
        let line_num = file_contents[..offset].lines().count();
        let new_line_directive = LineDirective {
            directive_file_name: file_name,
            directive_line_number: line_number.parse().unwrap(),
            original_span: dir_span,
            original_line_num: line_num,
        };
        self.line_directives.push(new_line_directive);
    }

    /// Get the file name from a [`Span`], factoring in `` `line `` directives
    pub fn get_line_directive_file(&self, span: &Span<'a>) -> &'a str {
        let Some(line_directive) = self
            .line_directives
            .iter()
            .rev()
            .filter(|line_directive| {
                (line_directive.original_span.file == span.file)
                    && (line_directive.original_span.bytes.start
                        < span.bytes.start) // Only relevant if file is included twice
            })
            .next()
        else {
            return span.file;
        };
        line_directive.directive_file_name
    }

    /// Get the line number of a [`Span`], factoring in `` `line `` directives
    pub fn get_line_directive_line(
        &mut self,
        span: &Span<'a>,
        cache: &'a PreprocessorCache<'a>,
    ) -> &'a str {
        let offset = span.bytes.end;
        let file_contents: &str = self.included_files.get(span.file).unwrap();
        let line_num = file_contents[..offset].lines().count();
        let Some(line_directive) = self
            .line_directives
            .iter()
            .rev()
            .filter(|line_directive| {
                (line_directive.original_span.file == span.file)
                    && (line_directive.original_span.bytes.start
                        < span.bytes.start) // Only relevant if file is included twice
            })
            .next()
        else {
            return cache.retain_string(line_num.to_string());
        };
        let new_line_num = (line_num + line_directive.directive_line_number)
            - (line_directive.original_line_num + 1);
        cache.retain_string(new_line_num.to_string())
    }

    /// Get the text referenced by a [`Span`]
    pub(crate) fn get_slice(&self, span: &Span<'a>) -> Option<&'a str> {
        let file_contents: &str = self.included_files.get(span.file)?;
        Some(&file_contents[span.bytes.start..span.bytes.end])
    }

    pub fn retain_string(
        &mut self,
        string: String,
        cache: &'a PreprocessorCache<'a>,
    ) -> &'a str {
        cache.retain_string(string)
    }

    /// Add a error encountered during preprocessing
    pub fn err(&mut self, warning: PreprocessorError<'a>) {
        self.errors.push(warning);
    }
}
