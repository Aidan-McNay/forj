// =======================================================================
// mod.rs
// =======================================================================
//! Preprocessing a token stream, elaborating compiler directives

pub mod cache;
pub(crate) mod conditional_compilation;
pub(crate) mod define;
pub(crate) mod error;
pub(crate) mod implicit_nettype;
pub(crate) mod include;
pub(crate) mod keywords;
pub(crate) mod line;
pub(crate) mod number;
pub mod state;
pub(crate) mod text_macro;
pub(crate) mod timescale;
pub(crate) mod unconnected;
use crate::*;
pub use cache::*;
use conditional_compilation::*;
use define::*;
pub use error::*;
pub(crate) use implicit_nettype::DefaultNettype;
use implicit_nettype::*;
use include::*;
use keywords::*;
use line::*;
use number::*;
pub use state::*;
use std::collections::VecDeque;
use text_macro::*;
use timescale::*;
pub(crate) use timescale::{Timescale, TimescaleUnit, TimescaleValue};
pub(crate) use unconnected::UnconnectedDrive;
use unconnected::*;

/// A peekable, extendable iterator over tokens.
///
/// This iterator extends `<T>` by keeping track of an additional
/// stack of tokens at the front, allowing users to peek the next
/// token, as well as push tokens to be iterated on next (such as
/// when expanding a preprocessor definition)
pub(crate) struct TokenIterator<'s, T: Iterator<Item = SpannedToken<'s>>> {
    iter: T,
    extras: VecDeque<SpannedToken<'s>>,
}

impl<'s, T: Iterator<Item = SpannedToken<'s>>> Iterator
    for TokenIterator<'s, T>
{
    type Item = SpannedToken<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(extra_token) = self.extras.pop_front() {
            Some(extra_token)
        } else {
            self.iter.next()
        }
    }
}

impl<'s, T: Iterator<Item = SpannedToken<'s>>> TokenIterator<'s, T> {
    pub fn new(iter: T) -> Self {
        Self {
            iter,
            extras: VecDeque::default(),
        }
    }

    pub fn prepend_tokens<I>(&mut self, extra_tokens: I)
    where
        I: Iterator<Item = SpannedToken<'s>>
            + ExactSizeIterator
            + std::iter::DoubleEndedIterator,
    {
        self.extras.reserve(extra_tokens.len());
        for extra_token in extra_tokens.rev() {
            self.extras.push_front(extra_token);
        }
    }

    pub fn peek(&mut self) -> Option<&SpannedToken<'s>> {
        if self.extras.is_empty() {
            if let Some(next_token) = self.iter.next() {
                self.extras.push_back(next_token);
            }
        }
        self.extras.front()
    }
}

/// Attempt to recover from a preprocessor error by going to the next
/// non-escaped newline, returning whether one was encountered
pub(crate) fn recover_newline<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
) -> bool {
    loop {
        let Some(SpannedToken(curr_token, _)) = src.next() else {
            return false;
        };
        let next_token = src.peek();
        match (curr_token, next_token) {
            (Token::Bslash, Some(SpannedToken(Token::Newline, _))) => {
                let _newline_token = src.next();
                ()
            }
            (Token::Newline, _) => {
                return true;
            }
            _ => (),
        }
    }
}

