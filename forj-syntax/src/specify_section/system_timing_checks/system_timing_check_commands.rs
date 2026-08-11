// =======================================================================
// specify_timing_check_commands.rs
// =======================================================================
//! CST Nodes from 1800-2023 A.7.5.1
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum SystemTimingCheck<'a> {
    Setup(Box<DollarSetupTimingCheck<'a>>),
    Hold(Box<DollarHoldTimingCheck<'a>>),
    Setuphold(Box<DollarSetupholdTimingCheck<'a>>),
    Recovery(Box<DollarRecoveryTimingCheck<'a>>),
    Removal(Box<DollarRemovalTimingCheck<'a>>),
    Recrem(Box<DollarRecremTimingCheck<'a>>),
    Skew(Box<DollarSkewTimingCheck<'a>>),
    Timeskew(Box<DollarTimeskewTimingCheck<'a>>),
    Fullskew(Box<DollarFullskewTimingCheck<'a>>),
    Period(Box<DollarPeriodTimingCheck<'a>>),
    Width(Box<DollarWidthTimingCheck<'a>>),
    Nochange(Box<DollarNochangeTimingCheck<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DollarSetupTimingCheck<'a>(
    pub Metadata<'a>, // $setup
    pub Metadata<'a>, // (
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarHoldTimingCheck<'a>(
    pub Metadata<'a>, // $hold
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarSetupholdTimingCheck<'a>(
    pub Metadata<'a>, // $setuphold
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
        Option<(
            Metadata<'a>, // ,
            Option<TimestampCondition<'a>>,
            Option<(
                Metadata<'a>, // ,
                Option<TimecheckCondition<'a>>,
                Option<(
                    Metadata<'a>, // ,
                    Option<DelayedReference<'a>>,
                    Option<(
                        Metadata<'a>, // ,
                        Option<DelayedData<'a>>,
                    )>,
                )>,
            )>,
        )>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarRecoveryTimingCheck<'a>(
    pub Metadata<'a>, // $recovery
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarRemovalTimingCheck<'a>(
    pub Metadata<'a>, // $removal
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarRecremTimingCheck<'a>(
    pub Metadata<'a>, // $recrem
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
        Option<(
            Metadata<'a>, // ,
            Option<TimestampCondition<'a>>,
            Option<(
                Metadata<'a>, // ,
                Option<TimecheckCondition<'a>>,
                Option<(
                    Metadata<'a>, // ,
                    Option<DelayedReference<'a>>,
                    Option<(
                        Metadata<'a>, // ,
                        Option<DelayedData<'a>>,
                    )>,
                )>,
            )>,
        )>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarSkewTimingCheck<'a>(
    pub Metadata<'a>, // $skew
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarTimeskewTimingCheck<'a>(
    pub Metadata<'a>, // $timeskew
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
        Option<(
            Metadata<'a>,
            Option<EventBasedFlag<'a>>,
            Option<(Metadata<'a>, Option<RemainActiveFlag<'a>>)>,
        )>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarFullskewTimingCheck<'a>(
    pub Metadata<'a>, // $fullskew
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
        Option<(
            Metadata<'a>,
            Option<EventBasedFlag<'a>>,
            Option<(Metadata<'a>, Option<RemainActiveFlag<'a>>)>,
        )>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarPeriodTimingCheck<'a>(
    pub Metadata<'a>, // $period
    pub Metadata<'a>, // (
    pub ControlledReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarWidthTimingCheck<'a>(
    pub Metadata<'a>, // $width
    pub Metadata<'a>, // (
    pub ControlledReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub TimingCheckLimit<'a>,
    pub Metadata<'a>, // ,
    pub Threshold<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct DollarNochangeTimingCheck<'a>(
    pub Metadata<'a>, // $nochange
    pub Metadata<'a>, // (
    pub ReferenceEvent<'a>,
    pub Metadata<'a>, // ,
    pub DataEvent<'a>,
    pub Metadata<'a>, // ,
    pub StartEdgeOffset<'a>,
    pub Metadata<'a>, // ,
    pub EndEdgeOffset<'a>,
    pub  Option<(
        Metadata<'a>, // ,
        Option<Notifier<'a>>,
    )>,
    pub Metadata<'a>, // )
    pub Metadata<'a>, // ;
);
