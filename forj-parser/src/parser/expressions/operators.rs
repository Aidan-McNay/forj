// =======================================================================
// operators.rs
// =======================================================================
// Parsing for 1800-2023 A.8.6

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn unary_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UnaryOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::Plus).map(|a| UnaryOperator::Plus(a)),
        token(Token::Minus).map(|a| UnaryOperator::Minus(a)),
        token(Token::Exclamation).map(|a| UnaryOperator::Exclamation(a)),
        token(Token::Tilde).map(|a| UnaryOperator::Tilde(a)),
        token(Token::Amp).map(|a| UnaryOperator::Amp(a)),
        token(Token::TildeAmp).map(|a| UnaryOperator::TildeAmp(a)),
        token(Token::Pipe).map(|a| UnaryOperator::Pipe(a)),
        token(Token::TildePipe).map(|a| UnaryOperator::TildePipe(a)),
        token(Token::Caret).map(|a| UnaryOperator::Caret(a)),
        token(Token::TildeCaret).map(|a| UnaryOperator::TildeCaret(a)),
        token(Token::CaretTilde).map(|a| UnaryOperator::CaretTilde(a)),
    ))
    .parse_next(input)
}

pub fn binary_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinaryOperator<'s>, VerboseError<'s>> {
    // Note: Got errors when combining these into one `choice`, seemingly because Rust used
    // alphabetic identifiers for each parser type, and didn't like more than 26...
    let temp1 = alt((
        token(Token::Plus).map(|a| BinaryOperator::Plus(a)),
        token(Token::Minus).map(|a| BinaryOperator::Minus(a)),
        token(Token::Star).map(|a| BinaryOperator::Star(a)),
        token(Token::Slash).map(|a| BinaryOperator::Slash(a)),
        token(Token::Percent).map(|a| BinaryOperator::Percent(a)),
        token(Token::EqEq).map(|a| BinaryOperator::EqEq(a)),
        token(Token::ExclEq).map(|a| BinaryOperator::ExclEq(a)),
        token(Token::EqEqEq).map(|a| BinaryOperator::EqEqEq(a)),
        token(Token::ExclEqEq).map(|a| BinaryOperator::ExclEqEq(a)),
        token(Token::EqEqQuest).map(|a| BinaryOperator::EqEqQuest(a)),
        token(Token::ExclEqQuest).map(|a| BinaryOperator::ExclEqQuest(a)),
        token(Token::AmpAmp).map(|a| BinaryOperator::AmpAmp(a)),
        token(Token::PipePipe).map(|a| BinaryOperator::PipePipe(a)),
        token(Token::StarStar).map(|a| BinaryOperator::StarStar(a)),
    ));
    let temp2 = alt((
        token(Token::Lt).map(|a| BinaryOperator::Lt(a)),
        token(Token::LtEq).map(|a| BinaryOperator::LtEq(a)),
        token(Token::Gt).map(|a| BinaryOperator::Gt(a)),
        token(Token::GtEq).map(|a| BinaryOperator::GtEq(a)),
        token(Token::Amp).map(|a| BinaryOperator::Amp(a)),
        token(Token::Pipe).map(|a| BinaryOperator::Pipe(a)),
        token(Token::Caret).map(|a| BinaryOperator::Caret(a)),
        token(Token::CaretTilde).map(|a| BinaryOperator::CaretTilde(a)),
        token(Token::TildeCaret).map(|a| BinaryOperator::TildeCaret(a)),
        token(Token::GtGt).map(|a| BinaryOperator::GtGt(a)),
        token(Token::LtLt).map(|a| BinaryOperator::LtLt(a)),
        token(Token::GtGtGt).map(|a| BinaryOperator::GtGtGt(a)),
        token(Token::LtLtLt).map(|a| BinaryOperator::LtLtLt(a)),
        token(Token::MinusGt).map(|a| BinaryOperator::MinusGt(a)),
        token(Token::LtMinusGt).map(|a| BinaryOperator::LtMinusGt(a)),
    ));
    alt((temp1, temp2)).parse_next(input)
}

pub fn inc_or_dec_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IncOrDecOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::PlusPlus).map(|a| IncOrDecOperator::PlusPlus(a)),
        token(Token::MinusMinus).map(|a| IncOrDecOperator::MinusMinus(a)),
    ))
    .parse_next(input)
}

pub fn unary_module_path_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UnaryModulePathOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::Exclamation)
            .map(|a| UnaryModulePathOperator::Exclamation(a)),
        token(Token::Tilde).map(|a| UnaryModulePathOperator::Tilde(a)),
        token(Token::Amp).map(|a| UnaryModulePathOperator::Amp(a)),
        token(Token::TildeAmp).map(|a| UnaryModulePathOperator::TildeAmp(a)),
        token(Token::Pipe).map(|a| UnaryModulePathOperator::Pipe(a)),
        token(Token::TildePipe).map(|a| UnaryModulePathOperator::TildePipe(a)),
        token(Token::Caret).map(|a| UnaryModulePathOperator::Caret(a)),
        token(Token::TildeCaret)
            .map(|a| UnaryModulePathOperator::TildeCaret(a)),
        token(Token::CaretTilde)
            .map(|a| UnaryModulePathOperator::CaretTilde(a)),
    ))
    .parse_next(input)
}

pub fn binary_module_path_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinaryModulePathOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::EqEq).map(|a| BinaryModulePathOperator::EqEq(a)),
        token(Token::ExclEq).map(|a| BinaryModulePathOperator::ExclEq(a)),
        token(Token::AmpAmp).map(|a| BinaryModulePathOperator::AmpAmp(a)),
        token(Token::PipePipe).map(|a| BinaryModulePathOperator::PipePipe(a)),
        token(Token::Amp).map(|a| BinaryModulePathOperator::Amp(a)),
        token(Token::Pipe).map(|a| BinaryModulePathOperator::Pipe(a)),
        token(Token::Caret).map(|a| BinaryModulePathOperator::Caret(a)),
        token(Token::CaretTilde)
            .map(|a| BinaryModulePathOperator::CaretTilde(a)),
        token(Token::TildeCaret)
            .map(|a| BinaryModulePathOperator::TildeCaret(a)),
    ))
    .parse_next(input)
}
