// =======================================================================
// pp_cli.rs
// =======================================================================
// Preprocess whatever was provided on the command line (useful for
// testing). Includes aren't supported

use forj_parser::report::Sources;
use forj_parser::*;

fn format_span<'a>(span: &Span<'a>) -> String {
    let mut base_span = format!("[{}:{}]", span.bytes.start, span.bytes.end);
    if let Some(expanded_from) = span.expanded_from {
        base_span = format!("{}({})", base_span, format_span(expanded_from.1));
    };
    base_span
}

fn main() {
    let text = std::env::args().nth(1).expect("Usage: pp_cli text_to_pp");
    let string_cache = preprocessor::PreprocessorCache::new();
    let mut state = preprocessor::PreprocessorState::new(vec![], vec![]);
    let (_, src) = state.retain_file(
        "test".to_owned(),
        text.as_bytes().to_vec(),
        &string_cache,
    );
    let lexed_src = lex(src, "test");
    let token_stream = lexed_src.tokens();
    let preprocess_result = preprocess(token_stream, &mut state, &string_cache);
    let mut sources = state.included_files().sources();
    if !state.errors.is_empty() {
        for err in &state.errors {
            let report: report::Report = err.into();
            report.print(&mut sources).unwrap();
        }
    }
    let preprocessed_stream = match preprocess_result {
        Err(_) => {
            return;
        }
        Ok(preprocessed_stream) => preprocessed_stream,
    };
    for token in preprocessed_stream {
        println!(" - {} {}", format_span(&token.1), token.0);
    }
}
