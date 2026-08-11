// =======================================================================
// numbers.rs
// =======================================================================
// Parsing for 1800-2023 A.8.8

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::token::any;

pub fn string_literal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StringLiteral<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::StringLiteral(text) => {
                Some(StringLiteral::QuotedString(Box::new(QuotedString(
                    text,
                    Metadata::new(s.1.clone(), Vec::new()),
                ))))
            }
            Token::TripleQuoteStringLiteral(text) => {
                Some(StringLiteral::TripleQuotedString(Box::new(
                    TripleQuotedString(
                        text,
                        Metadata::new(s.1.clone(), Vec::new()),
                    ),
                )))
            }
            _ => None,
        }),
        non_trivia_parser,
    )
        .map(|(string, non_trivia)| match string {
            StringLiteral::QuotedString(quoted_string_box) => {
                StringLiteral::QuotedString(Box::new(QuotedString(
                    quoted_string_box.0,
                    replace_non_trivia(quoted_string_box.1, non_trivia),
                )))
            }
            StringLiteral::TripleQuotedString(triple_quoted_string_box) => {
                StringLiteral::TripleQuotedString(Box::new(TripleQuotedString(
                    triple_quoted_string_box.0,
                    replace_non_trivia(triple_quoted_string_box.1, non_trivia),
                )))
            }
        })
        .context("a string")
        .parse_next(input)
}
