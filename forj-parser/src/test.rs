// =======================================================================
// test.rs
// =======================================================================
// Utilities for testing parsers

use forj_syntax::*;

#[macro_export]
macro_rules! check_lexer {
    ($input:expr, $expected:expr) => {{
        let input = lex($input, "<test>")
            .map(|a| a.0.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(input, $expected)
    }};
}

#[macro_export]
macro_rules! check_preprocessor {
    ($input:expr, $expected:expr) => {{
        let mut state = PreprocessorState::new(vec![], vec![]);
        let cache = PreprocessorCache::new();
        let (_, src) =
            state.retain_file("<test>".to_string(), $input.to_string(), &cache);
        let input = lex(src, "<test>").tokens().collect::<Vec<_>>();
        let preprocess_result = preprocess(
            &mut TokenIterator::new(input.into_iter()),
            &mut state,
            &cache,
        );
        match preprocess_result {
            Ok(result) => {
                assert_eq!(result, $expected);
                if let Some(err) = state.errors.first() {
                    panic!("{:?}", err)
                }
            }
            Err(()) => panic!("{:?}", state.errors.first()),
        }
    }};
}

#[macro_export]
macro_rules! apply_parser {
    ($input:literal, $parser:ident, $storage:expr) => {{
        *$storage = lex($input, "<test>").tokens().collect::<Vec<_>>();
        let mut tokens = Tokens {
            input: TokenSlice::new(&$storage[..]),
            state: None,
        };
        $parser(&mut tokens).unwrap()
    }};
}

#[macro_export]
macro_rules! check_parser {
    ($input:literal, $parser:ident, $expected:expr) => {{
        let input = lex($input, "<test>").tokens().collect::<Vec<_>>();
        let mut tokens = Tokens {
            input: TokenSlice::new(&input[..]),
            state: None,
        };
        assert_eq!($parser(&mut tokens).unwrap(), $expected)
    }};
}

pub fn test_metadata<'s>() -> Metadata<'s> {
    Metadata::new(Span::default(), vec![])
}
