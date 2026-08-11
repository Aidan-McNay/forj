// =======================================================================
// tokens.rs
// =======================================================================
// The tokens that a SystemVerilog source file is parsed into

use crate::callbacks::*;
use logos::Logos;
use std::fmt;

/// A single syntactic token for a SystemVerilog source file
///
/// Most variants don't carry data, and represent a specific
/// keyword, directive, or other literal token of the language.
/// Those that can vary in content (such as identifiers, strings,
/// etc.) contain a reference to that content in the source file.
#[derive(Logos, Debug, Clone, PartialEq, Eq, Copy)]
#[logos(skip r"[ \t\f]+")]
#[logos(error = String)]
pub enum Token<'a> {
    /// A lexer error
    Error,
    // 1364-1995
    #[token("always")]
    Always,
    #[token("and")]
    And,
    #[token("assign")]
    Assign,
    #[token("begin")]
    Begin,
    #[token("buf")]
    Buf,
    #[token("bufif0")]
    Bufif0,
    #[token("bufif1")]
    Bufif1,
    #[token("case")]
    Case,
    #[token("casex")]
    Casex,
    #[token("casez")]
    Casez,
    #[token("cmos")]
    Cmos,
    #[token("deassign")]
    Deassign,
    #[token("default")]
    Default,
    #[token("defparam")]
    Defparam,
    #[token("disable")]
    Disable,
    #[token("edge")]
    Edge,
    #[token("else")]
    Else,
    #[token("end")]
    End,
    #[token("endcase")]
    Endcase,
    #[token("endfunction")]
    Endfunction,
    #[token("endmodule")]
    Endmodule,
    #[token("endprimitive")]
    Endprimitive,
    #[token("endspecify")]
    Endspecify,
    #[token("endtable")]
    Endtable,
    #[token("endtask")]
    Endtask,
    #[token("event")]
    Event,
    #[token("for")]
    For,
    #[token("force")]
    Force,
    #[token("forever")]
    Forever,
    #[token("fork")]
    Fork,
    #[token("function")]
    Function,
    #[token("highz0")]
    Highz0,
    #[token("highz1")]
    Highz1,
    #[token("if")]
    If,
    #[token("ifnone")]
    Ifnone,
    #[token("initial")]
    Initial,
    #[token("inout")]
    Inout,
    #[token("input")]
    Input,
    #[token("integer")]
    Integer,
    #[token("join")]
    Join,
    #[token("large")]
    Large,
    #[token("macromodule")]
    Macromodule,
    #[token("medium")]
    Medium,
    #[token("module")]
    Module,
    #[token("nand")]
    Nand,
    #[token("negedge")]
    Negedge,
    #[token("nmos")]
    Nmos,
    #[token("nor")]
    Nor,
    #[token("not")]
    Not,
    #[token("notif0")]
    Notif0,
    #[token("notif1")]
    Notif1,
    #[token("or")]
    Or,
    #[token("output")]
    Output,
    #[token("parameter")]
    Parameter,
    #[token("pmos")]
    Pmos,
    #[token("posedge")]
    Posedge,
    #[token("primitive")]
    Primitive,
    #[token("pull0")]
    Pull0,
    #[token("pull1")]
    Pull1,
    #[token("pulldown")]
    Pulldown,
    #[token("pullup")]
    Pullup,
    #[token("rcmos")]
    Rcmos,
    #[token("real")]
    Real,
    #[token("realtime")]
    Realtime,
    #[token("reg")]
    Reg,
    #[token("release")]
    Release,
    #[token("repeat")]
    Repeat,
    #[token("rnmos")]
    Rnmos,
    #[token("rpmos")]
    Rpmos,
    #[token("rtran")]
    Rtran,
    #[token("rtranif0")]
    Rtranif0,
    #[token("rtranif1")]
    Rtranif1,
    #[token("scalared")]
    Scalared,
    #[token("small")]
    Small,
    #[token("specify")]
    Specify,
    #[token("specparam")]
    Specparam,
    #[token("strong0")]
    Strong0,
    #[token("strong1")]
    Strong1,
    #[token("supply0")]
    Supply0,
    #[token("supply1")]
    Supply1,
    #[token("table")]
    Table,
    #[token("task")]
    Task,
    #[token("time")]
    Time,
    #[token("tran")]
    Tran,
    #[token("tranif0")]
    Tranif0,
    #[token("tranif1")]
    Tranif1,
    #[token("tri")]
    Tri,
    #[token("tri0")]
    Tri0,
    #[token("tri1")]
    Tri1,
    #[token("triand")]
    Triand,
    #[token("trior")]
    Trior,
    #[token("trireg")]
    Trireg,
    #[token("vectored")]
    Vectored,
    #[token("wait")]
    Wait,
    #[token("wand")]
    Wand,
    #[token("weak0")]
    Weak0,
    #[token("weak1")]
    Weak1,
    #[token("while")]
    While,
    #[token("wire")]
    Wire,
    #[token("wor")]
    Wor,
    #[token("xnor")]
    Xnor,
    #[token("xor")]
    Xor,
    // 1364-2001
    #[token("automatic")]
    Automatic,
    #[token("cell")]
    Cell,
    #[token("config")]
    Config,
    #[token("design")]
    Design,
    #[token("endconfig")]
    Endconfig,
    #[token("endgenerate")]
    Endgenerate,
    #[token("generate")]
    Generate,
    #[token("genvar")]
    Genvar,
    #[token("incdir")]
    Incdir,
    #[token("include")]
    Include,
    #[token("instance")]
    Instance,
    #[token("liblist")]
    Liblist,
    #[token("library")]
    Library,
    #[token("localparam")]
    Localparam,
    #[token("noshowcancelled")]
    Noshowcancelled,
    #[token("pulsestyle_ondetect")]
    PulsestyleOndetect,
    #[token("pulsestyle_onevent")]
    PulsestyleOnevent,
    #[token("showcancelled")]
    Showcancelled,
    #[token("signed")]
    Signed,
    #[token("unsigned")]
    Unsigned,
    #[token("use")]
    Use,
    // 1364-2005
    #[token("uwire")]
    Uwire,
    // 1800-2005
    #[token("alias")]
    Alias,
    #[token("always_comb")]
    AlwaysComb,
    #[token("always_ff")]
    AlwaysFf,
    #[token("always_latch")]
    AlwaysLatch,
    #[token("assert")]
    Assert,
    #[token("assume")]
    Assume,
    #[token("before")]
    Before,
    #[token("bind")]
    Bind,
    #[token("bins")]
    Bins,
    #[token("binsof")]
    Binsof,
    #[token("bit")]
    Bit,
    #[token("break")]
    Break,
    #[token("byte")]
    Byte,
    #[token("chandle")]
    Chandle,
    #[token("class")]
    Class,
    #[token("clocking")]
    Clocking,
    #[token("const")]
    Const,
    #[token("constraint")]
    Constraint,
    #[token("context")]
    Context,
    #[token("continue")]
    Continue,
    #[token("cover")]
    Cover,
    #[token("covergroup")]
    Covergroup,
    #[token("coverpoint")]
    Coverpoint,
    #[token("cross")]
    Cross,
    #[token("dist")]
    Dist,
    #[token("do")]
    Do,
    #[token("endclass")]
    Endclass,
    #[token("endclocking")]
    Endclocking,
    #[token("endgroup")]
    Endgroup,
    #[token("endinterface")]
    Endinterface,
    #[token("endpackage")]
    Endpackage,
    #[token("endprogram")]
    Endprogram,
    #[token("endproperty")]
    Endproperty,
    #[token("endsequence")]
    Endsequence,
    #[token("enum")]
    Enum,
    #[token("expect")]
    Expect,
    #[token("export")]
    Export,
    #[token("extends")]
    Extends,
    #[token("extern")]
    Extern,
    #[token("final")]
    Final,
    #[token("first_match")]
    FirstMatch,
    #[token("foreach")]
    Foreach,
    #[token("forkjoin")]
    Forkjoin,
    #[token("iff")]
    Iff,
    #[token("ignore_bins")]
    IgnoreBins,
    #[token("illegal_bins")]
    IllegalBins,
    #[token("import")]
    Import,
    #[token("inside")]
    Inside,
    #[token("int")]
    Int,
    #[token("interface")]
    Interface,
    #[token("intersect")]
    Intersect,
    #[token("join_any")]
    JoinAny,
    #[token("join_none")]
    JoinNone,
    #[token("local")]
    Local,
    #[token("logic")]
    Logic,
    #[token("longint")]
    Longint,
    #[token("matches")]
    Matches,
    #[token("modport")]
    Modport,
    #[token("new")]
    New,
    #[token("null")]
    Null,
    #[token("package")]
    Package,
    #[token("packed")]
    Packed,
    #[token("priority")]
    Priority,
    #[token("program")]
    Program,
    #[token("property")]
    Property,
    #[token("protected")]
    Protected,
    #[token("pure")]
    Pure,
    #[token("rand")]
    Rand,
    #[token("randc")]
    Randc,
    #[token("randcase")]
    Randcase,
    #[token("randsequence")]
    Randsequence,
    #[token("ref")]
    Ref,
    #[token("return")]
    Return,
    #[token("sequence")]
    Sequence,
    #[token("shortint")]
    Shortint,
    #[token("shortreal")]
    Shortreal,
    #[token("solve")]
    Solve,
    #[token("static")]
    Static,
    #[token("string")]
    String,
    #[token("struct")]
    Struct,
    #[token("super")]
    Super,
    #[token("tagged")]
    Tagged,
    #[token("this")]
    This,
    #[token("throughout")]
    Throughout,
    #[token("timeprecision")]
    Timeprecision,
    #[token("timeunit")]
    Timeunit,
    #[token("type")]
    Type,
    #[token("typedef")]
    Typedef,
    #[token("union")]
    Union,
    #[token("unique")]
    Unique,
    #[token("var")]
    Var,
    #[token("virtual")]
    Virtual,
    #[token("void")]
    Void,
    #[token("wait_order")]
    WaitOrder,
    #[token("wildcard")]
    Wildcard,
    #[token("with")]
    With,
    #[token("within")]
    Within,
    // 1800-2009
    #[token("accept_on")]
    AcceptOn,
    #[token("checker")]
    Checker,
    #[token("endchecker")]
    Endchecker,
    #[token("eventually")]
    Eventually,
    #[token("global")]
    Global,
    #[token("implies")]
    Implies,
    #[token("let")]
    Let,
    #[token("nexttime")]
    Nexttime,
    #[token("reject_on")]
    RejectOn,
    #[token("restrict")]
    Restrict,
    #[token("s_always")]
    SAlways,
    #[token("s_eventually")]
    SEventually,
    #[token("s_nexttime")]
    SNexttime,
    #[token("s_until")]
    SUntil,
    #[token("s_until_with")]
    SUntilWith,
    #[token("strong")]
    Strong,
    #[token("sync_accept_on")]
    SyncAcceptOn,
    #[token("sync_reject_on")]
    SyncRejectOn,
    #[token("unique0")]
    Unique0,
    #[token("until")]
    Until,
    #[token("until_with")]
    UntilWith,
    #[token("untyped")]
    Untyped,
    #[token("weak")]
    Weak,
    // 1800-2012
    #[token("implements")]
    Implements,
    #[token("interconnect")]
    Interconnect,
    #[token("nettype")]
    Nettype,
    #[token("soft")]
    Soft,
    // Directives
    #[token("`__FILE__")]
    DirUnderscoreFile,
    #[token("`__LINE__")]
    DirUnderscoreLine,
    #[token("`begin_keywords")]
    DirBeginKeywords,
    #[token("`celldefine")]
    DirCelldefine,
    #[token("`default_nettype")]
    DirDefaultNettype,
    #[token("`define")]
    DirDefine,
    #[token("`else")]
    DirElse,
    #[token("`elsif")]
    DirElsif,
    #[token("`end_keywords")]
    DirEndKeywords,
    #[token("`endcelldefine")]
    DirEndcelldefine,
    #[token("`endif")]
    DirEndif,
    #[token("`ifdef")]
    DirIfdef,
    #[token("`ifndef")]
    DirIfndef,
    #[token("`include")]
    DirInclude,
    #[token("`line")]
    DirLine,
    #[token("`nounconnected_drive")]
    DirNounconnectedDrive,
    #[token("`pragma")]
    DirPragma,
    #[token("`resetall")]
    DirResetall,
    #[token("`timescale")]
    DirTimescale,
    #[token("`unconnected_drive")]
    DirUnconnectedDrive,
    #[token("`undef")]
    DirUndef,
    #[token("`undefineall")]
    DirUndefineall,
    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("!")]
    Exclamation,
    #[token("?")]
    Quest,
    #[token("~")]
    Tilde,
    #[token("&")]
    Amp,
    #[token("~&")]
    TildeAmp,
    #[token("|")]
    Pipe,
    #[token("~|")]
    TildePipe,
    #[token("^")]
    Caret,
    #[token("~^")]
    TildeCaret,
    #[token("^~")]
    CaretTilde,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("==")]
    EqEq,
    #[token("!=")]
    ExclEq,
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    StarEq,
    #[token("/=")]
    SlashEq,
    #[token("%=")]
    PercentEq,
    #[token("&=")]
    AmpEq,
    #[token("|=")]
    PipeEq,
    #[token("^=")]
    CaretEq,
    #[token("===")]
    EqEqEq,
    #[token("!==")]
    ExclEqEq,
    #[token("==?")]
    EqEqQuest,
    #[token("!=?")]
    ExclEqQuest,
    #[token("&&")]
    AmpAmp,
    #[token("&&&")]
    AmpAmpAmp,
    #[token("||")]
    PipePipe,
    #[token("**")]
    StarStar,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    #[token(">>")]
    GtGt,
    #[token("<<")]
    LtLt,
    #[token(">>=")]
    GtGtEq,
    #[token("<<=")]
    LtLtEq,
    #[token(">>>")]
    GtGtGt,
    #[token("<<<")]
    LtLtLt,
    #[token(">>>=")]
    GtGtGtEq,
    #[token("<<<=")]
    LtLtLtEq,
    #[token("->")]
    MinusGt,
    #[token("->>")]
    MinusGtGt,
    #[token("<->")]
    LtMinusGt,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("+:")]
    PlusColon,
    #[token("-:")]
    MinusColon,
    #[token("+/-")]
    PlusSlashMinus,
    #[token("+%-")]
    PlusPercentMinus,
    // Symbols
    #[token("(")]
    Paren,
    #[token(")")]
    EParen,
    #[token("[")]
    Bracket,
    #[token("]")]
    EBracket,
    #[token("{")]
    Brace,
    #[token("}")]
    EBrace,
    #[token(":")]
    Colon,
    #[token(";")]
    SColon,
    #[token("'")]
    Apost,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    #[token("#")]
    Pound,
    #[token("$")]
    Dollar,
    #[token("@")]
    At,
    #[token("@@")]
    AtAt,
    #[token("=")]
    Eq,
    #[token("::")]
    ColonColon,
    #[token(":=")]
    ColonEq,
    #[token(":/")]
    ColonSlash,
    #[token("##")]
    PoundPound,
    #[token("#-#")]
    PoundMinusPound,
    #[token("#=#")]
    PoundEqPound,
    #[token("=>")]
    EqGt,
    #[token("*>")]
    StarGt,
    #[token("|->")]
    PipeMinusGt,
    #[token("|=>")]
    PipeEqGt,
    #[token(r"\")]
    Bslash,
    // Other Language Grammar
    #[token("std")]
    Std,
    #[token("PATHPULSE$")]
    PathpulseDollar,
    #[token("option")]
    Option,
    #[token("type_option")]
    TypeOption,
    #[token("randomize")]
    Randomize,
    #[token("sample")]
    Sample,
    #[token("1step")]
    OneStep,
    #[token("$setup")]
    DollarSetup,
    #[token("$hold")]
    DollarHold,
    #[token("$setuphold")]
    DollarSetuphold,
    #[token("$recovery")]
    DollarRecovery,
    #[token("$removal")]
    DollarRemoval,
    #[token("$recrem")]
    DollarRecrem,
    #[token("$skew")]
    DollarSkew,
    #[token("$timeskew")]
    DollarTimeskew,
    #[token("$fullskew")]
    DollarFullskew,
    #[token("$period")]
    DollarPeriod,
    #[token("$width")]
    DollarWidth,
    #[token("$nochange")]
    DollarNochange,
    #[token("$root")]
    DollarRoot,
    #[token("$unit")]
    DollarUnit,
    #[token("$fatal")]
    DollarFatal,
    #[token("$error")]
    DollarError,
    #[token("$warning")]
    DollarWarning,
    #[token("$info")]
    DollarInfo,
    // Comments
    #[regex(r"//[^\r\n]*", oneline_comment, allow_greedy = true)]
    OnelineComment(&'a str),
    #[token(r"/*", block_comment)]
    BlockComment(&'a str),
    // Numbers
    #[regex(r"[0-9][0-9_]*", |lex| lex.slice())]
    UnsignedNumber(&'a str),
    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*", |lex| lex.slice())]
    FixedPointNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(b|B)\s*[0-1xXzZ\?][0-1xXzZ\?_]*", |lex| lex.slice())]
    BinaryNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(o|O)\s*[0-7xXzZ\?][0-7xXzZ\?_]*", |lex| lex.slice())]
    OctalNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(d|D)\s*[0-9][0-9_]*", |lex| lex.slice())]
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(d|D)\s*(x|X|z|Z|\?)_*", |lex| lex.slice())]
    DecimalNumber(&'a str),
    #[regex(r"([0-9][0-9_]*)?\s*'[s|S]?(h|H)\s*[0-9a-fA-FxXzZ\?][0-9a-fA-FxXzZ\?_]*", |lex| lex.slice())]
    HexNumber(&'a str),
    #[regex(r"[0-9][0-9_]*(\.[0-9][0-9_]*)?(e|E)(\+|-)?[0-9][0-9_]*", |lex| lex.slice())]
    ScientificNumber(&'a str),
    #[regex(r"('0|'1|'x|'X|'z|'Z|'\?)", |lex| lex.slice())]
    UnbasedUnsizedLiteral(&'a str),
    // Literals
    #[regex(r"\$[a-zA-Z0-9_\$]+", |lex| lex.slice())]
    SystemTfIdentifier(&'a str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_\$]*", |lex| lex.slice())]
    SimpleIdentifier(&'a str),
    #[regex(r"\\[!-~]+\s", |lex| lex.slice())]
    EscapedIdentifier(&'a str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_\$]*(``([a-zA-Z_][a-zA-Z0-9_\$]*)?)+", |lex| lex.slice())]
    PreprocessorIdentifier(&'a str),
    #[regex(r"`[a-zA-Z_][a-zA-Z0-9_\$]*", text_macro)]
    #[regex(r"`\\[!-~]+", text_macro)]
    TextMacro(&'a str),
    #[regex(
        r"`[a-zA-Z_][a-zA-Z0-9_\$]*(``([a-zA-Z_][a-zA-Z0-9_\$]*)?)+",
        text_macro
    )]
    ConcatenatedTextMacro(&'a str),
    #[token(r#"""#, string_literal)]
    StringLiteral(&'a str),
    #[token(r#"`""#, preprocessor_string_literal)]
    PreprocessorStringLiteral(&'a str),
    #[token(r#"""""#, multiline_string_literal)]
    TripleQuoteStringLiteral(&'a str),
    #[token(r#"`""""#, preprocessor_multiline_string_literal)]
    PreprocessorTripleQuoteStringLiteral(&'a str),
    #[token("\n")]
    #[token("\r")]
    #[token("\r\n")]
    #[token("\u{0085}")]
    #[token("\u{2028}")]
    #[token("\u{2029}")]
    Newline,
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
            | Token::TextMacro(_)
            | Token::ConcatenatedTextMacro(_) => true,
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
            Token::Std => "std",
            Token::PathpulseDollar => "PATHPULSE$",
            Token::Option => "option",
            Token::TypeOption => "type_option",
            Token::Randomize => "randomize",
            Token::Sample => "sample",
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
            Token::PreprocessorIdentifier(_text) => "<preprocessor identifier>",
            Token::TextMacro(_text) => "<text macro>",
            Token::ConcatenatedTextMacro(_text) => "<concatenated text macro>",
            Token::StringLiteral(_text) => "<string>",
            Token::PreprocessorStringLiteral(_text) => "<preprocessor string>",
            Token::TripleQuoteStringLiteral(_text) => "<triple-quote string>",
            Token::PreprocessorTripleQuoteStringLiteral(_text) => {
                "<preprocessor triple-quote string>"
            }
            Token::Newline => "newline",
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
            Token::PreprocessorIdentifier(text) => {
                temp_str = format!("preprocessor identifier '{}'", text);
                temp_str.as_str()
            }
            Token::TextMacro(text) => {
                temp_str = format!("text macro '{}'", text);
                temp_str.as_str()
            }
            Token::ConcatenatedTextMacro(text) => {
                temp_str = format!("concatenated text macro '{}'", text);
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
            _ => self.as_str(),
        };
        write!(f, "{}", str_repr)
    }
}
