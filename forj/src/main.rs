// =======================================================================
// main.rs
// =======================================================================
// The top-level code for forj

use clap::{Args, Parser, Subcommand};
use forj_parser::report::Sources;
use forj_parser::*;
use std::path::PathBuf;
mod cli;
mod constructs;
use constructs::constructs;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format source files in-place
    Format(FormatArgs),

    /// Check the constructs used
    Constructs(constructs::ConstructsArgs),
}

// -----------------------------------------------------------------------
// format
// -----------------------------------------------------------------------

#[derive(Args)]
struct FormatArgs {
    /// The file(s) to format
    paths: Vec<String>,

    /// A directory to dump debug output to
    #[arg(short, long)]
    debug: Option<PathBuf>,
}

fn format(args: &FormatArgs) {
    for path in &args.paths {
        let src = std::fs::read_to_string(&path).unwrap();
        let string_cache = PreprocessorCache::new();
        let mut state = PreprocessorState::new(vec![], vec![]);
        let (_, src) = state.retain_file(path.clone(), src, &string_cache);
        let lexed_src = lex(src, path.as_str()).process();
        if let Some(debug_path) = &args.debug {
            match lexed_src.dump(&debug_path) {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }
        let lex_errors = lexed_src.report_errors().collect::<Vec<_>>();
        if !lex_errors.is_empty() {
            let mut sources = state.included_files().sources();
            for report in lex_errors {
                report.print(&mut sources).unwrap();
            }
            return;
        }
        let token_stream = lexed_src.tokens();
        let preprocess_result =
            preprocess(token_stream, &mut state, &string_cache);
        let mut sources = state.included_files().sources();
        if !state.errors.is_empty() {
            for err in state.errors {
                let report: report::Report = (&err).into();
                report.print(&mut sources).unwrap();
            }
        }
        let preprocessed_stream = match preprocess_result {
            Err(_) => {
                return;
            }
            Ok(preprocessed_stream) => preprocessed_stream,
        };
        let parsed_src = parse(&preprocessed_stream);
        if let Err(err) = parsed_src {
            let report: report::Report = err.report("P1");
            report.print(&mut sources).unwrap();
            return;
        }
        // println!("{:#?}", parsed_src);
    }
}

// fn format(args: &FormatArgs) {
//     for path in &args.paths {
//         let src = std::fs::read_to_string(&path).unwrap();
//         let result = lex(&src);
//         for (token, span) in &result {
//             println!("{:?} ({:?})", token, span);
//         }
//         for report in report_lex_errors(result, &path) {
//             report
//                 .eprint((path.as_str(), Source::from(src.as_str())))
//                 .unwrap()
//         }
//     }
// }

// -----------------------------------------------------------------------
// main
// -----------------------------------------------------------------------

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Format(args) => format(&args),
        Commands::Constructs(args) => constructs(&args),
    }
}
