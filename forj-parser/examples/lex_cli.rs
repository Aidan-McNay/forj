// =======================================================================
// lex_cli.rs
// =======================================================================
// Lex whatever was provided on the command line (useful for testing)

use forj_parser::*;

fn main() {
    let text = std::env::args().nth(1).expect("Usage: lex text_to_lex");
    let lexed_src = lex(&text, "test");
    for token in lexed_src.tokens() {
        println!(" - {}", token.0);
    }
}
