// =======================================================================
// conditional_compilation.rs
// =======================================================================
// Preprocessing for conditional compiler directives

use crate::Span;
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroIdentifier<'a>(pub &'a str, pub Span<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum IfdefCondition<'a> {
    TextMacro(Box<TextMacroIdentifier<'a>>),
    ParenMacro(
        Box<(
            Span<'a>, // (
            IfdefMacroExpression<'a>,
            Span<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IfdefMacroExpression<'a> {
    Text(Box<TextMacroIdentifier<'a>>),
    Operator(
        Box<(
            IfdefMacroExpression<'a>,
            BinaryLogicalOperator<'a>,
            IfdefMacroExpression<'a>,
        )>,
    ),
    Not(
        Box<(
            Span<'a>, // !
            IfdefMacroExpression<'a>,
        )>,
    ),
    Paren(
        Box<(
            Span<'a>, // (
            IfdefMacroExpression<'a>,
            Span<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryLogicalOperator<'a> {
    AmpAmp(Span<'a>),
    PipePipe(Span<'a>),
    Implication(Span<'a>),
    Equivalence(Span<'a>),
}

fn preprocess_untaken_conditional<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
) -> Result<(), PreprocessorError<'s>> {
    loop {
        match src.next() {
            None => {
                break Ok(());
            }
            Some(SpannedToken(Token::DirEndif, endif_span)) => {
                break Err(PreprocessorError::Endif { endif_span });
            }
            Some(SpannedToken(Token::DirElse, else_span)) => {
                break Err(PreprocessorError::Else { else_span });
            }
            Some(SpannedToken(Token::DirElsif, elsif_span)) => {
                break Err(PreprocessorError::Elsif { elsif_span });
            }
            Some(SpannedToken(Token::DirIfdef, _))
            | Some(SpannedToken(Token::DirIfndef, _)) => {
                'inner_conditional: loop {
                    match preprocess_untaken_conditional(src) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(PreprocessorError::Endif { .. }) => {
                            break 'inner_conditional;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

fn get_ifdef_condition<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    ifdef_span: Span<'s>,
) -> Result<IfdefCondition<'s>, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: ifdef_span,
        });
    };
    match spanned_token.0 {
        Token::SimpleIdentifier(id_str) => Ok(IfdefCondition::TextMacro(
            Box::new(TextMacroIdentifier(id_str, spanned_token.1)),
        )),
        Token::EscapedIdentifier(id_str) => Ok(IfdefCondition::TextMacro(
            Box::new(TextMacroIdentifier(id_str, spanned_token.1)),
        )),
        Token::Paren => {
            let ifdef_macro_expression =
                get_ifdef_macro_expression(src, ifdef_span, 0)?;
            let Some(eparen_token) = src.next() else {
                return Err(PreprocessorError::VerboseError {
                    err: VerboseError {
                        span: spanned_token.1,
                        found: None,
                        expected: vec![Expectation::Token(Token::EParen)],
                    },
                });
            };
            Ok(IfdefCondition::ParenMacro(Box::new((
                spanned_token.1,
                ifdef_macro_expression,
                eparen_token.1,
            ))))
        }
        _ => {
            return Err(PreprocessorError::VerboseError {
                err: VerboseError {
                    span: spanned_token.1,
                    found: Some(spanned_token.0),
                    expected: vec![Expectation::Label(
                        "a preprocessor macro expression",
                    )],
                },
            });
        }
    }
}

#[inline(always)]
fn amp_amp_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(6)
}

#[inline(always)]
fn pipe_pipe_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(5)
}

#[inline(always)]
fn implication_operator_binding_power<'s>() -> (u8, u8) {
    right_assoc(3)
}

#[inline(always)]
fn equivalence_operator_binding_power<'s>() -> (u8, u8) {
    right_assoc(3)
}

fn get_ifdef_macro_expression<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    previous_span: Span<'s>,
    min_bp: u8,
) -> Result<IfdefMacroExpression<'s>, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective {
            directive_span: previous_span,
        });
    };
    let mut lhs = match spanned_token.0 {
        Token::SimpleIdentifier(id_str) => IfdefMacroExpression::Text(
            Box::new(TextMacroIdentifier(id_str, spanned_token.1)),
        ),
        Token::EscapedIdentifier(id_str) => IfdefMacroExpression::Text(
            Box::new(TextMacroIdentifier(id_str, spanned_token.1)),
        ),
        Token::Paren => {
            let inner_expression =
                get_ifdef_macro_expression(src, previous_span.clone(), 0)?;
            let Some(SpannedToken(Token::EParen, eparen_span)) = src.next()
            else {
                return Err(PreprocessorError::VerboseError {
                    err: VerboseError {
                        span: spanned_token.1,
                        found: Some(Token::Paren),
                        expected: vec![Expectation::Label(
                            "a closing parenthesis",
                        )],
                    },
                });
            };
            IfdefMacroExpression::Paren(Box::new((
                spanned_token.1,
                inner_expression,
                eparen_span,
            )))
        }
        Token::Exclamation => {
            let negated_expr =
                get_ifdef_macro_expression(src, previous_span.clone(), 255)?;
            IfdefMacroExpression::Not(Box::new((spanned_token.1, negated_expr)))
        }
        _ => {
            return Err(PreprocessorError::VerboseError {
                err: VerboseError {
                    span: spanned_token.1,
                    found: Some(spanned_token.0),
                    expected: vec![Expectation::Label(
                        "a preprocessor macro expression",
                    )],
                },
            });
        }
    };
    loop {
        let Some(spanned_token) = src.peek() else {
            return Ok(lhs);
        };
        let (op, r_bp) = match spanned_token.0 {
            Token::AmpAmp => {
                let (l_bp, r_bp) = amp_amp_operator_binding_power();
                if l_bp < min_bp {
                    return Ok(lhs);
                }
                (BinaryLogicalOperator::AmpAmp(spanned_token.1.clone()), r_bp)
            }
            Token::PipePipe => {
                let (l_bp, r_bp) = pipe_pipe_operator_binding_power();
                if l_bp < min_bp {
                    return Ok(lhs);
                }
                (
                    BinaryLogicalOperator::PipePipe(spanned_token.1.clone()),
                    r_bp,
                )
            }
            Token::MinusGt => {
                let (l_bp, r_bp) = implication_operator_binding_power();
                if l_bp < min_bp {
                    return Ok(lhs);
                }
                (
                    BinaryLogicalOperator::Implication(spanned_token.1.clone()),
                    r_bp,
                )
            }
            Token::LtMinusGt => {
                let (l_bp, r_bp) = equivalence_operator_binding_power();
                if l_bp < min_bp {
                    return Ok(lhs);
                }
                (
                    BinaryLogicalOperator::Equivalence(spanned_token.1.clone()),
                    r_bp,
                )
            }
            _ => return Ok(lhs),
        };
        let _ = src.next().unwrap(); // Consume peeked token
        let rhs = get_ifdef_macro_expression(src, previous_span.clone(), r_bp)?;
        lhs = IfdefMacroExpression::Operator(Box::new((lhs, op, rhs)));
    }
}

