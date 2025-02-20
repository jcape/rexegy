//! Exegy Feed Identifiers

use ref_cast::RefCast;

pub mod us;
pub mod xx;

/// Exegy Feed ID
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct Id(rexegy_sys::XC_EXCHANGE_ID);
