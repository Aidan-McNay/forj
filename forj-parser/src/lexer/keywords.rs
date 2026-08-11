// =======================================================================
// keywords.rs
// =======================================================================
// The keywords used for various SystemVerilog standards

use crate::*;

/// A specific SystemVerilog standard to operate under
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StandardVersion {
    IEEE1364_1995,
    IEEE1364_2001,
    IEEE1364_2001Noconfig,
    IEEE1364_2005,
    IEEE1800_2005,
    IEEE1800_2009,
    IEEE1800_2012,
    IEEE1800_2017,
    IEEE1800_2023,
}

impl Default for StandardVersion {
    fn default() -> Self {
        StandardVersion::IEEE1800_2023
    }
}

impl<'a> Token<'a> {
    /// Whether the token is a keyword that needs to be replaced, based on
    /// the current standard
    pub fn keyword_replace(&self, standard: &StandardVersion) -> bool {
        let valid_in_standard = match self {
            Token::Automatic
            | Token::Endgenerate
            | Token::Generate
            | Token::Genvar
            | Token::Localparam
            | Token::Noshowcancelled
            | Token::PulsestyleOndetect
            | Token::PulsestyleOnevent
            | Token::Showcancelled
            | Token::Signed
            | Token::Unsigned => *standard >= StandardVersion::IEEE1364_2001,
            Token::Cell
            | Token::Config
            | Token::Design
            | Token::Endconfig
            | Token::Incdir
            | Token::Include
            | Token::Instance
            | Token::Liblist
            | Token::Library
            | Token::Use => {
                (*standard >= StandardVersion::IEEE1364_2001)
                    & (*standard != StandardVersion::IEEE1364_2001Noconfig)
            }
            Token::Uwire => *standard >= StandardVersion::IEEE1364_2005,
            Token::Alias
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
            | Token::Within => *standard >= StandardVersion::IEEE1800_2005,
            Token::AcceptOn
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
            | Token::Weak => *standard >= StandardVersion::IEEE1800_2009,
            Token::Implements
            | Token::Interconnect
            | Token::Nettype
            | Token::Soft => *standard >= StandardVersion::IEEE1800_2012,
            _ => true,
        };
        !valid_in_standard
    }
}
