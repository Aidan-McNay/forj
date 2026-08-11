// =======================================================================
// cli.rs
// =======================================================================
// Helper functions for CLI parsing

use forj_parser::SpannedString;
use forj_parser::preprocessor::state::{Define, DefineBody};
use forj_parser::{LexedSource, lex};
use forj_syntax::Span;

fn _parse_cli_define<'a>(cli_define: &'a str) -> Define<'a> {
    if let Some((first, second)) = cli_define.split_once('=') {
        let name = SpannedString(first, Span::default());
        let body =
            DefineBody::Text(lex(second, "").tokens().collect::<Vec<_>>());
        Define { name, body }
    } else {
        todo!()
    }
}
