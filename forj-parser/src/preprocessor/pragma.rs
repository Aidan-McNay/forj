// =======================================================================
// state.rs
// =======================================================================
//! Preprocessing for compiler pragmas

use crate::*;

/// A value in a `` `pragma `` directive
pub enum PragmaValue<'a> {
    Number(&'a str),
    Identifier(&'a str),
    String(&'a str),
    Expressions(Box<Vec<PragmaExpression<'a>>>),
}

/// An expression for a `` `pragma `` directive
pub enum PragmaExpression<'a> {
    Keyword(&'a str),
    Value(PragmaValue<'a>),
    KeywordValue(&'a str, PragmaValue<'a>),
}

/// An object that can be called when instances of `` `pragma ``
/// are found
pub trait PragmaHandler<'a> {
    /// Encountered a `` `pragma resetall ``, or a `` `pragma reset ``
    /// specifying this handler
    fn reset(&mut self);
    /// Encountered a `` `pragma `` for this handler
    fn callback(
        &mut self,
        name: &'a str,
        name_span: &Span<'a>,
        args: Vec<PragmaExpression<'a>>,
        dest: &mut Vec<SpannedToken<'a>>,
    );
}

/// Convert a [`Token`] into an identifier when possible
fn into_identifier<'a>(token: &Token<'a>) -> Option<&'static str> {
    match token {
        Token::Always
        | Token::And
        | Token::Assign
        | Token::Begin
        | Token::Buf
        | Token::Bufif0
        | Token::Bufif1
        | Token::Case
        | Token::Casex
        | Token::Casez
        | Token::Cmos
        | Token::Deassign
        | Token::Default
        | Token::Defparam
        | Token::Disable
        | Token::Edge
        | Token::Else
        | Token::End
        | Token::Endcase
        | Token::Endfunction
        | Token::Endmodule
        | Token::Endprimitive
        | Token::Endspecify
        | Token::Endtable
        | Token::Endtask
        | Token::Event
        | Token::For
        | Token::Force
        | Token::Forever
        | Token::Fork
        | Token::Function
        | Token::Highz0
        | Token::Highz1
        | Token::If
        | Token::Ifnone
        | Token::Initial
        | Token::Inout
        | Token::Input
        | Token::Integer
        | Token::Join
        | Token::Large
        | Token::Macromodule
        | Token::Medium
        | Token::Module
        | Token::Nand
        | Token::Negedge
        | Token::Nmos
        | Token::Nor
        | Token::Not
        | Token::Notif0
        | Token::Notif1
        | Token::Or
        | Token::Output
        | Token::Parameter
        | Token::Pmos
        | Token::Posedge
        | Token::Primitive
        | Token::Pull0
        | Token::Pull1
        | Token::Pulldown
        | Token::Pullup
        | Token::Rcmos
        | Token::Real
        | Token::Realtime
        | Token::Reg
        | Token::Release
        | Token::Repeat
        | Token::Rnmos
        | Token::Rpmos
        | Token::Rtran
        | Token::Rtranif0
        | Token::Rtranif1
        | Token::Scalared
        | Token::Small
        | Token::Specify
        | Token::Specparam
        | Token::Strong0
        | Token::Strong1
        | Token::Supply0
        | Token::Supply1
        | Token::Table
        | Token::Task
        | Token::Time
        | Token::Tran
        | Token::Tranif0
        | Token::Tranif1
        | Token::Tri
        | Token::Tri0
        | Token::Tri1
        | Token::Triand
        | Token::Trior
        | Token::Trireg
        | Token::Vectored
        | Token::Wait
        | Token::Wand
        | Token::Weak0
        | Token::Weak1
        | Token::While
        | Token::Wire
        | Token::Wor
        | Token::Xnor
        | Token::Xor
        | Token::Automatic
        | Token::Cell
        | Token::Config
        | Token::Design
        | Token::Endconfig
        | Token::Endgenerate
        | Token::Generate
        | Token::Genvar
        | Token::Incdir
        | Token::Include
        | Token::Instance
        | Token::Liblist
        | Token::Library
        | Token::Localparam
        | Token::Noshowcancelled
        | Token::PulsestyleOndetect
        | Token::PulsestyleOnevent
        | Token::Showcancelled
        | Token::Signed
        | Token::Unsigned
        | Token::Use
        | Token::Uwire
        | Token::Alias
        | Token::AlwaysComb
        | Token::AlwaysFf
        | Token::AlwaysLatch
        | Token::Assert
        | Token::Assume
        | Token::Before
        | Token::Bind
        | Token::Bins
        | Token::Binsof
        | Token::Bit
        | Token::Break
        | Token::Byte
        | Token::Chandle
        | Token::Class
        | Token::Clocking
        | Token::Const
        | Token::Constraint
        | Token::Context
        | Token::Continue
        | Token::Cover
        | Token::Covergroup
        | Token::Coverpoint
        | Token::Cross
        | Token::Dist
        | Token::Do
        | Token::Endclass
        | Token::Endclocking
        | Token::Endgroup
        | Token::Endinterface
        | Token::Endpackage
        | Token::Endprogram
        | Token::Endproperty
        | Token::Endsequence
        | Token::Enum
        | Token::Expect
        | Token::Export
        | Token::Extends
        | Token::Extern
        | Token::Final
        | Token::FirstMatch
        | Token::Foreach
        | Token::Forkjoin
        | Token::Iff
        | Token::IgnoreBins
        | Token::IllegalBins
        | Token::Import
        | Token::Inside
        | Token::Int
        | Token::Interface
        | Token::Intersect
        | Token::JoinAny
        | Token::JoinNone
        | Token::Local
        | Token::Logic
        | Token::Longint
        | Token::Matches
        | Token::Modport
        | Token::New
        | Token::Null
        | Token::Package
        | Token::Packed
        | Token::Priority
        | Token::Program
        | Token::Property
        | Token::Protected
        | Token::Pure
        | Token::Rand
        | Token::Randc
        | Token::Randcase
        | Token::Randsequence
        | Token::Ref
        | Token::Return
        | Token::Sequence
        | Token::Shortint
        | Token::Shortreal
        | Token::Solve
        | Token::Static
        | Token::String
        | Token::Struct
        | Token::Super
        | Token::Tagged
        | Token::This
        | Token::Throughout
        | Token::Timeprecision
        | Token::Timeunit
        | Token::Type
        | Token::Typedef
        | Token::Union
        | Token::Unique
        | Token::Var
        | Token::Virtual
        | Token::Void
        | Token::WaitOrder
        | Token::Wildcard
        | Token::With
        | Token::Within
        | Token::AcceptOn
        | Token::Checker
        | Token::Endchecker
        | Token::Eventually
        | Token::Global
        | Token::Implies
        | Token::Let
        | Token::Nexttime
        | Token::RejectOn
        | Token::Restrict
        | Token::SAlways
        | Token::SEventually
        | Token::SNexttime
        | Token::SUntil
        | Token::SUntilWith
        | Token::Strong
        | Token::SyncAcceptOn
        | Token::SyncRejectOn
        | Token::Unique0
        | Token::Until
        | Token::UntilWith
        | Token::Untyped
        | Token::Weak
        | Token::Implements
        | Token::Interconnect
        | Token::Nettype
        | Token::Soft
        | Token::PathpulseDollar
        | Token::OneStep => Some(token.as_str()),
        _ => None,
    }
}

pub(crate) fn get_pragma_name<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    pragma_span: &Span<'s>,
) -> Result<(&'s str, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, state, cache)? else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: pragma_span.clone(),
        });
    };
    match spanned_token.0 {
        Token::SimpleIdentifier(text) => Ok((
            unsafe { std::str::from_utf8_unchecked(text) },
            spanned_token.1,
        )),
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a pragma name")],
            },
        }),
    }
}

