// =======================================================================
// tokens.rs
// =======================================================================
// The tokens that a SystemVerilog source file is parsed into

use crate::callbacks::*;
use bstr::BStr;
use logos::Logos;
use std::fmt;

/// A single syntactic token for a SystemVerilog source file
///
/// Most variants don't carry data, and represent a specific
/// keyword, directive, or other literal token of the language.
/// Those that can vary in content (such as identifiers, strings,
/// etc.) contain a reference to that content in the source file.
#[derive(Logos, Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[logos(skip r"[ \t\f]+")]
#[logos(error = String)]
#[logos(utf8 = false)]
pub enum Token<'a> {
    /// A lexer error
    Error,
    // 1364-1995
    #[token(b"always")]
    Always,
    #[token(b"and")]
    And,
    #[token(b"assign")]
    Assign,
    #[token(b"begin")]
    Begin,
    #[token(b"buf")]
    Buf,
    #[token(b"bufif0")]
    Bufif0,
    #[token(b"bufif1")]
    Bufif1,
    #[token(b"case")]
    Case,
    #[token(b"casex")]
    Casex,
    #[token(b"casez")]
    Casez,
    #[token(b"cmos")]
    Cmos,
    #[token(b"deassign")]
    Deassign,
    #[token(b"default")]
    Default,
    #[token(b"defparam")]
    Defparam,
    #[token(b"disable")]
    Disable,
    #[token(b"edge")]
    Edge,
    #[token(b"else")]
    Else,
    #[token(b"end")]
    End,
    #[token(b"endcase")]
    Endcase,
    #[token(b"endfunction")]
    Endfunction,
    #[token(b"endmodule")]
    Endmodule,
    #[token(b"endprimitive")]
    Endprimitive,
    #[token(b"endspecify")]
    Endspecify,
    #[token(b"endtable")]
    Endtable,
    #[token(b"endtask")]
    Endtask,
    #[token(b"event")]
    Event,
    #[token(b"for")]
    For,
    #[token(b"force")]
    Force,
    #[token(b"forever")]
    Forever,
    #[token(b"fork")]
    Fork,
    #[token(b"function")]
    Function,
    #[token(b"highz0")]
    Highz0,
    #[token(b"highz1")]
    Highz1,
    #[token(b"if")]
    If,
    #[token(b"ifnone")]
    Ifnone,
    #[token(b"initial")]
    Initial,
    #[token(b"inout")]
    Inout,
    #[token(b"input")]
    Input,
    #[token(b"integer")]
    Integer,
    #[token(b"join")]
    Join,
    #[token(b"large")]
    Large,
    #[token(b"macromodule")]
    Macromodule,
    #[token(b"medium")]
    Medium,
    #[token(b"module")]
    Module,
    #[token(b"nand")]
    Nand,
    #[token(b"negedge")]
    Negedge,
    #[token(b"nmos")]
    Nmos,
    #[token(b"nor")]
    Nor,
    #[token(b"not")]
    Not,
    #[token(b"notif0")]
    Notif0,
    #[token(b"notif1")]
    Notif1,
    #[token(b"or")]
    Or,
    #[token(b"output")]
    Output,
    #[token(b"parameter")]
    Parameter,
    #[token(b"pmos")]
    Pmos,
    #[token(b"posedge")]
    Posedge,
    #[token(b"primitive")]
    Primitive,
    #[token(b"pull0")]
    Pull0,
    #[token(b"pull1")]
    Pull1,
    #[token(b"pulldown")]
    Pulldown,
    #[token(b"pullup")]
    Pullup,
    #[token(b"rcmos")]
    Rcmos,
    #[token(b"real")]
    Real,
    #[token(b"realtime")]
    Realtime,
    #[token(b"reg")]
    Reg,
    #[token(b"release")]
    Release,
    #[token(b"repeat")]
    Repeat,
    #[token(b"rnmos")]
    Rnmos,
    #[token(b"rpmos")]
    Rpmos,
    #[token(b"rtran")]
    Rtran,
    #[token(b"rtranif0")]
    Rtranif0,
    #[token(b"rtranif1")]
    Rtranif1,
    #[token(b"scalared")]
    Scalared,
    #[token(b"small")]
    Small,
    #[token(b"specify")]
    Specify,
    #[token(b"specparam")]
    Specparam,
    #[token(b"strong0")]
    Strong0,
    #[token(b"strong1")]
    Strong1,
    #[token(b"supply0")]
    Supply0,
    #[token(b"supply1")]
    Supply1,
    #[token(b"table")]
    Table,
    #[token(b"task")]
    Task,
    #[token(b"time")]
    Time,
    #[token(b"tran")]
    Tran,
    #[token(b"tranif0")]
    Tranif0,
    #[token(b"tranif1")]
    Tranif1,
    #[token(b"tri")]
    Tri,
    #[token(b"tri0")]
    Tri0,
    #[token(b"tri1")]
    Tri1,
    #[token(b"triand")]
    Triand,
    #[token(b"trior")]
    Trior,
    #[token(b"trireg")]
    Trireg,
    #[token(b"vectored")]
    Vectored,
    #[token(b"wait")]
    Wait,
    #[token(b"wand")]
    Wand,
    #[token(b"weak0")]
    Weak0,
    #[token(b"weak1")]
    Weak1,
    #[token(b"while")]
    While,
    #[token(b"wire")]
    Wire,
    #[token(b"wor")]
    Wor,
    #[token(b"xnor")]
    Xnor,
    #[token(b"xor")]
    Xor,
    // 1364-2001
    #[token(b"automatic")]
    Automatic,
    #[token(b"cell")]
    Cell,
    #[token(b"config")]
    Config,
    #[token(b"design")]
    Design,
    #[token(b"endconfig")]
    Endconfig,
    #[token(b"endgenerate")]
    Endgenerate,
    #[token(b"generate")]
    Generate,
    #[token(b"genvar")]
    Genvar,
    #[token(b"incdir")]
    Incdir,
    #[token(b"include")]
    Include,
    #[token(b"instance")]
    Instance,
    #[token(b"liblist")]
    Liblist,
    #[token(b"library")]
    Library,
    #[token(b"localparam")]
    Localparam,
    #[token(b"noshowcancelled")]
    Noshowcancelled,
    #[token(b"pulsestyle_ondetect")]
    PulsestyleOndetect,
    #[token(b"pulsestyle_onevent")]
    PulsestyleOnevent,
    #[token(b"showcancelled")]
    Showcancelled,
    #[token(b"signed")]
    Signed,
    #[token(b"unsigned")]
    Unsigned,
    #[token(b"use")]
    Use,
    // 1364-2005
    #[token(b"uwire")]
    Uwire,
    // 1800-2005
    #[token(b"alias")]
    Alias,
    #[token(b"always_comb")]
    AlwaysComb,
    #[token(b"always_ff")]
    AlwaysFf,
    #[token(b"always_latch")]
    AlwaysLatch,
    #[token(b"assert")]
    Assert,
    #[token(b"assume")]
    Assume,
    #[token(b"before")]
    Before,
    #[token(b"bind")]
    Bind,
    #[token(b"bins")]
    Bins,
    #[token(b"binsof")]
    Binsof,
    #[token(b"bit")]
    Bit,
    #[token(b"break")]
    Break,
    #[token(b"byte")]
    Byte,
    #[token(b"chandle")]
    Chandle,
    #[token(b"class")]
    Class,
    #[token(b"clocking")]
    Clocking,
    #[token(b"const")]
    Const,
    #[token(b"constraint")]
    Constraint,
    #[token(b"context")]
    Context,
    #[token(b"continue")]
    Continue,
    #[token(b"cover")]
    Cover,
    #[token(b"covergroup")]
    Covergroup,
    #[token(b"coverpoint")]
    Coverpoint,
    #[token(b"cross")]
    Cross,
    #[token(b"dist")]
    Dist,
    #[token(b"do")]
    Do,
    #[token(b"endclass")]
    Endclass,
    #[token(b"endclocking")]
    Endclocking,
    #[token(b"endgroup")]
    Endgroup,
    #[token(b"endinterface")]
    Endinterface,
    #[token(b"endpackage")]
    Endpackage,
    #[token(b"endprogram")]
    Endprogram,
    #[token(b"endproperty")]
    Endproperty,
    #[token(b"endsequence")]
    Endsequence,
    #[token(b"enum")]
    Enum,
    #[token(b"expect")]
    Expect,
    #[token(b"export")]
    Export,
    #[token(b"extends")]
    Extends,
    #[token(b"extern")]
    Extern,
    #[token(b"final")]
    Final,
    #[token(b"first_match")]
    FirstMatch,
    #[token(b"foreach")]
    Foreach,
    #[token(b"forkjoin")]
    Forkjoin,
    #[token(b"iff")]
    Iff,
    #[token(b"ignore_bins")]
    IgnoreBins,
    #[token(b"illegal_bins")]
    IllegalBins,
    #[token(b"import")]
    Import,
    #[token(b"inside")]
    Inside,
    #[token(b"int")]
    Int,
    #[token(b"interface")]
    Interface,
    #[token(b"intersect")]
    Intersect,
    #[token(b"join_any")]
    JoinAny,
    #[token(b"join_none")]
    JoinNone,
    #[token(b"local")]
    Local,
    #[token(b"logic")]
    Logic,
    #[token(b"longint")]
    Longint,
    #[token(b"matches")]
    Matches,
    #[token(b"modport")]
    Modport,
    #[token(b"new")]
    New,
    #[token(b"null")]
    Null,
    #[token(b"package")]
    Package,
    #[token(b"packed")]
    Packed,
    #[token(b"priority")]
    Priority,
    #[token(b"program")]
    Program,
    #[token(b"property")]
    Property,
    #[token(b"protected")]
    Protected,
    #[token(b"pure")]
    Pure,
    #[token(b"rand")]
    Rand,
    #[token(b"randc")]
    Randc,
    #[token(b"randcase")]
    Randcase,
    #[token(b"randsequence")]
    Randsequence,
    #[token(b"ref")]
    Ref,
    #[token(b"return")]
    Return,
    #[token(b"sequence")]
    Sequence,
    #[token(b"shortint")]
    Shortint,
    #[token(b"shortreal")]
    Shortreal,
    #[token(b"solve")]
    Solve,
    #[token(b"static")]
    Static,
    #[token(b"string")]
    String,
    #[token(b"struct")]
    Struct,
    #[token(b"super")]
    Super,
    #[token(b"tagged")]
    Tagged,
    #[token(b"this")]
    This,
    #[token(b"throughout")]
    Throughout,
    #[token(b"timeprecision")]
    Timeprecision,
    #[token(b"timeunit")]
    Timeunit,
    #[token(b"type")]
    Type,
    #[token(b"typedef")]
    Typedef,
    #[token(b"union")]
    Union,
    #[token(b"unique")]
    Unique,
    #[token(b"var")]
    Var,
    #[token(b"virtual")]
    Virtual,
    #[token(b"void")]
    Void,
    #[token(b"wait_order")]
    WaitOrder,
    #[token(b"wildcard")]
    Wildcard,
    #[token(b"with")]
    With,
    #[token(b"within")]
    Within,
    // 1800-2009
    #[token(b"accept_on")]
    AcceptOn,
    #[token(b"checker")]
    Checker,
    #[token(b"endchecker")]
    Endchecker,
    #[token(b"eventually")]
    Eventually,
    #[token(b"global")]
    Global,
    #[token(b"implies")]
    Implies,
    #[token(b"let")]
    Let,
    #[token(b"nexttime")]
    Nexttime,
    #[token(b"reject_on")]
    RejectOn,
    #[token(b"restrict")]
    Restrict,
    #[token(b"s_always")]
    SAlways,
    #[token(b"s_eventually")]
    SEventually,
    #[token(b"s_nexttime")]
    SNexttime,
    #[token(b"s_until")]
    SUntil,
    #[token(b"s_until_with")]
    SUntilWith,
    #[token(b"strong")]
    Strong,
    #[token(b"sync_accept_on")]
    SyncAcceptOn,
    #[token(b"sync_reject_on")]
    SyncRejectOn,
    #[token(b"unique0")]
    Unique0,
    #[token(b"until")]
    Until,
    #[token(b"until_with")]
    UntilWith,
    #[token(b"untyped")]
    Untyped,
    #[token(b"weak")]
    Weak,
    // 1800-2012
    #[token(b"implements")]
    Implements,
    #[token(b"interconnect")]
    Interconnect,
    #[token(b"nettype")]
    Nettype,
    #[token(b"soft")]
    Soft,
    // Directives
    #[token(b"`__FILE__")]
    DirUnderscoreFile,
    #[token(b"`__LINE__")]
    DirUnderscoreLine,
    #[token(b"`begin_keywords")]
    DirBeginKeywords,
    #[token(b"`celldefine")]
    DirCelldefine,
    #[token(b"`default_nettype")]
    DirDefaultNettype,
    #[token(b"`define")]
    DirDefine,
    #[token(b"`else")]
    DirElse,
    #[token(b"`elsif")]
    DirElsif,
    #[token(b"`end_keywords")]
    DirEndKeywords,
    #[token(b"`endcelldefine")]
    DirEndcelldefine,
    #[token(b"`endif")]
    DirEndif,
    #[token(b"`ifdef")]
    DirIfdef,
    #[token(b"`ifndef")]
    DirIfndef,
    #[token(b"`include")]
    DirInclude,
    #[token(b"`line")]
    DirLine,
    #[token(b"`nounconnected_drive")]
    DirNounconnectedDrive,
    #[token(b"`pragma")]
    DirPragma,
    #[token(b"`resetall")]
    DirResetall,
    #[token(b"`timescale")]
    DirTimescale,
    #[token(b"`unconnected_drive")]
    DirUnconnectedDrive,
    #[token(b"`undef")]
    DirUndef,
    #[token(b"`undefineall")]
    DirUndefineall,
    // Operators
    #[token(b"+")]
    Plus,
    #[token(b"-")]
    Minus,
    #[token(b"!")]
    Exclamation,
    #[token(b"?")]
    Quest,
    #[token(b"~")]
    Tilde,
    #[token(b"&")]
    Amp,
    #[token(b"~&")]
    TildeAmp,
    #[token(b"|")]
    Pipe,
    #[token(b"~|")]
    TildePipe,
    #[token(b"^")]
    Caret,
    #[token(b"~^")]
    TildeCaret,
    #[token(b"^~")]
    CaretTilde,
    #[token(b"*")]
    Star,
    #[token(b"/")]
    Slash,
    #[token(b"%")]
    Percent,
    #[token(b"==")]
    EqEq,
    #[token(b"!=")]
    ExclEq,
    #[token(b"+=")]
    PlusEq,
    #[token(b"-=")]
    MinusEq,
    #[token(b"*=")]
    StarEq,
    #[token(b"/=")]
    SlashEq,
    #[token(b"%=")]
    PercentEq,
    #[token(b"&=")]
    AmpEq,
    #[token(b"|=")]
    PipeEq,
    #[token(b"^=")]
    CaretEq,
    #[token(b"===")]
    EqEqEq,
    #[token(b"!==")]
    ExclEqEq,
    #[token(b"==?")]
    EqEqQuest,
    #[token(b"!=?")]
    ExclEqQuest,
    #[token(b"&&")]
    AmpAmp,
    #[token(b"&&&")]
    AmpAmpAmp,
    #[token(b"||")]
    PipePipe,
    #[token(b"**")]
    StarStar,
    #[token(b"<")]
    Lt,
    #[token(b"<=")]
    LtEq,
    #[token(b">")]
    Gt,
    #[token(b">=")]
    GtEq,
    #[token(b">>")]
    GtGt,
    #[token(b"<<")]
    LtLt,
    #[token(b">>=")]
    GtGtEq,
    #[token(b"<<=")]
    LtLtEq,
    #[token(b">>>")]
    GtGtGt,
    #[token(b"<<<")]
    LtLtLt,
    #[token(b">>>=")]
    GtGtGtEq,
    #[token(b"<<<=")]
    LtLtLtEq,
    #[token(b"->")]
    MinusGt,
    #[token(b"->>")]
    MinusGtGt,
    #[token(b"<->")]
    LtMinusGt,
    #[token(b"++")]
    PlusPlus,
    #[token(b"--")]
    MinusMinus,
    #[token(b"+:")]
    PlusColon,
    #[token(b"-:")]
    MinusColon,
    #[token(b"+/-")]
    PlusSlashMinus,
    #[token(b"+%-")]
    PlusPercentMinus,
    // Symbols
    #[token(b"(")]
    Paren,
    #[token(b")")]
    EParen,
    #[token(b"[")]
    Bracket,
    #[token(b"]")]
    EBracket,
    #[token(b"{")]
    Brace,
    #[token(b"}")]
    EBrace,
    #[token(b":")]
    Colon,
    #[token(b";")]
    SColon,
    #[token(b"'")]
    Apost,
    #[token(b",")]
    Comma,
    #[token(b".")]
    Period,
    #[token(b"#")]
    Pound,
    #[token(b"$")]
    Dollar,
    #[token(b"@")]
    At,
    #[token(b"@@")]
    AtAt,
    #[token(b"=")]
    Eq,
    #[token(b"::")]
    ColonColon,
    #[token(b":=")]
    ColonEq,
    #[token(b":/")]
    ColonSlash,
    #[token(b"##")]
    PoundPound,
    #[token(b"#-#")]
    PoundMinusPound,
    #[token(b"#=#")]
    PoundEqPound,
    #[token(b"=>")]
    EqGt,
    #[token(b"*>")]
    StarGt,
    #[token(b"|->")]
    PipeMinusGt,
    #[token(b"|=>")]
    PipeEqGt,
    #[token(r"\")]
    Bslash,
    // Other Language Grammar
    #[token(b"PATHPULSE$")]
    PathpulseDollar,
    #[token(b"1step")]
    OneStep,
    #[token(b"$setup")]
    DollarSetup,
    #[token(b"$hold")]
    DollarHold,
    #[token(b"$setuphold")]
    DollarSetuphold,
    #[token(b"$recovery")]
    DollarRecovery,
    #[token(b"$removal")]
    DollarRemoval,
    #[token(b"$recrem")]
    DollarRecrem,
    #[token(b"$skew")]
    DollarSkew,
    #[token(b"$timeskew")]
    DollarTimeskew,
    #[token(b"$fullskew")]
    DollarFullskew,
    #[token(b"$period")]
    DollarPeriod,
    #[token(b"$width")]
    DollarWidth,
    #[token(b"$nochange")]
    DollarNochange,
    #[token(b"$root")]
    DollarRoot,
    #[token(b"$unit")]
    DollarUnit,
    #[token(b"$fatal")]
    DollarFatal,
    #[token(b"$error")]
    DollarError,
    #[token(b"$warning")]
    DollarWarning,
    #[token(b"$info")]
    DollarInfo,
    // Comments
    #[regex(br"//[^\r\n]*", oneline_comment, allow_greedy = true)]
    OnelineComment(&'a BStr),
    #[token(r"/*", block_comment)]
    BlockComment(&'a BStr),
    // Numbers
    #[regex(br"[0-9][0-9_]*", |lex| Into::<&BStr>::into(lex.slice()))]
    UnsignedNumber(&'a BStr),
    #[regex(br"[0-9][0-9_]*\.[0-9][0-9_]*", |lex| Into::<&BStr>::into(lex.slice()))]
    FixedPointNumber(&'a BStr),
    #[regex(br"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(b|B)[^\S\r\n]*[0-1xXzZ\?][0-1xXzZ\?_]*", |lex| Into::<&BStr>::into(lex.slice()))]
    BinaryNumber(&'a BStr),
    #[regex(br"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(o|O)[^\S\r\n]*[0-7xXzZ\?][0-7xXzZ\?_]*", |lex| Into::<&BStr>::into(lex.slice()))]
    OctalNumber(&'a BStr),
    #[regex(br"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(d|D)[^\S\r\n]*[0-9][0-9_]*", |lex| Into::<&BStr>::into(lex.slice()))]
    #[regex(br"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(d|D)[^\S\r\n]*(x|X|z|Z|\?)_*", |lex| Into::<&BStr>::into(lex.slice()))]
    DecimalNumber(&'a BStr),
    #[regex(br"([0-9][0-9_]*)?[^\S\r\n]*'[s|S]?(h|H)[^\S\r\n]*[0-9a-fA-FxXzZ\?][0-9a-fA-FxXzZ\?_]*", |lex| Into::<&BStr>::into(lex.slice()))]
    HexNumber(&'a BStr),
    #[regex(br"[0-9][0-9_]*(\.[0-9][0-9_]*)?(e|E)(\+|-)?[0-9][0-9_]*", |lex| Into::<&BStr>::into(lex.slice()))]
    ScientificNumber(&'a BStr),
    #[regex(br"('0|'1|'x|'X|'z|'Z|'\?)", |lex| Into::<&BStr>::into(lex.slice()))]
    UnbasedUnsizedLiteral(&'a BStr),
    // Literals
    #[regex(br"\$[a-zA-Z0-9_\$]+", |lex| Into::<&BStr>::into(lex.slice()))]
    SystemTfIdentifier(&'a BStr),
    #[regex(br"[a-zA-Z_][a-zA-Z0-9_\$]*", |lex| Into::<&BStr>::into(lex.slice()))]
    SimpleIdentifier(&'a BStr),
    #[regex(br"\\[!-~]+(\s|$)", |lex| Into::<&BStr>::into(lex.slice()))]
    EscapedIdentifier(&'a BStr),
    #[regex(br"`[a-zA-Z_][a-zA-Z0-9_\$]*", text_macro)]
    #[regex(br"`\\[!-~]+", text_macro)]
    TextMacro(&'a BStr),
    #[token(b"``")]
    MacroConcatenate,
    #[token(r#"""#, string_literal)]
    StringLiteral(&'a BStr),
    #[token(r#"`""#, preprocessor_string_literal)]
    PreprocessorStringLiteral(&'a BStr),
    #[token(r#"""""#, multiline_string_literal)]
    TripleQuoteStringLiteral(&'a BStr),
    #[token(r#"`""""#, preprocessor_multiline_string_literal)]
    PreprocessorTripleQuoteStringLiteral(&'a BStr),
    #[token(b"\n")]
    #[token(b"\r")]
    #[token(b"\r\n")]
    #[token("\u{0085}")]
    #[token("\u{2028}")]
    #[token("\u{2029}")]
    Newline,
    // Used for intermediate concatenation results
    InvalidConcatenation(&'a BStr),
}

impl<'a> Token<'a> {
    /// Whether the token represents a compiler directive
    pub fn is_directive(&self) -> bool {
        match self {
            Token::DirUnderscoreFile
            | Token::DirUnderscoreLine
            | Token::DirBeginKeywords
            | Token::DirCelldefine
            | Token::DirDefaultNettype
            | Token::DirDefine
            | Token::DirElse
            | Token::DirElsif
            | Token::DirEndKeywords
            | Token::DirEndcelldefine
            | Token::DirEndif
            | Token::DirIfdef
            | Token::DirIfndef
            | Token::DirInclude
            | Token::DirLine
            | Token::DirNounconnectedDrive
            | Token::DirPragma
            | Token::DirResetall
            | Token::DirTimescale
            | Token::DirUnconnectedDrive
            | Token::DirUndef
            | Token::DirUndefineall
            | Token::TextMacro(_) => true,
            _ => false,
        }
    }
    /// A string representation of the token (usually the literal matching text)
    pub fn as_str(&self) -> &'static str {
        match self {
            Token::Error => "a lexer error",
            Token::Always => "always",
            Token::And => "and",
            Token::Assign => "assign",
            Token::Begin => "begin",
            Token::Buf => "buf",
            Token::Bufif0 => "bufif0",
            Token::Bufif1 => "bufif1",
            Token::Case => "case",
            Token::Casex => "casex",
            Token::Casez => "casez",
            Token::Cmos => "cmos",
            Token::Deassign => "deassign",
            Token::Default => "default",
            Token::Defparam => "defparam",
            Token::Disable => "disable",
            Token::Edge => "edge",
            Token::Else => "else",
            Token::End => "end",
            Token::Endcase => "endcase",
            Token::Endfunction => "endfunction",
            Token::Endmodule => "endmodule",
            Token::Endprimitive => "endprimitive",
            Token::Endspecify => "endspecify",
            Token::Endtable => "endtable",
            Token::Endtask => "endtask",
            Token::Event => "event",
            Token::For => "for",
            Token::Force => "force",
            Token::Forever => "forever",
            Token::Fork => "fork",
            Token::Function => "function",
            Token::Highz0 => "highz0",
            Token::Highz1 => "highz1",
            Token::If => "if",
            Token::Ifnone => "ifnone",
            Token::Initial => "initial",
            Token::Inout => "inout",
            Token::Input => "input",
            Token::Integer => "integer",
            Token::Join => "join",
            Token::Large => "large",
            Token::Macromodule => "macromodule",
            Token::Medium => "medium",
            Token::Module => "module",
            Token::Nand => "nand",
            Token::Negedge => "negedge",
            Token::Nmos => "nmos",
            Token::Nor => "nor",
            Token::Not => "not",
            Token::Notif0 => "notif0",
            Token::Notif1 => "notif1",
            Token::Or => "or",
            Token::Output => "output",
            Token::Parameter => "parameter",
            Token::Pmos => "pmos",
            Token::Posedge => "posedge",
            Token::Primitive => "primitive",
            Token::Pull0 => "pull0",
            Token::Pull1 => "pull1",
            Token::Pulldown => "pulldown",
            Token::Pullup => "pullup",
            Token::Rcmos => "rcmos",
            Token::Real => "real",
            Token::Realtime => "realtime",
            Token::Reg => "reg",
            Token::Release => "release",
            Token::Repeat => "repeat",
            Token::Rnmos => "rnmos",
            Token::Rpmos => "rpmos",
            Token::Rtran => "rtran",
            Token::Rtranif0 => "rtranif0",
            Token::Rtranif1 => "rtranif1",
            Token::Scalared => "scalared",
            Token::Small => "small",
            Token::Specify => "specify",
            Token::Specparam => "specparam",
            Token::Strong0 => "strong0",
            Token::Strong1 => "strong1",
            Token::Supply0 => "supply0",
            Token::Supply1 => "supply1",
            Token::Table => "table",
            Token::Task => "task",
            Token::Time => "time",
            Token::Tran => "tran",
            Token::Tranif0 => "tranif0",
            Token::Tranif1 => "tranif1",
            Token::Tri => "tri",
            Token::Tri0 => "tri0",
            Token::Tri1 => "tri1",
            Token::Triand => "triand",
            Token::Trior => "trior",
            Token::Trireg => "trireg",
            Token::Vectored => "vectored",
            Token::Wait => "wait",
            Token::Wand => "wand",
            Token::Weak0 => "weak0",
            Token::Weak1 => "weak1",
            Token::While => "while",
            Token::Wire => "wire",
            Token::Wor => "wor",
            Token::Xnor => "xnor",
            Token::Xor => "xor",
            Token::Automatic => "automatic",
            Token::Cell => "cell",
            Token::Config => "config",
            Token::Design => "design",
            Token::Endconfig => "endconfig",
            Token::Endgenerate => "endgenerate",
            Token::Generate => "generate",
            Token::Genvar => "genvar",
            Token::Incdir => "incdir",
            Token::Include => "include",
            Token::Instance => "instance",
            Token::Liblist => "liblist",
            Token::Library => "library",
            Token::Localparam => "localparam",
            Token::Noshowcancelled => "noshowcancelled",
            Token::PulsestyleOndetect => "pulsestyle_ondetect",
            Token::PulsestyleOnevent => "pulsestyle_onevent",
            Token::Showcancelled => "showcancelled",
            Token::Signed => "signed",
            Token::Unsigned => "unsigned",
            Token::Use => "use",
            Token::Uwire => "uwire",
            Token::Alias => "alias",
            Token::AlwaysComb => "always_comb",
            Token::AlwaysFf => "always_ff",
            Token::AlwaysLatch => "always_latch",
            Token::Assert => "assert",
            Token::Assume => "assume",
            Token::Before => "before",
            Token::Bind => "bind",
            Token::Bins => "bins",
            Token::Binsof => "binsof",
            Token::Bit => "bit",
            Token::Break => "break",
            Token::Byte => "byte",
            Token::Chandle => "chandle",
            Token::Class => "class",
            Token::Clocking => "clocking",
            Token::Const => "const",
            Token::Constraint => "constraint",
            Token::Context => "context",
            Token::Continue => "continue",
            Token::Cover => "cover",
            Token::Covergroup => "covergroup",
            Token::Coverpoint => "coverpoint",
            Token::Cross => "cross",
            Token::Dist => "dist",
            Token::Do => "do",
            Token::Endclass => "endclass",
            Token::Endclocking => "endclocking",
            Token::Endgroup => "endgroup",
            Token::Endinterface => "endinterface",
            Token::Endpackage => "endpackage",
            Token::Endprogram => "endprogram",
            Token::Endproperty => "endproperty",
            Token::Endsequence => "endsequence",
            Token::Enum => "enum",
            Token::Expect => "expect",
            Token::Export => "export",
            Token::Extends => "extends",
            Token::Extern => "extern",
            Token::Final => "final",
            Token::FirstMatch => "first_match",
            Token::Foreach => "foreach",
            Token::Forkjoin => "forkjoin",
            Token::Iff => "iff",
            Token::IgnoreBins => "ignore_bins",
            Token::IllegalBins => "illegal_bins",
            Token::Import => "import",
            Token::Inside => "inside",
            Token::Int => "int",
            Token::Interface => "interface",
            Token::Intersect => "intersect",
            Token::JoinAny => "join_any",
            Token::JoinNone => "join_none",
            Token::Local => "local",
            Token::Logic => "logic",
            Token::Longint => "longint",
            Token::Matches => "matches",
            Token::Modport => "modport",
            Token::New => "new",
            Token::Null => "null",
            Token::Package => "package",
            Token::Packed => "packed",
            Token::Priority => "priority",
            Token::Program => "program",
            Token::Property => "property",
            Token::Protected => "protected",
            Token::Pure => "pure",
            Token::Rand => "rand",
            Token::Randc => "randc",
            Token::Randcase => "randcase",
            Token::Randsequence => "randsequence",
            Token::Ref => "ref",
            Token::Return => "return",
            Token::Sequence => "sequence",
            Token::Shortint => "shortint",
            Token::Shortreal => "shortreal",
            Token::Solve => "solve",
            Token::Static => "static",
            Token::String => "string",
            Token::Struct => "struct",
            Token::Super => "super",
            Token::Tagged => "tagged",
            Token::This => "this",
            Token::Throughout => "throughout",
            Token::Timeprecision => "timeprecision",
            Token::Timeunit => "timeunit",
            Token::Type => "type",
            Token::Typedef => "typedef",
            Token::Union => "union",
            Token::Unique => "unique",
            Token::Var => "var",
            Token::Virtual => "virtual",
            Token::Void => "void",
            Token::WaitOrder => "wait_order",
            Token::Wildcard => "wildcard",
            Token::With => "with",
            Token::Within => "within",
            Token::AcceptOn => "accept_on",
            Token::Checker => "checker",
            Token::Endchecker => "endchecker",
            Token::Eventually => "eventually",
            Token::Global => "global",
            Token::Implies => "implies",
            Token::Let => "let",
            Token::Nexttime => "nexttime",
            Token::RejectOn => "reject_on",
            Token::Restrict => "restrict",
            Token::SAlways => "s_always",
            Token::SEventually => "s_eventually",
            Token::SNexttime => "s_nexttime",
            Token::SUntil => "s_until",
            Token::SUntilWith => "s_until_with",
            Token::Strong => "strong",
            Token::SyncAcceptOn => "sync_accept_on",
            Token::SyncRejectOn => "sync_reject_on",
            Token::Unique0 => "unique0",
            Token::Until => "until",
            Token::UntilWith => "until_with",
            Token::Untyped => "untyped",
            Token::Weak => "weak",
            Token::Implements => "implements",
            Token::Interconnect => "interconnect",
            Token::Nettype => "nettype",
            Token::Soft => "soft",
            Token::DirUnderscoreFile => "`__FILE__",
            Token::DirUnderscoreLine => "`__LINE__",
            Token::DirBeginKeywords => "`begin_keywords",
            Token::DirCelldefine => "`celldefine",
            Token::DirDefaultNettype => "`default_nettype",
            Token::DirDefine => "`define",
            Token::DirElse => "`else",
            Token::DirElsif => "`elsif",
            Token::DirEndKeywords => "`end_keywords",
            Token::DirEndcelldefine => "`endcelldefine",
            Token::DirEndif => "`endif",
            Token::DirIfdef => "`ifdef",
            Token::DirIfndef => "`ifndef",
            Token::DirInclude => "`include",
            Token::DirLine => "`line",
            Token::DirNounconnectedDrive => "`nounconnected_drive",
            Token::DirPragma => "`pragma",
            Token::DirResetall => "`resetall",
            Token::DirTimescale => "`timescale",
            Token::DirUnconnectedDrive => "`unconnected_drive",
            Token::DirUndef => "`undef",
            Token::DirUndefineall => "`undefineall",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Exclamation => "!",
            Token::Quest => "?",
            Token::Tilde => "~",
            Token::Amp => "&",
            Token::TildeAmp => "~&",
            Token::Pipe => "|",
            Token::TildePipe => "~|",
            Token::Caret => "^",
            Token::TildeCaret => "~^",
            Token::CaretTilde => "^~",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Percent => "%",
            Token::EqEq => "==",
            Token::ExclEq => "!=",
            Token::PlusEq => "+=",
            Token::MinusEq => "-=",
            Token::StarEq => "*=",
            Token::SlashEq => "/=",
            Token::PercentEq => "%=",
            Token::AmpEq => "&=",
            Token::PipeEq => "|=",
            Token::CaretEq => "^=",
            Token::EqEqEq => "===",
            Token::ExclEqEq => "!==",
            Token::EqEqQuest => "==?",
            Token::ExclEqQuest => "!=?",
            Token::AmpAmp => "&&",
            Token::AmpAmpAmp => "&&&",
            Token::PipePipe => "||",
            Token::StarStar => "**",
            Token::Lt => "<",
            Token::LtEq => "<=",
            Token::Gt => ">",
            Token::GtEq => ">=",
            Token::GtGt => ">>",
            Token::LtLt => "<<",
            Token::GtGtEq => ">>=",
            Token::LtLtEq => "<<=",
            Token::GtGtGt => ">>>",
            Token::LtLtLt => "<<<",
            Token::GtGtGtEq => ">>>=",
            Token::LtLtLtEq => "<<<=",
            Token::MinusGt => "->",
            Token::MinusGtGt => "->>",
            Token::LtMinusGt => "<->",
            Token::PlusPlus => "++",
            Token::MinusMinus => "--",
            Token::PlusColon => "+:",
            Token::MinusColon => "-:",
            Token::PlusSlashMinus => "+/-",
            Token::PlusPercentMinus => "+%-",
            Token::Paren => "(",
            Token::EParen => ")",
            Token::Bracket => "[",
            Token::EBracket => "]",
            Token::Brace => "{",
            Token::EBrace => "}",
            Token::Colon => ":",
            Token::SColon => ";",
            Token::Apost => "'",
            Token::Comma => "a comma",
            Token::Period => ".",
            Token::Pound => "#",
            Token::Dollar => "$",
            Token::At => "@",
            Token::AtAt => "@@",
            Token::Eq => "=",
            Token::ColonColon => "::",
            Token::ColonEq => ":=",
            Token::ColonSlash => ":/",
            Token::PoundPound => "##",
            Token::PoundMinusPound => "#-#",
            Token::PoundEqPound => "#=#",
            Token::EqGt => "=>",
            Token::StarGt => "*>",
            Token::PipeMinusGt => "|->",
            Token::PipeEqGt => "|=>",
            Token::Bslash => r"\",
            Token::PathpulseDollar => "PATHPULSE$",
            Token::OneStep => "1step",
            Token::DollarSetup => "$setup",
            Token::DollarHold => "$hold",
            Token::DollarSetuphold => "$setuphold",
            Token::DollarRecovery => "$recovery",
            Token::DollarRemoval => "$removal",
            Token::DollarRecrem => "$recrem",
            Token::DollarSkew => "$skew",
            Token::DollarTimeskew => "$timeskew",
            Token::DollarFullskew => "$fullskew",
            Token::DollarPeriod => "$period",
            Token::DollarWidth => "$width",
            Token::DollarNochange => "$nochange",
            Token::DollarRoot => "$root",
            Token::DollarUnit => "$unit",
            Token::DollarFatal => "$fatal",
            Token::DollarError => "$error",
            Token::DollarWarning => "$warning",
            Token::DollarInfo => "$info",
            Token::OnelineComment(_text) => "<oneline comment>",
            Token::BlockComment(_text) => "<block comment>",
            Token::UnsignedNumber(_text) => "<unsigned number>",
            Token::FixedPointNumber(_text) => "<real number>",
            Token::BinaryNumber(_text) => "<binary number>",
            Token::OctalNumber(_text) => "<octal number>",
            Token::DecimalNumber(_text) => "<decimal number>",
            Token::HexNumber(_text) => "<hex number>",
            Token::ScientificNumber(_text) => "<scientific number>",
            Token::UnbasedUnsizedLiteral(_text) => "<unsized literal>",
            Token::SystemTfIdentifier(_text) => "<system tf identifier>",
            Token::SimpleIdentifier(_text) => "<simple identifier>",
            Token::EscapedIdentifier(_text) => "<escaped identifier>",
            Token::TextMacro(_text) => "<text macro>",
            Token::MacroConcatenate => "``",
            Token::StringLiteral(_text) => "<string>",
            Token::PreprocessorStringLiteral(_text) => "<preprocessor string>",
            Token::TripleQuoteStringLiteral(_text) => "<triple-quote string>",
            Token::PreprocessorTripleQuoteStringLiteral(_text) => {
                "<preprocessor triple-quote string>"
            }
            Token::Newline => "newline",
            Token::InvalidConcatenation(_text) => {
                "<invalid token concatenation>"
            }
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let temp_str: String;
        let str_repr = match self {
            Token::OnelineComment(text) => {
                temp_str = format!("comment '{}'", text);
                temp_str.as_str()
            }
            Token::BlockComment(text) => {
                temp_str = format!("block comment '{}'", text);
                temp_str.as_str()
            }
            Token::UnsignedNumber(text) => {
                temp_str = format!("number '{}' ", text);
                temp_str.as_str()
            }
            Token::FixedPointNumber(text) => {
                temp_str = format!("real number '{}' ", text);
                temp_str.as_str()
            }
            Token::BinaryNumber(text) => {
                temp_str = format!("binary number '{}' ", text);
                temp_str.as_str()
            }
            Token::OctalNumber(text) => {
                temp_str = format!("octal number '{}' ", text);
                temp_str.as_str()
            }
            Token::DecimalNumber(text) => {
                temp_str = format!("decimal number '{}' ", text);
                temp_str.as_str()
            }
            Token::HexNumber(text) => {
                temp_str = format!("hexadecimal number '{}' ", text);
                temp_str.as_str()
            }
            Token::ScientificNumber(text) => {
                temp_str = format!("real number '{}' ", text);
                temp_str.as_str()
            }
            Token::UnbasedUnsizedLiteral(text) => {
                temp_str = format!("unsized literal '{}' ", text);
                temp_str.as_str()
            }
            Token::SystemTfIdentifier(text) => {
                temp_str = format!("{}", text);
                temp_str.as_str()
            }
            Token::SimpleIdentifier(text) => {
                temp_str = format!("identifier '{}'", text);
                temp_str.as_str()
            }
            Token::EscapedIdentifier(text) => {
                temp_str = format!("escaped identifier '{}'", text);
                temp_str.as_str()
            }
            Token::TextMacro(text) => {
                temp_str = format!("text macro '{}'", text);
                temp_str.as_str()
            }
            Token::StringLiteral(text) => {
                temp_str = format!("string \"{}\"", text);
                temp_str.as_str()
            }
            Token::PreprocessorStringLiteral(text) => {
                temp_str = format!("preprocessor string `\"{}`\"", text);
                temp_str.as_str()
            }
            Token::TripleQuoteStringLiteral(text) => {
                temp_str = format!("string \"\"\"{}\"\"\"", text);
                temp_str.as_str()
            }
            Token::PreprocessorTripleQuoteStringLiteral(text) => {
                temp_str =
                    format!("preprocessor string `\"\"\"{}`\"\"\"", text);
                temp_str.as_str()
            }
            Token::InvalidConcatenation(text) => {
                temp_str = format!("invalid token concatenation '{}'", text);
                temp_str.as_str()
            }
            _ => self.as_str(),
        };
        write!(f, "{}", str_repr)
    }
}
