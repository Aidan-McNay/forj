// =======================================================================
// mod.rs
// =======================================================================
//! Parsing a token stream into a SystemVerilog CST
pub(crate) mod behavioral_statements;
pub(crate) mod combinators;
pub(crate) mod declarations;
pub(crate) mod expressions;
pub(crate) mod general;
pub(crate) mod instantiations;
pub(crate) mod pratt;
pub(crate) mod primitive_instances;
pub(crate) mod source_text;
pub(crate) mod spanned_token;
pub(crate) mod specify_section;
pub(crate) mod udp_declaration_and_instantiation;
pub(crate) mod utils;
use crate::*;
pub(crate) use behavioral_statements::*;
pub(crate) use combinators::*;
pub(crate) use declarations::*;
pub(crate) use expressions::*;
use forj_syntax::*;
pub(crate) use general::*;
pub(crate) use instantiations::*;
pub(crate) use pratt::*;
pub(crate) use primitive_instances::*;
pub(crate) use source_text::*;
pub(crate) use spanned_token::*;
pub(crate) use specify_section::*;
pub(crate) use udp_declaration_and_instantiation::*;
pub(crate) use utils::*;
use winnow::error::{ErrMode, ParserError};

/// Parse the token stream into a SystemVerilog CST as defined in [`forj_syntax`]
///
/// ```rust
/// # use forj_parser::*;
/// # let mut state = PreprocessorState::new(vec![], vec![]);
/// # let cache = PreprocessorCache::new();
/// let file_contents = "module test_module; endmodule".as_bytes();
/// let tokens = lex(file_contents, "test_file.v").tokens();
/// let pp_tokens = preprocess(tokens, &mut state, &cache).unwrap();
/// let ast: forj_syntax::SourceText<'_> = parse(&pp_tokens).unwrap();
/// let descriptions = &ast.2;
/// assert!(matches!(descriptions.first(), Some(forj_syntax::Description::ModuleDeclaration(_))))
/// ```
pub fn parse<'s>(
    input: &'s [SpannedToken<'s>],
) -> Result<SourceText<'s>, VerboseError<'s>> {
    let stream = TokenSlice::new(input);
    let mut state = ParserState::new(&stream);
    let mut stateful_input = Tokens {
        input: stream,
        state: &mut state,
    };
    let result = source_text_parser.parse_next(&mut stateful_input);
    match result {
        Ok(source_text) => Ok(source_text),
        Err(ErrMode::Backtrack(err)) | Err(ErrMode::Cut(err)) => {
            Err(match state.last_err {
                None => err,
                Some(prev_err) => err.or(prev_err),
            })
        }
        Err(ErrMode::Incomplete(_)) => {
            panic!("Produced 'incomplete', an unsupported error")
        }
    }
}