pub(crate) fn get_pragma_value<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    curr_token: SpannedToken<'s>,
    keyword_possible: bool,
    pragma_span: &Span<'s>,
) -> Result<PragmaValue<'s>, PreprocessorError<'s>> {
    match curr_token.0 {
        Token::Paren => {
            let expressions =
                get_pragma_expressions(src, state, cache, pragma_span)?;
            let Some(spanned_token) = preprocess_single(src, state, cache)?
            else {
                return Err(PreprocessorError::VerboseError {
                    err: VerboseError {
                        span: curr_token.1,
                        found: None,
                        expected: vec![Expectation::Label("a corresponding )")],
                    },
                });
            };
            if spanned_token.0 != Token::EParen {
                return Err(PreprocessorError::VerboseError {
                    err: VerboseError {
                        span: spanned_token.1,
                        found: Some(spanned_token.0),
                        expected: vec![Expectation::Token(Token::EParen)],
                    },
                });
            }
            Ok(PragmaValue::Expressions(Box::new(expressions)))
        }
        Token::UnsignedNumber(text)
        | Token::FixedPointNumber(text)
        | Token::BinaryNumber(text)
        | Token::OctalNumber(text)
        | Token::DecimalNumber(text)
        | Token::HexNumber(text)
        | Token::ScientificNumber(text) => Ok(PragmaValue::Number(unsafe {
            std::str::from_utf8_unchecked(text)
        })),
        Token::SimpleIdentifier(text) | Token::EscapedIdentifier(text) => {
            Ok(PragmaValue::Identifier(unsafe {
                std::str::from_utf8_unchecked(text)
            }))
        }
        Token::StringLiteral(text) => Ok(PragmaValue::String(unsafe {
            std::str::from_utf8_unchecked(text)
        })),
        _ if let Some(ident_text) = into_identifier(&curr_token.0) => {
            Ok(PragmaValue::String(ident_text))
        }
        _ => Err(PreprocessorError::VerboseError {
            err: VerboseError {
                span: curr_token.1,
                found: Some(curr_token.0),
                expected: if keyword_possible {
                    vec![Expectation::Label("a pragma expression")]
                } else {
                    vec![Expectation::Label("a pragma value")]
                },
            },
        }),
    }
}

