//! Top-of-book Containers for Equity and Equity Options Streams

use crate::{
    AlternateId, FeedId, GroupId, InstrumentStatus, Key, MarketStatus, TradeVenue,
    container::RealTime,
    error::Result,
    event::{
        EquityCancel, EquityCorrection, EquityQuote, EquityRefresh, EquityTrade,
        ExchangeStatistics, IndicativePrice, OrderImbalance, Subscribe, TradeSummary,
        TradingAction,
    },
    field::{self, Field as FieldTrait},
    impl_wrapper_on_newtype,
    object::Kind as ObjectKind,
};
use std::{ffi::c_void, ptr::NonNull};

/// An equity stream container.
#[derive(Debug)]
pub struct Stream(NonNull<c_void>);

impl_wrapper_on_newtype!(Stream, ObjectKind::RealtimeEquityStream);

impl RealTime for Stream {}

impl Stream {
    /// Retrieve the maximum rate, in quotes per second, for "metered quote delivery" to
    /// subscriptions for this object.
    pub fn quote_rate(&self) -> Result<u32> {
        field::get_u32(self, rxegy_sys::XC_CONTAINER, Field::QuoteRate)
    }

    /// Set the maximum rate, in quotes per second, for "metered quote delivery" to subscriptions
    /// for this object.
    ///
    /// Note that setting the field does not change the quote delivery rate in effect for previously
    /// existing subscriptions. When a subscription request is made, XCAPI checks the field and
    /// applies the per second rate found there to the requested subscription only. (This allows
    /// users to specify a different maximum rate for each subscription in their application, if
    /// desired.) If the number of quote events for a given subscription then exceeds its
    /// user-specified rate, quotes are conflated in a manner such that the subscribing application
    /// always receives the freshest quote available without exceeding the applicable per-second
    /// delivery limit. Other events, such as trades and trading actions, are not affected by
    /// metered quote delivery. Default value: 0 ("unlimited quotes"). Edge-Cache connected users
    /// should note that Edge Cache configuration settings can also affect quote metering. (If so
    /// configured, the Edge Cache can enforce a base maximum delivery rate for top-of-book quotes
    /// to all connected applications. In this situation, the configured rate acts as a cap,
    /// although the "MAX_QUOTE_RATE" field can still be used to set a lower rate on a per
    /// subscription basis, if desired.) Contact your Exegy Technical Account Representative for
    /// information on the current configuration settings for your installation.
    pub fn set_quote_rate(&self, qps: u32) -> Result<()> {
        field::set_u32(self, rxegy_sys::XC_CONTAINER, Field::QuoteRate, qps)
    }

    pub fn key(&self, slot: u32) -> Result<Key> {
        field::get_xc_key(self, slot, Field::Key).map(Key::new)
    }

    pub fn key_string(&self, slot: u32) -> Result<String> {
        field::get_fixedstring(self, slot, Field::KeyString, 80)
    }

    pub fn update_recv_hitime(&self, slot: u32) -> Result<u64> {
        field::get_u64(self, slot, Field::UpdateReceiveHitime)
    }

    pub fn update_xcapi_recv_hitime(&self, slot: u32) -> Result<u64> {
        field::get_u64(self, slot, Field::UpdateXcapiReceiveHitime)
    }

    pub fn alternate_id1(&self, slot: u32) -> Result<AlternateId> {
        field::get_xc_alternate_id(self, slot, Field::AlternateId1).map(AlternateId::new)
    }

    pub fn alternate_id2(&self, slot: u32) -> Result<AlternateId> {
        field::get_xc_alternate_id(self, slot, Field::AlternateId2).map(AlternateId::new)
    }

    pub fn order_ref_id_type(&self, slot: u32) -> Result<u8> {
        field::get_u8(self, slot, Field::OrderRefIdType)
    }

    pub fn price_type(&self, slot: u32) -> Result<u8> {
        field::get_u8(self, slot, Field::PriceType)
    }

    pub fn symbol_type(&self, slot: u32) -> Result<u8> {
        field::get_u8(self, slot, Field::SymbolType)
    }

    pub fn instrument_status(&self, slot: u32) -> Result<InstrumentStatus> {
        field::get_xc_trading_state(self, slot, Field::InstrumentStatus).map(InstrumentStatus::new)
    }

    pub fn market_status(&self, slot: u32) -> Result<MarketStatus> {
        field::get_xc_trading_state(self, slot, Field::MarketStatus).map(MarketStatus::new)
    }

    pub fn primary_feed_id(&self, slot: u32) -> Result<FeedId> {
        field::get_xc_exchange_id(self, slot, Field::PrimeExch).map(FeedId::new)
    }

    pub fn primary_group_id(&self, slot: u32) -> Result<GroupId> {
        field::get_xc_country_id(self, slot, Field::PrimeCountry).map(GroupId::new)
    }