/// Attempt to recover from a preprocessor error, reproducing the error if
/// not possible
///
/// Many of these are trivial, as they are removed from the token stream
/// already
pub(crate) fn recover<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    err: PreprocessorError<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let recovered = match err {
        PreprocessorError::Endif { .. } => true,
        PreprocessorError::NoEndif { .. } => false, // EOF
        PreprocessorError::Elsif { .. } => true,
        PreprocessorError::Else { .. } => true,
        PreprocessorError::EndKeywords { .. } => true,
        PreprocessorError::NoEndKeywords { .. } => false, // EOF
        PreprocessorError::InvalidDefineParameter { .. } => {
            recover_newline(src)
        }
        PreprocessorError::InvalidDefineArgument { .. } => recover_newline(src),
        PreprocessorError::InvalidVersionSpecifier { .. } => true,
        PreprocessorError::IncompleteDirective { .. } => recover_newline(src),
        PreprocessorError::IncompleteDefine { .. } => recover_newline(src),
        PreprocessorError::UndefinedMacro { .. } => true, // Don't worry about functions here
        PreprocessorError::DuplicateMacroParameter { .. } => {
            recover_newline(src)
        }
        PreprocessorError::NoDefaultAfterDefault { .. } => recover_newline(src),
        PreprocessorError::NoMacroArguments { .. } => true,
        PreprocessorError::TooManyMacroArguments { .. } => true,
        PreprocessorError::MissingMacroArgument { .. } => true,
        PreprocessorError::InvalidIdentifierFormation { .. } => true,
        PreprocessorError::InvalidRelativeTimescales { .. } => true,
        PreprocessorError::IncompleteMacroWithToken { .. } => {
            recover_newline(src)
        }
        PreprocessorError::Include { .. } => true,
        PreprocessorError::IncludeDepth { .. } => true,
        PreprocessorError::VerboseError { .. } => recover_newline(src),
        PreprocessorError::NotPreviouslyDefinedMacro { .. }
        | PreprocessorError::RedefinedMacro { .. } => {
            panic!("Shouldn't need to recover from warnings")
        }
        PreprocessorError::NewlineInDefine(_)
        | PreprocessorError::EndOfFunctionArgument(_) => {
            panic!("Tried to recover from an internal error")
        }
    };
    if recovered {
        state.err(err);
        Ok(())
    } else {
        Err(err)
    }
}