pub(crate) fn check_for_expressions<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<bool, PreprocessorError<'s>> {
    match preprocess_single(src, state, cache)? {
        None => Ok(false),
        Some(spanned_token) => {
            let use_expressions = match spanned_token.0 {
                Token::Paren
                | Token::UnsignedNumber(_)
                | Token::FixedPointNumber(_)
                | Token::BinaryNumber(_)
                | Token::OctalNumber(_)
                | Token::DecimalNumber(_)
                | Token::HexNumber(_)
                | Token::ScientificNumber(_)
                | Token::SimpleIdentifier(_)
                | Token::EscapedIdentifier(_)
                | Token::StringLiteral(_) => true,
                _ if let Some(_) = into_identifier(&spanned_token.0) => true,
                _ => false,
            };
            src.prepend_tokens(std::iter::once(spanned_token));
            Ok(use_expressions)
        }
    }
}

pub(crate) fn get_pragma_expressions<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    pragma_span: &Span<'s>,
) -> Result<Vec<PragmaExpression<'s>>, PreprocessorError<'s>> {
    let mut expressions = vec![];
    loop {
        let Some(spanned_token) = preprocess_single(src, state, cache)? else {
            return Err(PreprocessorError::VerboseError {
                err: VerboseError {
                    span: pragma_span.clone(),
                    found: None,
                    expected: vec![Expectation::Label("a pragma expression")],
                },
            });
        };
        let next_expression = match spanned_token.0 {
            Token::SimpleIdentifier(text) => {
                if let Some(SpannedToken(Token::Eq, _)) = src.peek() {
                    src.next(); // Eq
                    let Some(value_token) =
                        preprocess_single(src, state, cache)?
                    else {
                        return Err(PreprocessorError::VerboseError {
                            err: VerboseError {
                                span: pragma_span.clone(),
                                found: None,
                                expected: vec![Expectation::Label(
                                    "a pragma value",
                                )],
                            },
                        });
                    };
                    let value = get_pragma_value(
                        src,
                        state,
                        cache,
                        value_token,
                        false,
                        pragma_span,
                    )?;
                    PragmaExpression::KeywordValue(
                        unsafe { std::str::from_utf8_unchecked(text) },
                        value,
                    )
                } else {
                    PragmaExpression::Keyword(unsafe {
                        std::str::from_utf8_unchecked(text)
                    })
                }
            }
            _ => {
                let value = get_pragma_value(
                    src,
                    state,
                    cache,
                    spanned_token,
                    true,
                    pragma_span,
                )?;
                PragmaExpression::Value(value)
            }
        };
        expressions.push(next_expression);
        let next_token = preprocess_single(src, state, cache)?;
        if let Some(SpannedToken(Token::Comma, _)) = next_token {
            ()
        } else {
            if let Some(token) = next_token {
                src.prepend_tokens(std::iter::once(token));
            }
            break;
        }
    }
    Ok(expressions)
}

pub(crate) fn pragma_reset<'s>(
    _state: &mut PreprocessorState<'s>,
    _args: Vec<PragmaExpression<'s>>,
) {
    todo!()
}

pub(crate) fn pragma_resetall<'s>(
    _state: &mut PreprocessorState<'s>,
    _args: Vec<PragmaExpression<'s>>,
) {
    todo!()
}