    pub fn primary_trade_venue(&self, slot: u32) -> Result<TradeVenue> {
        field::get_xc_trade_venue(self, slot, Field::PrimeTradeVenue).map(TradeVenue::new)
    }

    pub fn lot_size(&self, slot: u32) -> Result<u16> {
        field::get_u16(self, slot, Field::LotSize)
    }

    pub fn short_sale_restricted(&self, slot: u32) -> Result<bool> {
        Ok(field::get_u8(self, slot, Field::ShortSaleRestricted)? != 0)
    }
}

/// The function prototype for a subscription callback.
pub type SubscribeFn = fn(stream: &Stream, event: &Subscribe) -> Result<()>;

/// The callback function prototype for a refresh event callback.
pub type RefreshFn = fn(stream: &Stream, event: &EquityRefresh) -> Result<()>;

/// The callback function prototype for a trade event callback.
pub type TradeFn = fn(stream: &Stream, event: &EquityTrade) -> Result<()>;

/// The callback function prototype for a quote event callback.
pub type QuoteFn = fn(stream: &Stream, event: &EquityQuote) -> Result<()>;

/// The callback function prototype for a cancel event callback.
pub type CancelFn = fn(stream: &Stream, event: &EquityCancel) -> Result<()>;

/// The callback function prototype for a correction eent callback.
pub type CorrectionFn = fn(stream: &Stream, event: &EquityCorrection) -> Result<()>;

/// The callback function prototype for a order imbalance callback.
pub type OrderImbalanceFn = fn(stream: &Stream, event: &OrderImbalance) -> Result<()>;

/// The callback function prototype for a order imbalance callback.
pub type TradingActionFn = fn(stream: &Stream, event: &TradingAction) -> Result<()>;

/// The callback function prototype for a order imbalance callback.
pub type IndicativePriceFn = fn(stream: &Stream, event: &IndicativePrice) -> Result<()>;

/// The callback function prototype for a order imbalance callback.
pub type TradeSummaryFn = fn(stream: &Stream, event: &TradeSummary) -> Result<()>;

/// The callback function prototype for a order imbalance callback.
pub type ExchangeStatisticsFn = fn(stream: &Stream, event: &ExchangeStatistics) -> Result<()>;

/// A builder which can create an equity stream container
#[derive(Debug, Default)]
pub struct Builder {
    subscribe: Option<SubscribeFn>,
    refresh: Option<RefreshFn>,
    trade: Option<TradeFn>,
    quote: Option<QuoteFn>,
    cancel: Option<CancelFn>,
    correction: Option<CorrectionFn>,
    order_imbalance: Option<OrderImbalanceFn>,
    trading_action: Option<TradingActionFn>,
    indicative_price: Option<IndicativePriceFn>,
    trade_summary: Option<TradeSummaryFn>,
    exchange_statistics: Option<ExchangeStatisticsFn>,
}

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
enum Field {
    QuoteRate = rxegy_sys::XFLD_RT_EQSTRM_MAX_QUOTE_RATE,

    Key = rxegy_sys::XFLD_RT_EQSTRM_KEY,
    KeyString = rxegy_sys::XFLD_RT_EQSTRM_KEY_STRING,
    UpdateReceiveHitime = rxegy_sys::XFLD_RT_EQSTRM_UPDATE_RECEIVE_HITIME,
    UpdateXcapiReceiveHitime = rxegy_sys::XFLD_RT_EQSTRM_UPDATE_XCAPI_RECEIVE_HITIME,
    AlternateId1 = rxegy_sys::XFLD_RT_EQSTRM_ALTERNATE_ID1,
    AlternateId2 = rxegy_sys::XFLD_RT_EQSTRM_ALTERNATE_ID2,
    OrderRefIdType = rxegy_sys::XFLD_RT_EQSTRM_ORDER_REF_ID_TYPE,
    PriceType = rxegy_sys::XFLD_RT_EQSTRM_PRICE_TYPE,
    SymbolType = rxegy_sys::XFLD_RT_EQSTRM_SYMBOL_TYPE,
    InstrumentStatus = rxegy_sys::XFLD_RT_EQSTRM_INSTRUMENT_STATUS,
    MarketStatus = rxegy_sys::XFLD_RT_EQSTRM_MARKET_STATUS,
    PrimeExch = rxegy_sys::XFLD_RT_EQSTRM_PRIME_EXCH,
    PrimeCountry = rxegy_sys::XFLD_RT_EQSTRM_PRIME_COUNTRY,
    PrimeTradeVenue = rxegy_sys::XFLD_RT_EQSTRM_PRIME_TRADE_VENUE,
    LotSize = rxegy_sys::XFLD_RT_EQSTRM_LOT_SIZE,
    ShortSaleRestricted = rxegy_sys::XFLD_RT_EQSTRM_SHORT_SALE_RESTRICTED,
}

impl FieldTrait for Field {
    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
