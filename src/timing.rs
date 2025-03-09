//! Timing Groups

use ref_cast::RefCast;
use rxegy_sys::XC_GROUP_EVENT_TIMING;

#[derive(Clone, Debug, RefCast)]
#[repr(transparent)]
pub struct EventTiming(XC_GROUP_EVENT_TIMING);

impl EventTiming {
    /// Create a new event timing object.
    pub(crate) fn new(inner: XC_GROUP_EVENT_TIMING) -> Self {
        Self(inner)
    }

    /// The exchange sequence number of the corresponding event.
    pub fn exchange_sequence(&self) -> Option<u64> {
        wrap_in_option(self.0.xtev_sequence)
    }

    /// The exchange timestamp for the corresponding event.
    pub fn exchange_timestamp(&self) -> Option<u64> {
        wrap_in_option(self.0.xtev_exchange_hitime)
    }

    /// The timestamp the event was received by the Exegy appliance.
    pub fn appliance_receive_timestamp(&self) -> Option<u64> {
        wrap_in_option(self.0.xtev_receive_hitime)
    }

    /// The timestamp the event was transmitted by the Exegy appliance.
    pub fn appliance_transmit_timestamp(&self) -> Option<u64> {
        wrap_in_option(self.0.xtev_transmit_hitime)
    }

    /// The time when the event was received by the XCAPI library.
    pub fn xcapi_receive_timestamp(&self) -> Option<u64> {
        wrap_in_option(self.0.xtev_xcapi_receive_hitime)
    }

    /// The time when the callback was fired by the XCAPI library.
    pub fn xcapi_callback_timestamp(&self) -> Option<u64> {
        wrap_in_option(self.0.xtev_xcapi_callback_hitime)
    }

    /// Record the spans indicated by this timing group.
    pub fn record_spans(&self) {
        // TODO: hook into tracing
    }
}

#[inline(always)]
fn wrap_in_option(value: u64) -> Option<u64> {
    if value == 0 { None } else { Some(value) }
}
