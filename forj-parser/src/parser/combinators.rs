// =======================================================================
// combinators.rs
// =======================================================================
// Custom combinators that note errors that may not be needed

use crate::*;
use winnow::Parser;
use winnow::combinator::trace;
use winnow::error::{ErrMode, ParserError};
use winnow::stream::{Accumulate, Stream};

pub fn repeat_note<'s: 'b, 'b, Output, ParseNext>(
    parser: ParseNext,
) -> RepeatNote<'s, 'b, ParseNext, Output>
where
    ParseNext: Parser<Tokens<'s, 'b>, Output, ErrMode<VerboseError<'s>>>,
{
    RepeatNote {
        parser,
        i: Default::default(),
        o: Default::default(),
    }
}

pub struct RepeatNote<'s: 'b, 'b, P, O>
where
    P: Parser<Tokens<'s, 'b>, O, ErrMode<VerboseError<'s>>>,
{
    parser: P,
    i: core::marker::PhantomData<Tokens<'s, 'b>>,
    o: core::marker::PhantomData<O>,
}

impl<'s: 'b, 'b, P, O> Parser<Tokens<'s, 'b>, Vec<O>, ErrMode<VerboseError<'s>>>
    for RepeatNote<'s, 'b, P, O>
where
    P: Parser<Tokens<'s, 'b>, O, ErrMode<VerboseError<'s>>>,
{
    #[inline(always)]
    fn parse_next(
        &mut self,
        i: &mut Tokens<'s, 'b>,
    ) -> Result<Vec<O>, ErrMode<VerboseError<'s>>> {
        trace("repeat_note", move |i: &mut Tokens<'s, 'b>| {
            fold_repeat0_(
                &mut self.parser,
                &mut || Vec::initial(None),
                &mut |mut acc, o| {
                    acc.accumulate(o);
                    acc
                },
                i,
            )
        })
        .parse_next(i)
    }
}

fn fold_repeat0_<'s: 'b, 'b, O, P, N, F, R>(
    parser: &mut P,
    init: &mut N,
    fold: &mut F,
    input: &mut Tokens<'s, 'b>,
) -> Result<R, ErrMode<VerboseError<'s>>>
where
    P: Parser<Tokens<'s, 'b>, O, ErrMode<VerboseError<'s>>>,
    N: FnMut() -> R,
    F: FnMut(R, O) -> R,
{
    let mut res = init();

    loop {
        let start = input.checkpoint();
        let len = input.eof_offset();
        match parser.parse_next(input) {
            Ok(output) => {
                // infinite loop check: the parser must always consume
                if input.eof_offset() == len {
                    return Err(ParserError::assert(
                        input,
                        "`repeat_note` parsers must always consume",
                    ));
                }

                res = fold(res, output);
            }
            Err(err) => match err {
                ErrMode::Backtrack(verbose_error) => {
                    input.reset(&start);
                    match &mut input.state.last_err {
                        None => input.state.last_err = Some(verbose_error),
                        Some(prev_error) => {
                            prev_error.or_in_place(verbose_error)
                        }
                    };
                    return Ok(res);
                }
                _ => return Err(err),
            },
        }
    }
}

pub fn opt_note<'s: 'b, 'b, Output, ParseNext>(
    mut parser: ParseNext,
) -> impl Parser<Tokens<'s, 'b>, Option<Output>, ErrMode<VerboseError<'s>>>
where
    ParseNext: Parser<Tokens<'s, 'b>, Output, ErrMode<VerboseError<'s>>>,
{
    trace("opt", move |input: &mut Tokens<'s, 'b>| {
        let start = input.checkpoint();
        match parser.parse_next(input) {
            Ok(o) => Ok(Some(o)),
            Err(err) => match err {
                ErrMode::Backtrack(verbose_error) => {
                    input.reset(&start);
                    match &mut input.state.last_err {
                        None => input.state.last_err = Some(verbose_error),
                        Some(prev_error) => {
                            prev_error.or_in_place(verbose_error)
                        }
                    };
                    return Ok(None);
                }
                _ => return Err(err),
            },
        }
    })
}
