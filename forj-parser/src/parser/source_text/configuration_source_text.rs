// =======================================================================
// configuration_source_text.rs
// =======================================================================
// Parsing for 1800-2023 A.1.5

use crate::*;
use forj_syntax::*;
use winnow::ModalResult;
use winnow::combinator::alt;

pub fn config_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConfigDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Config),
        config_identifier_parser,
        token(Token::SColon),
        repeat_note((local_parameter_declaration_parser, token(Token::SColon))),
        design_statement_parser,
        repeat_note(config_rule_statement_parser),
        token(Token::Endconfig),
        opt_note((token(Token::Colon), config_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            ConfigDeclaration(a, b, c, d, e, f, g, h)
        })
        .parse_next(input)
}

pub fn design_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DesignStatement<'s>, VerboseError<'s>> {
    (
        token(Token::Design),
        repeat_note((
            opt_note((library_identifier_parser, token(Token::Period))),
            cell_identifier_parser,
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| DesignStatement(a, b, c))
        .parse_next(input)
}

pub enum LiblistOrUseClause<'a> {
    Liblist(LiblistClause<'a>),
    Use(UseClause<'a>),
}

pub enum InstOrCellClause<'a> {
    Inst(InstClause<'a>),
    Cell(CellClause<'a>),
}

pub fn config_rule_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConfigRuleStatement<'s>, VerboseError<'s>> {
    let _default_parser = (
        default_clause_parser,
        liblist_clause_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| {
            ConfigRuleStatement::DefaultLiblist(Box::new((a, b, c)))
        });
    let _first_clause_parser = alt((
        inst_clause_parser.map(|a| InstOrCellClause::Inst(a)),
        cell_clause_parser.map(|a| InstOrCellClause::Cell(a)),
    ));
    let _second_clause_parser = alt((
        liblist_clause_parser.map(|a| LiblistOrUseClause::Liblist(a)),
        use_clause_parser.map(|a| LiblistOrUseClause::Use(a)),
    ));
    let _remaining_parser = (
        _first_clause_parser,
        _second_clause_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| match (a, b) {
            (
                InstOrCellClause::Inst(inst_clause),
                LiblistOrUseClause::Liblist(liblist_clause),
            ) => ConfigRuleStatement::InstLiblist(Box::new((
                inst_clause,
                liblist_clause,
                c,
            ))),
            (
                InstOrCellClause::Inst(inst_clause),
                LiblistOrUseClause::Use(use_clause),
            ) => ConfigRuleStatement::InstUse(Box::new((
                inst_clause,
                use_clause,
                c,
            ))),
            (
                InstOrCellClause::Cell(cell_clause),
                LiblistOrUseClause::Liblist(liblist_clause),
            ) => ConfigRuleStatement::CellLiblist(Box::new((
                cell_clause,
                liblist_clause,
                c,
            ))),
            (
                InstOrCellClause::Cell(cell_clause),
                LiblistOrUseClause::Use(use_clause),
            ) => ConfigRuleStatement::CellUse(Box::new((
                cell_clause,
                use_clause,
                c,
            ))),
        });
    alt((_default_parser, _remaining_parser)).parse_next(input)
}

pub fn default_clause_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DefaultClause<'s>, VerboseError<'s>> {
    token(Token::Default)
        .map(|a| DefaultClause(a))
        .parse_next(input)
}

pub fn inst_clause_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InstClause<'s>, VerboseError<'s>> {
    (token(Token::Instance), inst_name_parser)
        .map(|(a, b)| InstClause(a, b))
        .parse_next(input)
}

pub fn inst_name_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InstName<'s>, VerboseError<'s>> {
    (
        topmodule_identifier_parser,
        repeat_note((token(Token::Period), instance_identifier_parser)),
    )
        .map(|(a, b)| InstName(a, b))
        .parse_next(input)
}

pub fn cell_clause_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CellClause<'s>, VerboseError<'s>> {
    (
        token(Token::Cell),
        opt_note((library_identifier_parser, token(Token::Period))),
        cell_identifier_parser,
    )
        .map(|(a, b, c)| CellClause(a, b, c))
        .parse_next(input)
}

pub fn liblist_clause_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LiblistClause<'s>, VerboseError<'s>> {
    (
        token(Token::Liblist),
        repeat_note(library_identifier_parser),
    )
        .map(|(a, b)| LiblistClause(a, b))
        .parse_next(input)
}

pub fn use_clause_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UseClause<'s>, VerboseError<'s>> {
    (
        token(Token::Use),
        opt_note((
            opt_note((library_identifier_parser, token(Token::Period))),
            cell_identifier_parser,
        )),
        opt_note((
            named_parameter_assignment_parser,
            repeat_note((
                token(Token::Comma),
                named_parameter_assignment_parser,
            )),
        )),
        opt_note((token(Token::Colon), token(Token::Config))),
    )
        .verify_map(|(a, b, c, d)| match (b, c) {
            (Some((library_identifier, cell_identifier)), None) => {
                Some(UseClause::Cell(Box::new((
                    a,
                    library_identifier,
                    cell_identifier,
                    d,
                ))))
            }
            (None, Some((first_param, repeat_param))) => {
                Some(UseClause::Parameter(Box::new((
                    a,
                    first_param,
                    repeat_param,
                    d,
                ))))
            }
            (
                Some((library_identifier, cell_identifier)),
                Some((first_param, repeat_param)),
            ) => Some(UseClause::CellParameter(Box::new((
                a,
                library_identifier,
                cell_identifier,
                first_param,
                repeat_param,
                d,
            )))),
            (None, None) => None,
        })
        .parse_next(input)
}
