//! Event Objects

pub use self::{
    equity::{
        Cancel as EquityCancel, Correction as EquityCorrection, Quote as EquityQuote,
        Refresh as EquityRefresh, Trade as EquityTrade,
    },
    exchange_statistics::Event as ExchangeStatistics,
    indicative_price::Event as IndicativePrice,
    keylist::{CatalogRefresh as KeylistCatalogRefresh, CatalogUpdate as KeylistCatalogUpdate},
    order_imbalance::Event as OrderImbalance,
    subscribe::Event as Subscribe,
    trade_summary::Event as TradeSummary,
    trading_action::Event as TradingAction,
};

mod equity;
mod exchange_statistics;
mod indicative_price;
mod keylist;
mod order_imbalance;
mod subscribe;
mod trade_summary;
mod trading_action;

use crate::{
    error::{ExegyError, Result, Success},
    field::{self, Field as FieldTrait},
    key::Key,
    object::Wrapper,
    timing::EventTiming,
};
use std::result::Result as StdResult;

/// Field accessors common to every Exegy event.
#[allow(private_bounds)]
pub trait Common: Wrapper {
    /// Retrieve the status code from the event
    fn status(&self) -> Result<StdResult<Success, ExegyError>> {
        field::get_u32(self, rxegy_sys::XC_EVENT, Field::Status).map(Success::try_from)
    }

    /// Retrieve the key of the item (channel) this event refers to
    fn item_key(&self) -> Result<Key> {
        field::get_xc_key(self, rxegy_sys::XC_EVENT, Field::ItemKey).map(Key::new)
    }

    /// Retrieve the key string of the item (channel) this event refers to
    fn item_key_string(&self) -> Result<String> {
        field::get_fixedstring(self, rxegy_sys::XC_EVENT, Field::ItemKeyString, 80)
    }

    /// Retrieve the line ID the event was received from.
    fn line_id(&self) -> Result<u16> {
        field::get_u16(self, rxegy_sys::XC_EVENT, Field::LineId)
    }

    /// Retreive the time the event was received by Exegy
    fn receive_time(&self) -> Result<u64> {
        field::get_u64(self, rxegy_sys::XC_EVENT, Field::ReceiveTime)
    }

    /// Retrieve the time the event was transmitted by Exegy
    fn transmit_time(&self) -> Result<u64> {
        field::get_u64(self, rxegy_sys::XC_EVENT, Field::TransmitTime)
    }

    /// Retrieve the time the event was received by Exegy
    fn xcapi_receive_timestamp(&self) -> Result<u64> {
        field::get_u64(self, rxegy_sys::XC_EVENT, Field::XcapiReceiveTime)
    }

    /// Retrieve the timestamp the callback was fired
    fn xcapi_callback_timestamp(&self) -> Result<u64> {
        field::get_u64(self, rxegy_sys::XC_EVENT, Field::XcapiCallbackTime)
    }

    /// Retrieve the exchange sequence number of the event
    fn exchange_sequence(&self) -> Result<u64> {
        field::get_u64(self, rxegy_sys::XC_EVENT, Field::ExchangeSequence)
    }

    /// Retrieve the timings of this event
    fn timing(&self) -> Result<EventTiming> {
        field::get_xc_group_event_timing(self, rxegy_sys::XC_EVENT, Field::TimingGroup)
            .map(EventTiming::new)
    }
}

#[derive(Clone, Copy)]
#[repr(u64)]
pub(crate) enum Field {
    /// Client-specified turnaround key (set during object creation).
    #[allow(dead_code)]
    Turnkey = rxegy_sys::XFLD_EVT_TURNKEY,
    /// Type of event object.
    #[allow(dead_code)]
    EventType = rxegy_sys::XFLD_EVT_EVENT_TYPE,
    /// Exegy timestamp indicating when the event was received by the appliance.
    ReceiveTime = rxegy_sys::XFLD_EVT_RECEIVE_HITIME,
    /// Exegy timestamp indicating when the event was sent to the client by the appliance.
    TransmitTime = rxegy_sys::XFLD_EVT_TRANSMIT_HITIME,
    /// Exegy timestamp indicating when the event was received by the Exegy Client API (XCAPI) on the client machine.
    XcapiReceiveTime = rxegy_sys::XFLD_EVT_XCAPI_RECEIVE_HITIME,
    /// Exegy timestamp indicating when the event was received by the client callback routine in the client application.
    XcapiCallbackTime = rxegy_sys::XFLD_EVT_XCAPI_CALLBACK_HITIME,
    /// Current session status.
    Status = rxegy_sys::XFLD_EVT_STATUS,
    /// Key for the data item associated with this event.
    ItemKey = rxegy_sys::XFLD_EVT_ITEM_KEY,
    /// The key string for the subscribed-to instrument.
    ItemKeyString = rxegy_sys::XFLD_EVT_ITEM_KEY_STRING,
    /// Line identifier.
    LineId = rxegy_sys::XFLD_EVT_LINE_ID,
    /// Sequence number from the exchange, typically the packet sequence number.
    ExchangeSequence = rxegy_sys::XFLD_EVT_SEQUENCE,
    /// A timing group structure
    TimingGroup = rxegy_sys::XFGRP_EVT_TIMING,
}

impl FieldTrait for Field {
    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