fn ifdef_condition_true<'s>(
    condition: IfdefCondition<'s>,
    state: &PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> bool {
    match condition {
        IfdefCondition::TextMacro(inner_box) => {
            let TextMacroIdentifier(text, _) = *inner_box;
            state.is_defined(text)
        }
        IfdefCondition::ParenMacro(inner_box) => match *inner_box {
            (_, expression, _) => {
                ifdef_expression_true(expression, state, cache)
            }
        },
    }
}

fn ifdef_expression_true<'s>(
    expression: IfdefMacroExpression<'s>,
    state: &PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> bool {
    match expression {
        IfdefMacroExpression::Text(inner_box) => {
            let TextMacroIdentifier(text, _) = *inner_box;
            state.is_defined(text)
        }
        IfdefMacroExpression::Paren(inner_box) => {
            let (_, expression, _) = *inner_box;
            ifdef_expression_true(expression, state, cache)
        }
        IfdefMacroExpression::Not(inner_box) => {
            let (_, expression) = *inner_box;
            !ifdef_expression_true(expression, state, cache)
        }
        IfdefMacroExpression::Operator(inner_box) => {
            let (first_expr, operator, second_expr) = *inner_box;
            match operator {
                BinaryLogicalOperator::AmpAmp(_) => {
                    ifdef_expression_true(first_expr, state, cache)
                        && ifdef_expression_true(second_expr, state, cache)
                }
                BinaryLogicalOperator::PipePipe(_) => {
                    ifdef_expression_true(first_expr, state, cache)
                        || ifdef_expression_true(second_expr, state, cache)
                }
                BinaryLogicalOperator::Implication(_) => {
                    (!ifdef_expression_true(first_expr, state, cache))
                        || ifdef_expression_true(second_expr, state, cache)
                }
                BinaryLogicalOperator::Equivalence(_) => {
                    ((!ifdef_expression_true(first_expr.clone(), state, cache))
                        || ifdef_expression_true(
                            second_expr.clone(),
                            state,
                            cache,
                        ))
                        && ((!ifdef_expression_true(second_expr, state, cache))
                            || ifdef_expression_true(first_expr, state, cache))
                }
            }
        }
    }
}