pub(crate) fn preprocess_pragma<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    pragma_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let (pragma_name, name_span) =
        get_pragma_name(src, state, cache, &pragma_span)?;
    let pragma_args = if check_for_expressions(src, state, cache)? {
        get_pragma_expressions(src, state, cache, &pragma_span)?
    } else {
        vec![]
    };
    match pragma_name {
        "reset" => {
            pragma_reset(state, pragma_args);
            Ok(())
        }
        "resetall" => {
            pragma_resetall(state, pragma_args);
            Ok(())
        }
        _ => {
            let Some(handler) = state.pragma_handlers.get_mut(pragma_name)
            else {
                // TODO: Warn no registered handler
                return Ok(());
            };
            handler.callback(pragma_name, &name_span, pragma_args, dest);
            Ok(())
        }
    }
}

#[cfg(test)]
struct BasicHandler {
    name: String,
    num_expr: usize,
}

#[cfg(test)]
impl BasicHandler {
    fn new() -> Self {
        Self {
            name: String::new(),
            num_expr: 0,
        }
    }
}

#[cfg(test)]
impl<'a> PragmaHandler<'a> for BasicHandler {
    fn reset(&mut self) {}
    fn callback(
        &mut self,
        name: &'a str,
        _name_span: &Span<'a>,
        args: Vec<PragmaExpression<'a>>,
        _dest: &mut Vec<SpannedToken<'a>>,
    ) {
        self.name = name.to_string();
        self.num_expr = args.len();
    }
}

#[cfg(test)]
fn test_pragma(
    input: &str,
    name: &str,
    num_expr: usize,
    expected: Vec<Token<'static>>,
) {
    let mut state = PreprocessorState::new(vec![], vec![]);
    let cache = PreprocessorCache::new();
    let (_, src) = state.retain_file(
        "<test>".to_string(),
        input.to_string().into_bytes(),
        &cache,
    );
    let mut handler = BasicHandler::new();
    state.set_pragma_handler(name, &mut handler);
    let input = lex(src, "<test>").tokens().collect::<Vec<_>>();
    let preprocess_result = preprocess(
        &mut TokenIterator::new(input.into_iter()),
        &mut state,
        &cache,
    );
    match preprocess_result {
        Ok(result) => {
            assert_eq!(result, expected);
            if let Some(err) = state.errors.first() {
                panic!("{:?}", err)
            }
        }
        Err(()) => panic!("{:?}", state.errors.first()),
    }
    assert_eq!(handler.name, name);
    assert_eq!(handler.num_expr, num_expr)
}

#[test]
fn basic() {
    test_pragma("`pragma hello", "hello", 0, vec![])
}

#[test]
fn keyword() {
    test_pragma(
        "`pragma keyword these, are, some, keywords",
        "keyword",
        4,
        vec![],
    )
}

#[test]
fn dictionary() {
    test_pragma(
        "`pragma dictionary baba=you, count=5",
        "dictionary",
        2,
        vec![],
    )
}

#[test]
fn nested() {
    test_pragma(
        "`pragma nested outer    =   (middle=\"this\", inner=(i_am=your_father, depth = 2))",
        "nested",
        1,
        vec![],
    )
}

#[test]
fn empty() {
    test_pragma("`pragma empty", "empty", 0, vec![])
}

#[test]
#[should_panic(expected = "a pragma name")]
fn no_name() {
    test_pragma(
        "`pragma
        module test(); endmodule",
        "nested",
        0,
        vec![],
    )
}

#[test]
#[should_panic(expected = "a pragma value")]
fn no_value() {
    test_pragma("`pragma no_value item = module", "no_value", 1, vec![])
}

#[test]
#[should_panic(expected = "a pragma value")]
fn no_value_eof() {
    test_pragma("`pragma no_value item =", "no_value", 1, vec![])
}

#[test]
#[should_panic(expected = "a pragma expression")]
fn no_expression() {
    test_pragma(
        "`pragma no_expression first_item, class",
        "no_expression",
        1,
        vec![],
    )
}

#[test]
#[should_panic(expected = "a pragma expression")]
fn no_expression_eof() {
    test_pragma(
        "`pragma no_expression first_item, ",
        "no_expression",
        1,
        vec![],
    )
}

#[test]
fn pragma_replace_keyword() {
    // class should be an identifier, not a keyword
    test_pragma(
        "`begin_keywords \"1364-1995\"
        `pragma class_pragma first_item, class
        `end_keywords",
        "class_pragma",
        2,
        vec![],
    )
}
