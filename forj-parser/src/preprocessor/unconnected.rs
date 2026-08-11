// =======================================================================
// unconnected.rs
// =======================================================================
// Preprocessing for directives affecting unconnected ports

use crate::Span;
use crate::*;

#[derive(Clone, Debug)]
pub enum UnconnectedDrive {
    PullUp,
    PullDown,
    NoUnconnected,
}

fn get_unconnected_drive<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    directive_span: Span<'s>,
) -> Result<UnconnectedDrive, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective { directive_span });
    };
    match spanned_token.0 {
        Token::Pull0 => Ok(UnconnectedDrive::PullDown),
        Token::Pull1 => Ok(UnconnectedDrive::PullUp),
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a valid unconnected drive")],
            },
        }),
    }
}

pub fn preprocess_unconnected_drive<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let unconnected_drive = get_unconnected_drive(src, directive_span.clone())?;
    state.add_unconnected_drive(directive_span, unconnected_drive);
    Ok(())
}

pub fn preprocess_nounconnected_drive<'s>(
    state: &mut PreprocessorState<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    state
        .add_unconnected_drive(directive_span, UnconnectedDrive::NoUnconnected);
    Ok(())
}

#[test]
fn unconnected_pull0() {
    check_preprocessor!("`unconnected_drive pull0", Vec::<Token<'_>>::new())
}

#[test]
fn unconnected_pull1() {
    check_preprocessor!("`unconnected_drive pull1", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "a valid unconnected drive")]
fn invalid_unconnected_drive() {
    check_preprocessor!("`unconnected_drive logic", Vec::<Token<'_>>::new())
}

#[test]
fn nounconnected() {
    check_preprocessor!("`nounconnected_drive", Vec::<Token<'_>>::new())
}