pub fn preprocess_ifdef<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
    mut ifdef_span: Span<'s>,
    is_ifdef: bool, // False for ifndef
) -> Result<(), PreprocessorError<'s>> {
    let ifdef_condition = get_ifdef_condition(src, ifdef_span.clone())?;
    let mut valid_condition_found =
        ifdef_condition_true(ifdef_condition, state, cache) ^ !is_ifdef;
    let mut curr_condition_valid = valid_condition_found;
    let mut conditional_token = if is_ifdef {
        Token::DirIfdef
    } else {
        Token::DirIfndef
    };
    loop {
        let result = if curr_condition_valid {
            preprocess_helper(src, dest, state, cache)
        } else {
            preprocess_untaken_conditional(src)
        };
        match result {
            Ok(()) => {
                return Err(PreprocessorError::NoEndif {
                    cond_token: conditional_token,
                    cond_token_span: ifdef_span,
                });
            }
            Err(PreprocessorError::Endif { .. }) => return Ok(()),
            Err(PreprocessorError::Elsif { elsif_span }) => {
                let ifdef_condition =
                    get_ifdef_condition(src, elsif_span.clone())?;
                if valid_condition_found {
                    curr_condition_valid = false;
                } else {
                    curr_condition_valid =
                        ifdef_condition_true(ifdef_condition, state, cache);
                    valid_condition_found = curr_condition_valid;
                };
                ifdef_span = elsif_span;
                conditional_token = Token::DirElsif;
                ()
            }
            Err(PreprocessorError::Else { else_span }) => {
                let result = if !valid_condition_found {
                    preprocess_helper(src, dest, state, cache)
                } else {
                    preprocess_untaken_conditional(src)
                };
                match result {
                    Ok(()) => {
                        return Err(PreprocessorError::NoEndif {
                            cond_token: Token::DirElse,
                            cond_token_span: else_span,
                        });
                    }
                    Err(PreprocessorError::Endif { .. }) => return Ok(()),
                    Err(err) => return Err(err),
                }
            }
            Err(err) => return Err(err),
        }
    }
}

#[test]
fn basic_ifdef() {
    check_preprocessor!(
        "`define TEST
        `ifdef TEST
        this_should_be_included
        `endif",
        vec![Token::SimpleIdentifier("this_should_be_included")]
    )
}

#[test]
fn basic_ifndef() {
    check_preprocessor!(
        "`ifndef TEST
        this_should_be_included
        `endif",
        vec![Token::SimpleIdentifier("this_should_be_included")]
    )
}

#[test]
fn stripped_ifdef() {
    check_preprocessor!(
        "
        `ifdef TEST
        this_shouldnt_be_included
        `endif",
        Vec::<Token<'_>>::new()
    )
}

#[test]
fn stripped_ifndef() {
    check_preprocessor!(
        "`define TEST
        `ifndef TEST
        this_shouldnt_be_included
        `endif",
        Vec::<Token<'_>>::new()
    )
}

