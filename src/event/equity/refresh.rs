//! Equity Refresh Events

use ref_cast::RefCast;
use rxegy_sys::XC_REFRESH_QUALS;

use crate::{
    HiTime, InstrumentStatus, MarketStatus, Price, event::Common, impl_wrapper_on_newtype,
    object::Kind as ObjectKind,
};
use std::{ffi::c_void, ptr::NonNull};

/// An equity refresh event object
#[derive(Debug)]
pub struct Event(NonNull<c_void>);

impl_wrapper_on_newtype!(Event, ObjectKind::EventEquityRefresh);

impl Common for Event {}

crate::impl_event_fields! {
    Event =>

        // Exchange Timestamp
        "Get the exchange timestamp indicating when the event left the exchange.",
        ExchangeHiTime, rxegy_sys::XFLD_EVT_EQTY_REFR_EXCHANGE_HITIME, exchange_hitime,
        get_u64, HiTime, HiTime::from;

        // Instrument Status
        "Get the normalized trading status/substatus for the instrument.

See the _Exegy Feature Brief: Market and Instrument Status_ for a detailed discussion of how Exegy
normalizes status indications across markets. Note that for composite, depth-of-book views, the
instrument status for each contributing market is provided as an array.",
        InstrumentStatus, rxegy_sys::XFLD_EVT_EQTY_REFR_INSTRUMENT_STATUS, instrument_status,
        get_xc_trading_state, InstrumentStatus, InstrumentStatus::new;

        // Market Status
        "Get the normalized trading status/substatus for the instrument.

See the _Exegy Feature Brief: Market and Instrument Status_ for a detailed discussion of how Exegy
normalizes status indications across markets. Note that for composite, depth-of-book views, the
instrument status for each contributing market is provided as an array.",
        MarketStatus, rxegy_sys::XFLD_EVT_EQTY_REFR_MARKET_STATUS, market_status,
        get_xc_trading_state, MarketStatus, MarketStatus::new;

        // Closing Price
        "Get the closing price.",
        ClosePrice, rxegy_sys::XFLD_EVT_EQTY_REFR_CLOSE_PRICE, close_price,
        get_i32, Price, Price::from;

        // Closing Time
        "Get the exchange-provided timestamp indicating the time of the close.",
        CloseHiTime, rxegy_sys::XFLD_EVT_EQTY_REFR_CLOSE_HITIME, close_hitime,
        get_u64, HiTime, HiTime::from;

        // Quote Sequence
        "Get the exchange timestamp indicating when the event left the exchange.",
        QuoteSequence, rxegy_sys::XFLD_EVT_EQTY_REFR_QUOTE_SEQUENCE, quote_sequence,
        get_u64, u64;

        // Morning VWAP
        "Get the exchange-provided VWAP value for the morning trading session.

Only populated, cached, and persisted if explicitly provided by the exchange.",
        MorningVwap, rxegy_sys::XFLD_EVT_EQTY_REFR_MORNING_VWAP, morning_vwap,
        get_i32, Price, Price::from;

        // Afternoon VWAP
        "Get the exchange-provided VWAP value for the afternoon trading session.

Only populated, cached, and persisted if explicitly provided by the exchange.",
        AfternoonVwap, rxegy_sys::XFLD_EVT_EQTY_REFR_AFTERNOON_VWAP, afternoon_vwap,
        get_i32, Price, Price::from;

        // All-Day VWAP
        "Get the exchange-provided VWAP value for the entire day's trading session.

Only populated, cached, and persisted if explicitly provided by the exchange.",
        AllDayVwap, rxegy_sys::XFLD_EVT_EQTY_REFR_ALLDAY_VWAP, allday_vwap,
        get_i32, Price, Price::from;
}

/// A set of refresh qualifiers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct RefreshQuals(XC_REFRESH_QUALS);

impl RefreshQuals {
    pub(crate) fn new(inner: XC_REFRESH_QUALS) -> Self {
        Self(inner)
    }
}
