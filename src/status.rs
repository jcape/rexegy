//! Trading State

use ref_cast::RefCast;
use rxegy_sys::XC_TRADING_STATE;

/// Instrument status
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct Instrument(XC_TRADING_STATE);

impl Instrument {
    pub(crate) fn new(inner: XC_TRADING_STATE) -> Self {
        Self(inner)
    }
}

/// Market status
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct Market(XC_TRADING_STATE);

impl Market {
    pub(crate) fn new(inner: XC_TRADING_STATE) -> Self {
        Self(inner)
    }
}