#[test]
fn else_used() {
    check_preprocessor!(
        "
        `ifdef FAKE_MATH
        1 + 1 = 3
        `else
        1 + 1 = 2
        `endif",
        vec![
            Token::UnsignedNumber("1"),
            Token::Plus,
            Token::UnsignedNumber("1"),
            Token::Eq,
            Token::UnsignedNumber("2")
        ]
    )
}

#[test]
fn else_unused() {
    check_preprocessor!(
        "`define FIRST_EXPR
        `ifdef FIRST_EXPR
        use_this_signal
        `else
        use_other_signal
        `endif",
        vec![Token::SimpleIdentifier("use_this_signal"),]
    )
}

#[test]
fn elsif() {
    check_preprocessor!(
        "`define REAL_MATH
        `ifdef FAKE_MATH
        1 + 1 = 3
        `elsif REAL_MATH
        1 + 1 = 2
        `elsif OTHER
        1 + 1 = 1'bx
        `else
        1 + 1 = ?
        `endif",
        vec![
            Token::UnsignedNumber("1"),
            Token::Plus,
            Token::UnsignedNumber("1"),
            Token::Eq,
            Token::UnsignedNumber("2")
        ]
    )
}

#[test]
#[should_panic(expected = "a preprocessor macro expression")]
fn empty_ifdef() {
    check_preprocessor!(
        "`ifdef
        `endif",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic(expected = "a preprocessor macro expression")]
fn empty_ifndef() {
    check_preprocessor!(
        "`ifndef
        `endif",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic(expected = "a preprocessor macro expression")]
fn empty_elsif() {
    check_preprocessor!(
        "`ifdef TEST
        `elsif
        `endif",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic(expected = "IncompleteDirective")]
fn no_tokens() {
    check_preprocessor!("`ifdef", Vec::<Token<'_>>::new())
}

#[test]
#[should_panic(expected = "NoEndif")]
fn no_endif() {
    check_preprocessor!("`ifdef TEST", Vec::<Token<'_>>::new())
}

#[test]
fn negate_expression() {
    check_preprocessor!(
        "
        `ifdef ( !TEST )
        bruh_just_use_ifndef
        `endif",
        vec![Token::SimpleIdentifier("bruh_just_use_ifndef")]
    )
}

#[test]
fn and_expression() {
    check_preprocessor!(
        "`define APPLE
        `define BANANA
        `ifdef (APPLE && BANANA)
        both_true
        `endif
        `ifdef (APPLE && PEAR)
        one_true
        `endif
        `ifdef (PEAR && ORANGE)
        none_true
        `endif",
        vec![Token::SimpleIdentifier("both_true")]
    )
}

#[test]
fn or_expression() {
    check_preprocessor!(
        "`define APPLE
        `define BANANA
        `ifdef (APPLE || BANANA)
        both_true
        `endif
        `ifdef (APPLE || PEAR)
        one_true
        `endif
        `ifdef (PEAR || ORANGE)
        none_true
        `endif",
        vec![
            Token::SimpleIdentifier("both_true"),
            Token::SimpleIdentifier("one_true")
        ]
    )
}

#[test]
fn implication_expression() {
    check_preprocessor!(
        "`define APPLE
        `define BANANA
        `ifdef (APPLE -> BANANA)
        true_implies_true
        `endif
        `ifdef (APPLE -> PEAR)
        true_implies_false
        `endif
        `ifdef (PEAR -> BANANA)
        false_implies_true
        `endif
        `ifdef (PEAR -> ORANGE)
        false_implies_false
        `endif",
        vec![
            Token::SimpleIdentifier("true_implies_true"),
            Token::SimpleIdentifier("false_implies_true"),
            Token::SimpleIdentifier("false_implies_false"),
        ]
    )
}

#[test]
fn equivalence_expression() {
    check_preprocessor!(
        "`define APPLE
        `define BANANA
        `ifdef (APPLE <-> BANANA)
        true_true
        `endif
        `ifdef (APPLE <-> PEAR)
        true_false
        `endif
        `ifdef (PEAR <-> BANANA)
        false_true
        `endif
        `ifdef (PEAR <-> ORANGE)
        false_false
        `endif",
        vec![
            Token::SimpleIdentifier("true_true"),
            Token::SimpleIdentifier("false_false"),
        ]
    )
}

#[test]
fn elsif_expression() {
    check_preprocessor!(
        "`define APPLES
        `ifdef (APPLES && BANANAS)
        should_i_use_and
        `elsif (APPLES || BANANAS)
        oh_no_wait_use_or
        `endif",
        vec![Token::SimpleIdentifier("oh_no_wait_use_or")]
    )
}

#[test]
fn composite_expression() {
    check_preprocessor!(
        "`define APPLE
        `ifdef ((APPLE -> (BANANA || ORANGE)) <-> PEAR)
        whoa_multiple_operators
        `endif",
        vec![Token::SimpleIdentifier("whoa_multiple_operators")]
    )
}

#[test]
fn associativity() {
    check_preprocessor!(
        "`define MIDDLE
        `ifdef (BEGIN -> MIDDLE -> END)
        right_associative
        `else
        left_associative
        `endif",
        vec![Token::SimpleIdentifier("right_associative")]
    );
    // Equal precedence
    check_preprocessor!(
        "
        `ifdef (BEGIN -> MIDDLE <-> END)
        right_associative
        `else
        left_associative
        `endif",
        vec![Token::SimpleIdentifier("right_associative")]
    );
    check_preprocessor!(
        "`define MIDDLE
        `define END
        `ifdef (BEGIN <-> MIDDLE -> END)
        left_associative
        `else
        right_associative
        `endif",
        vec![Token::SimpleIdentifier("right_associative")]
    )
}

#[test]
fn precedence() {
    check_preprocessor!(
        "`define FIRST
        `ifdef ( !FIRST && SECOND )
        and_higher_than_not
        `else
        not_higher_than_and
        `endif",
        vec![Token::SimpleIdentifier("not_higher_than_and")]
    );
    check_preprocessor!(
        "
        `define END
        `ifdef (BEGIN && MIDDLE || END)
        and_higher_than_or
        `else
        or_higher_than_and
        `endif",
        vec![Token::SimpleIdentifier("and_higher_than_or")]
    );
    check_preprocessor!(
        "
        `define APPLE
        `define BANANA
        `ifdef (APPLE -> BANANA || CHERRY -> DATE)
        implication_higher_than_or
        `else
        or_higher_than_implication
        `endif",
        vec![Token::SimpleIdentifier("or_higher_than_implication")]
    );
    check_preprocessor!(
        "`define APPLE
        `ifdef (APPLE || BANANA <-> CHERRY)
        equivalence_higher_than_or
        `else
        or_higher_than_equivalence
        `endif",
        vec![Token::SimpleIdentifier("or_higher_than_equivalence")]
    )
}

#[test]
fn nested_ifndef_else() {
    check_preprocessor!(
        "`define TEST
        `ifdef TEST
        this_should_be_included
        `else
        `ifdef TEST
        this_should_not_be_included
        `endif
        `endif",
        vec![Token::SimpleIdentifier("this_should_be_included")]
    );
    check_preprocessor!(
        "`define TEST
        `ifdef TEST
        this_should_be_included
        `else
        `ifdef TEST2
        this_should_not_be_included
        `endif
        `endif",
        vec![Token::SimpleIdentifier("this_should_be_included")]
    );
    check_preprocessor!(
        "`define TEST
        `ifdef TEST
        this_should_be_included
        `elsif TEST
        `ifdef TEST
        this_should_not_be_included
        `endif
        `endif",
        vec![Token::SimpleIdentifier("this_should_be_included")]
    );
    check_preprocessor!(
        "`define TEST
        `ifdef TEST
        this_should_be_included
        `elsif TEST
        `ifdef TEST2
        this_should_not_be_included
        `endif
        `endif",
        vec![Token::SimpleIdentifier("this_should_be_included")]
    );
    check_preprocessor!(
        "`define TEST
        `ifdef TEST
        this_should_be_included
        `else
        `ifdef TEST
        `ifdef TEST
        `ifdef TEST
        this_should_not_be_included
        `endif
        `endif
        `endif
        `endif",
        vec![Token::SimpleIdentifier("this_should_be_included")]
    );
}
