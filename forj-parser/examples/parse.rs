// =======================================================================
// iter_tree.rs
// =======================================================================
// Iterate across the AST of SystemVerilog source code

use clap::Parser;
use forj_parser::report::Sources;
use forj_parser::*;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    /// Directory to search for include paths
    #[arg(short = 'I', long, value_name = "DIR_PATH")]
    include: Vec<PathBuf>,

    /// Sets a preprocessor define
    #[arg(short = 'D', long, value_name = "NAME[=VALUE]")]
    define: Vec<String>,

    /// Whether to print the nodes
    #[arg(short, long)]
    print: bool,

    paths: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let args = Cli::parse();
    let string_cache = preprocessor::PreprocessorCache::new();
    let includes = args
        .include
        .iter()
        .map(|pathbuf| pathbuf.as_path())
        .collect::<Vec<_>>();
    let defines = args
        .define
        .iter()
        .map(|def| <std::string::String as AsRef<str>>::as_ref(def).into())
        .collect::<Vec<_>>();
    let mut state = preprocessor::PreprocessorState::new(includes, vec![]);
    for path in &args.paths {
        let src = std::fs::read_to_string(&path).unwrap();
        state.make_fresh(defines.clone());
        let (_, src) = state.retain_file(
            path.clone().into_os_string().into_string().unwrap(),
            src,
            &string_cache,
        );
        let lexed_src = lex(src, path.to_str().unwrap());
        let token_stream = lexed_src.tokens();
        let preprocess_result =
            preprocess(token_stream, &mut state, &string_cache);
        let mut sources = state.included_files().sources();
        if !state.errors.is_empty() {
            for err in &state.errors {
                let report: report::Report = err.into();
                report.print(&mut sources).unwrap();
            }
        }
        let preprocessed_stream = match preprocess_result {
            Err(_) => {
                return ExitCode::FAILURE;
            }
            Ok(preprocessed_stream) => preprocessed_stream,
        };
        let parsed_src = parse(&preprocessed_stream);
        if let Err(err) = parsed_src {
            let report: report::Report = err.report("P1");
            report.print(&mut sources).unwrap();
            return ExitCode::FAILURE;
        }
        let source_text = parsed_src.unwrap();
        if args.print {
            for node in source_text.iter() {
                println!("{}", node.name());
            }
        }
    }
    ExitCode::SUCCESS
}
