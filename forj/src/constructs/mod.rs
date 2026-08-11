// =======================================================================
// mod.rs
// =======================================================================
/// Checking which constructs are used in a source file
use clap::Args;
use forj_parser::report::Sources;
use forj_parser::*;
use forj_syntax::{Node, NodeID, Nodes};
use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

#[derive(Args)]
pub struct ConstructsArgs {
    /// The file(s) to check the constructs in
    paths: Vec<PathBuf>,

    /// Directory to search for include paths
    #[arg(short = 'I', long, value_name = "DIR_PATH")]
    include: Vec<PathBuf>,

    /// Sets a preprocessor define
    #[arg(short = 'D', long, value_name = "NAME[=VALUE]")]
    define: Vec<String>,

    /// A construct to allow
    #[arg(short = 'a', long, value_name = "CONSTRUCT")]
    allow: Vec<String>,

    /// A construct to disallow
    #[arg(short = 'd', long, value_name = "CONSTRUCT")]
    disallow: Vec<String>,

    /// A directory to dump debug output to
    #[arg(long, value_name = "DIR_PATH")]
    debug: Option<PathBuf>,
}

/// A collection of allowed constructs in a SystemVerilog source file
pub struct AllowedConstructs {
    ids: HashSet<NodeID>,
}

impl Default for AllowedConstructs {
    fn default() -> Self {
        Self {
            ids: HashSet::new(),
        }
    }
}

impl Deref for AllowedConstructs {
    type Target = HashSet<NodeID>;
    fn deref(&self) -> &Self::Target {
        &self.ids
    }
}

impl DerefMut for AllowedConstructs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ids
    }
}

impl AllowedConstructs {
    pub fn parse_allow(
        &mut self,
        allowed_constructs: &Vec<String>,
    ) -> Result<(), String> {
        for construct in allowed_constructs {
            let id = match NodeID::try_from_name(construct.as_str()) {
                Some(id) => id,
                None => {
                    return Err(format!(
                        "Error: {} (passed with `--allow`) isn't a valid SystemVerilog construct",
                        construct
                    ));
                }
            };
            if !self.ids.insert(id) {
                return Err(format!(
                    "Error: {} (passed with `--allow`) was allowed multiple times",
                    construct
                ));
            }
        }
        Ok(())
    }
    pub fn parse_disallow(
        &mut self,
        disallowed_constructs: &Vec<String>,
    ) -> Result<(), String> {
        for construct in disallowed_constructs {
            let id = match NodeID::try_from_name(construct.as_str()) {
                Some(id) => id,
                None => {
                    return Err(format!(
                        "Error: {} (passed with `--disallow`) isn't a valid SystemVerilog construct",
                        construct
                    ));
                }
            };
            if !self.ids.remove(&id) {
                return Err(format!(
                    "Error: {} (passed with `--disallow`) wasn't allowed already",
                    construct
                ));
            }
        }
        Ok(())
    }
    /// Whether the given `node` is allowed
    pub fn allowed(&self, node: &Node) -> bool {
        self.ids.contains(&NodeID::from_node(node))
    }
}

fn find_disallowed<'a, 'b>(
    node: &Node<'a, 'b>,
    allowed_constructs: &AllowedConstructs,
) -> Vec<(NodeID, Span<'a>)> {
    if allowed_constructs.allowed(node) {
        node.children()
            .iter()
            .map(|child_node| find_disallowed(child_node, allowed_constructs))
            .flatten()
            .collect()
    } else {
        vec![(NodeID::from_node(node), node.span())]
    }
}

pub fn constructs(args: &ConstructsArgs) {
    let string_cache = PreprocessorCache::new();
    let mut state = PreprocessorState::new(
        args.include
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect(),
        vec![],
    );
    let mut allowed_constructs = AllowedConstructs::default();
    match allowed_constructs.parse_allow(&args.allow) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    match allowed_constructs.parse_disallow(&args.disallow) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    for path in &args.paths {
        let src = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                println! {"Error reading {}: {}", path.to_string_lossy(), err};
                continue;
            }
        };
        state.make_fresh(vec![]);
        let (path, src) = state.retain_file(
            path.to_string_lossy().to_string(),
            src,
            &string_cache,
        );
        let lexed_src = lex(src, path).process();
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
        let parsed_src = parse(&preprocessed_stream);
        let source_text = match parsed_src {
            Err(err) => {
                let report: report::Report = err.report("P1");
                report.print(&mut sources).unwrap();
                return;
            }
            Ok(source_text) => source_text,
        };
        let disallowed_nodes =
            find_disallowed(&((&source_text).into()), &allowed_constructs);
        if disallowed_nodes.is_empty() {
            println!("No invalid constructs in {}", path);
        } else {
            for disallowed_node in disallowed_nodes {
                println!(
                    "Found disallowed node: {}",
                    NodeID::into_name(disallowed_node.0)
                )
            }
        }
    }
}
