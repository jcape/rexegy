use ref_cast::RefCast;
use rxegy_sys::XC_TRADE_VENUE;

/// A trade venue, stored as a 4-ascii-character MIC code
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct TradeVenue(XC_TRADE_VENUE);

impl TradeVenue {
    pub(crate) fn new(inner: XC_TRADE_VENUE) -> Self {
        Self(inner)
    }
}