pub(crate) fn preprocess_helper<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let mut enclosures: Vec<Token<'s>> = vec![];
    if state.in_define() || state.in_define_arg() || state.in_text_macro_arg() {
        while let Some(spanned_token) = src.next() {
            match spanned_token.0 {
                Token::Bslash if !state.in_text_macro_arg() => {
                    match src.next() {
                        None => dest.push(spanned_token),
                        Some(next_token) => match next_token.0 {
                            Token::Newline => (),
                            Token::Bslash => {
                                dest.push(spanned_token);
                            }
                            _ => {
                                dest.push(spanned_token);
                                dest.push(next_token)
                            }
                        },
                    };
                }
                Token::Newline if !state.in_text_macro_arg() => {
                    return Err(PreprocessorError::NewlineInDefine(
                        spanned_token.1,
                    ));
                }
                Token::Paren
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    enclosures.push(Token::Paren);
                    dest.push(spanned_token);
                }
                Token::Bracket
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    enclosures.push(Token::Bracket);
                    dest.push(spanned_token);
                }
                Token::Brace
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    enclosures.push(Token::Brace);
                    dest.push(spanned_token);
                }
                Token::EParen
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    match enclosures.pop() {
                        Some(Token::Paren) => dest.push(spanned_token),
                        None => {
                            return Err(
                                PreprocessorError::EndOfFunctionArgument(
                                    spanned_token,
                                ),
                            );
                        }
                        _ => {
                            recover(
                                src,
                                state,
                                PreprocessorError::IncompleteMacroWithToken {
                                    error_token: spanned_token.0,
                                    error_span: spanned_token.1,
                                },
                            )?;
                        }
                    }
                }
                Token::EBracket
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    match enclosures.pop() {
                        Some(Token::Bracket) => dest.push(spanned_token),
                        _ => {
                            recover(
                                src,
                                state,
                                PreprocessorError::IncompleteMacroWithToken {
                                    error_token: spanned_token.0,
                                    error_span: spanned_token.1,
                                },
                            )?;
                        }
                    }
                }
                Token::EBrace
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    match enclosures.pop() {
                        Some(Token::Brace) => dest.push(spanned_token),
                        _ => {
                            recover(
                                src,
                                state,
                                PreprocessorError::IncompleteMacroWithToken {
                                    error_token: spanned_token.0,
                                    error_span: spanned_token.1,
                                },
                            )?;
                        }
                    }
                }
                Token::Comma
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    if enclosures.is_empty() {
                        return Err(PreprocessorError::EndOfFunctionArgument(
                            spanned_token,
                        ));
                    } else {
                        dest.push(spanned_token)
                    }
                }
                Token::BlockComment(_) => (),
                Token::OnelineComment(comment_text) => {
                    if comment_text.ends_with('\\') & !state.in_text_macro_arg()
                    {
                        // Counts as escaping a newline
                        //
                        // Can safely consume and disregard next token (the newline)
                        let _ = src.next();
                    };
                }
                Token::TextMacro(macro_name)
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    if let Err(err) = preprocess_macro(
                        src,
                        state,
                        cache,
                        (macro_name, spanned_token.1),
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::Apost | Token::UnsignedNumber(_)
                    if state.in_define_arg() || state.in_text_macro_arg() =>
                {
                    if let Err(err) = preprocess_possible_number(
                        src,
                        dest,
                        state,
                        cache,
                        spanned_token,
                    ) {
                        recover(src, state, err)?;
                    }
                }
                _ => dest.push(spanned_token),
            }
        }
        Ok(())
    } else {
        while let Some(spanned_token) = src.next() {
            match spanned_token.0 {
                Token::DirResetall => {
                    state.reset_all(spanned_token.1);
                }
                Token::DirInclude => {
                    let include_span = cache.retain_span(spanned_token.1);
                    if let Err(err) = preprocess_include(
                        src,
                        dest,
                        state,
                        cache,
                        include_span,
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::DirUndefineall => {
                    state.undefineall();
                }
                Token::DirBeginKeywords => {
                    if let Err(err) = preprocess_begin_keyword(
                        src,
                        state,
                        cache,
                        spanned_token.1,
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::DirDefine => {
                    if let Err(err) =
                        preprocess_define(src, state, cache, spanned_token.1)
                    {
                        recover(src, state, err)?;
                    }
                }
                Token::DirElse => {
                    return Err(PreprocessorError::Else {
                        else_span: spanned_token.1,
                    });
                }
                Token::DirElsif => {
                    return Err(PreprocessorError::Elsif {
                        elsif_span: spanned_token.1,
                    });
                }
                Token::DirEndKeywords => {
                    if let Err(err) =
                        preprocess_end_keyword(state, spanned_token.1)
                    {
                        recover(src, state, err)?;
                    }
                }
                Token::DirEndif => {
                    return Err(PreprocessorError::Endif {
                        endif_span: spanned_token.1,
                    });
                }
                Token::DirIfdef => {
                    if let Err(err) = preprocess_ifdef(
                        src,
                        dest,
                        state,
                        cache,
                        spanned_token.1,
                        true,
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::DirIfndef => {
                    if let Err(err) = preprocess_ifdef(
                        src,
                        dest,
                        state,
                        cache,
                        spanned_token.1,
                        false,
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::TextMacro(macro_name) => {
                    if let Err(err) = preprocess_macro(
                        src,
                        state,
                        cache,
                        (macro_name, spanned_token.1),
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::Apost | Token::UnsignedNumber(_) => {
                    if let Err(err) = preprocess_possible_number(
                        src,
                        dest,
                        state,
                        cache,
                        spanned_token,
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::DirUndef => {
                    if let Err(err) =
                        preprocess_undefine(src, state, spanned_token.1)
                    {
                        recover(src, state, err)?;
                    }
                }
                Token::DirTimescale => {
                    if let Err(err) =
                        preprocess_timescale(src, state, cache, spanned_token.1)
                    {
                        recover(src, state, err)?;
                    }
                }
                Token::DirDefaultNettype => {
                    if let Err(err) = preprocess_default_nettype(
                        src,
                        state,
                        cache,
                        spanned_token.1,
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::DirUnconnectedDrive => {
                    if let Err(err) = preprocess_unconnected_drive(
                        src,
                        state,
                        spanned_token.1,
                    ) {
                        recover(src, state, err)?;
                    }
                }
                Token::DirNounconnectedDrive => {
                    if let Err(err) =
                        preprocess_nounconnected_drive(state, spanned_token.1)
                    {
                        recover(src, state, err)?;
                    }
                }
                Token::DirCelldefine => {
                    state.add_cell_define(true, spanned_token.1);
                }
                Token::DirEndcelldefine => {
                    state.add_cell_define(false, spanned_token.1);
                }
                Token::DirLine => {
                    if let Err(err) =
                        preprocess_line(src, state, cache, spanned_token.1)
                    {
                        recover(src, state, err)?;
                    }
                }
                Token::DirUnderscoreFile => dest.push(SpannedToken(
                    Token::StringLiteral(
                        state.get_line_directive_file(&spanned_token.1),
                    ),
                    spanned_token.1,
                )),
                Token::DirUnderscoreLine => dest.push(SpannedToken(
                    Token::UnsignedNumber(
                        state.get_line_directive_line(&spanned_token.1, cache),
                    ),
                    spanned_token.1,
                )),
                Token::BlockComment(_)
                | Token::OnelineComment(_)
                | Token::Newline => {
                    #[cfg(feature = "parse_lossless")]
                    {
                        dest.push(spanned_token)
                    }
                }
                token
                    if token.keyword_replace(state.get_keyword_standard()) =>
                {
                    let new_token = SpannedToken(
                        Token::SimpleIdentifier(token.as_str()),
                        spanned_token.1,
                    );
                    dest.push(new_token)
                }
                _ => dest.push(spanned_token),
            }
        }
        Ok(())
    }
}

pub(crate) fn preprocess_single<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<Option<SpannedToken<'s>>, PreprocessorError<'s>> {
    loop {
        match src.next() {
            None => {
                break Ok(None);
            }
            Some(SpannedToken(Token::BlockComment(_), _)) => (),
            Some(SpannedToken(Token::TextMacro(macro_name), macro_span)) => {
                preprocess_macro(src, state, cache, (macro_name, macro_span))?;
            }
            other => break Ok(other),
        }
    }
}

/// Produce any additional errors from examining the [`PreprocessorState`],
/// specifically those from preprocessor directives that expected a
/// pair and did not already produce an error
pub(crate) fn preprocess_cleanup<'s>(state: &mut PreprocessorState<'s>) {
    let keyword_standard_err = state.curr_standard.iter().map(|(_, span)| {
        PreprocessorError::NoEndKeywords {
            begin_keywords_span: span.clone(),
        }
    });
    state.errors.extend(keyword_standard_err);
}

/// Preprocess the given token stream, elaborating any compiler directives
///
/// `state` is augmented during preprocessing (and can be examined afterwards,
/// likely to inspect any errors found), and `cache` is used to retain any new
/// files/spans found during preprocessing
///
/// [`preprocess`] returns the elaborated stream, as well as whether the
/// initial stream was consumed completely (`false` if an irrecoverable
/// error was encountered)
///
/// ```rust
/// # use forj_parser::*;
/// # let mut state = PreprocessorState::new(vec![], vec![]);
/// # let cache = PreprocessorCache::new();
/// let file_contents = "
/// `define TEST(a, b) a + b
/// `TEST(1, 2)
/// ";
/// state.retain_file("test_file.v".to_string(), file_contents.to_string(), &cache);
/// let tokens = lex(file_contents, "test_file.v").tokens();
/// let mut pp_tokens = preprocess(tokens, &mut state, &cache).unwrap().into_iter();
/// assert_eq!(pp_tokens.next().unwrap().0, Token::UnsignedNumber("1"));
/// assert_eq!(pp_tokens.next().unwrap().0, Token::Plus);
/// assert_eq!(pp_tokens.next().unwrap().0, Token::UnsignedNumber("2"));
/// assert_eq!(pp_tokens.next(), None)
/// ```
pub fn preprocess<'s>(
    src: impl Iterator<Item = SpannedToken<'s>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<Vec<SpannedToken<'s>>, ()> {
    let mut token_iter = TokenIterator::new(src);
    let mut dest = Vec::new();
    loop {
        match preprocess_helper(&mut token_iter, &mut dest, state, cache) {
            Ok(()) => {
                preprocess_cleanup(state);
                if state.errors.iter().all(|err| err.is_warning()) {
                    return Ok(dest);
                } else {
                    return Err(());
                }
            }
            Err(err) => {
                if let Err(err) = recover(&mut token_iter, state, err) {
                    preprocess_cleanup(state);
                    state.errors.push(err);
                    return Err(());
                }
            }
        }
    }
}
