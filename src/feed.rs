//! Exegy Feed Identifiers

use crate::group::Group;
use ref_cast::RefCast;

pub mod us;
pub mod xx;

/// Exegy Feed ID
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct Id(rxegy_sys::XC_EXCHANGE_ID);

/// A trait for group-specific feeds to retrieve
pub trait Feed {
    /// Retrieve the SGMT or operating MIC for a given feed
    fn mic(&self) -> Option<i32>;

    /// Retrieve the feed group ("exegy country") this feed is part of
    fn group(&self) -> Group;
}
