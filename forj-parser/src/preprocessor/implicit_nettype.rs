// =======================================================================
// implicit_nettype.rs
// =======================================================================
// Preprocessing for default nettypes

use crate::Span;
use crate::*;

#[derive(Clone, Debug)]
pub enum DefaultNettype {
    Wire,
    Tri,
    Tri0,
    Tri1,
    Wand,
    Triand,
    Wor,
    Trior,
    Trireg,
    Uwire,
    None,
}

fn get_nettype<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    define_span: Span<'s>,
) -> Result<DefaultNettype, PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: define_span,
        });
    };
    match spanned_token.0 {
        Token::Wire => Ok(DefaultNettype::Wire),
        Token::Tri => Ok(DefaultNettype::Tri),
        Token::Tri0 => Ok(DefaultNettype::Tri0),
        Token::Tri1 => Ok(DefaultNettype::Tri1),
        Token::Wand => Ok(DefaultNettype::Wand),
        Token::Triand => Ok(DefaultNettype::Triand),
        Token::Wor => Ok(DefaultNettype::Wor),
        Token::Trior => Ok(DefaultNettype::Trior),
        Token::Trireg => Ok(DefaultNettype::Trireg),
        Token::Uwire => Ok(DefaultNettype::Uwire),
        Token::SimpleIdentifier("none") => Ok(DefaultNettype::None),
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![
                    Expectation::Token(Token::Wire),
                    Expectation::Token(Token::Tri),
                    Expectation::Token(Token::Tri0),
                    Expectation::Token(Token::Tri1),
                    Expectation::Token(Token::Wand),
                    Expectation::Token(Token::Triand),
                    Expectation::Token(Token::Wor),
                    Expectation::Token(Token::Trior),
                    Expectation::Token(Token::Trireg),
                    Expectation::Token(Token::Uwire),
                    Expectation::Label("'none'"),
                ],
            },
        }),
    }
}

pub fn preprocess_default_nettype<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let default_nettype =
        get_nettype(src, state, cache, directive_span.clone())?;
    state.add_default_nettype(directive_span, default_nettype);
    Ok(())
}

#[test]
fn wire() {
    check_preprocessor!("`default_nettype wire", Vec::<Token<'_>>::new())
}

#[test]
fn tri() {
    check_preprocessor!("`default_nettype tri", Vec::<Token<'_>>::new())
}

#[test]
fn tri0() {
    check_preprocessor!("`default_nettype tri0", Vec::<Token<'_>>::new())
}

#[test]
fn tri1() {
    check_preprocessor!("`default_nettype tri1", Vec::<Token<'_>>::new())
}

#[test]
fn wand() {
    check_preprocessor!("`default_nettype wand", Vec::<Token<'_>>::new())
}

#[test]
fn triand() {
    check_preprocessor!("`default_nettype triand", Vec::<Token<'_>>::new())
}

#[test]
fn wor() {
    check_preprocessor!("`default_nettype wor", Vec::<Token<'_>>::new())
}

#[test]
fn trior() {
    check_preprocessor!("`default_nettype trior", Vec::<Token<'_>>::new())
}

#[test]
fn trireg() {
    check_preprocessor!("`default_nettype trireg", Vec::<Token<'_>>::new())
}

#[test]
fn uwire() {
    check_preprocessor!("`default_nettype uwire", Vec::<Token<'_>>::new())
}

#[test]
fn none() {
    check_preprocessor!("`default_nettype none", Vec::<Token<'_>>::new())
}
