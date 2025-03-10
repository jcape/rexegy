//! Exegy Feed Identifiers

pub use self::{us::Feed as Us, xx::Feed as Internal};

mod us;
mod xx;

use crate::group::Group;
use ref_cast::RefCast;
use rxegy_sys::XC_EXCHANGE_ID;

/// Exegy Feed ID
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct Id(XC_EXCHANGE_ID);

impl Id {
    #[inline(always)]
    pub(crate) fn new(inner: XC_EXCHANGE_ID) -> Self {
        Self(inner)
    }
}

/// A trait for group-specific feeds to retrieve
pub trait Feed {
    /// Retrieve the SGMT or operating MIC for a given feed
    fn mic(&self) -> Option<i32>;

    /// Retrieve the feed group ("exegy country") this feed is part of
    fn group(&self) -> Group;
}
