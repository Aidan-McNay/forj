// =======================================================================
// combinators.rs
// =======================================================================
// Custom combinators that note errors that may not be needed

use crate::*;
use winnow::Parser;
use winnow::combinator::trace;
use winnow::error::{ErrMode, ParserError};
use winnow::stream::{Accumulate, Stream};

pub fn repeat_note<'s, Output, ParseNext>(
    parser: ParseNext,
) -> RepeatNote<'s, ParseNext, Output>
where
    ParseNext: Parser<Tokens<'s>, Output, ErrMode<VerboseError<'s>>>,
{
    RepeatNote {
        parser,
        i: Default::default(),
        o: Default::default(),
    }
}

pub struct RepeatNote<'s, P, O>
where
    P: Parser<Tokens<'s>, O, ErrMode<VerboseError<'s>>>,
{
    parser: P,
    i: core::marker::PhantomData<Tokens<'s>>,
    o: core::marker::PhantomData<O>,
}

impl<'s, P, O> Parser<Tokens<'s>, Vec<O>, ErrMode<VerboseError<'s>>>
    for RepeatNote<'s, P, O>
where
    P: Parser<Tokens<'s>, O, ErrMode<VerboseError<'s>>>,
{
    #[inline(always)]
    fn parse_next(
        &mut self,
        i: &mut Tokens<'s>,
    ) -> Result<Vec<O>, ErrMode<VerboseError<'s>>> {
        trace("repeat_strict", move |i: &mut Tokens<'s>| {
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

fn fold_repeat0_<'s, O, P, N, F, R>(
    parser: &mut P,
    init: &mut N,
    fold: &mut F,
    input: &mut Tokens<'s>,
) -> Result<R, ErrMode<VerboseError<'s>>>
where
    P: Parser<Tokens<'s>, O, ErrMode<VerboseError<'s>>>,
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
                        "`repeat_strict` parsers must always consume",
                    ));
                }

                res = fold(res, output);
            }
            Err(err) => match err {
                ErrMode::Backtrack(verbose_error) => {
                    input.reset(&start);
                    match &mut input.state {
                        None => input.state = Some(verbose_error),
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

pub fn opt_note<'s, Output, ParseNext>(
    mut parser: ParseNext,
) -> impl Parser<Tokens<'s>, Option<Output>, ErrMode<VerboseError<'s>>>
where
    ParseNext: Parser<Tokens<'s>, Output, ErrMode<VerboseError<'s>>>,
{
    trace("opt", move |input: &mut Tokens<'s>| {
        let start = input.checkpoint();
        match parser.parse_next(input) {
            Ok(o) => Ok(Some(o)),
            Err(err) => match err {
                ErrMode::Backtrack(verbose_error) => {
                    input.reset(&start);
                    match &mut input.state {
                        None => input.state = Some(verbose_error),
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
