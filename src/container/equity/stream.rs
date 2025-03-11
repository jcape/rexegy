//! Top-of-book Containers for Equity and Equity Options Streams

use crate::{
    AlternateId, FeedId, GroupId, HiTime, InstrumentStatus, Key, MarketStatus, TradeVenue,
    container::{
        RealTime,
        callbacks::{
            EquityStreamCancelFn, EquityStreamCorrectionFn, EquityStreamExchangeStatisticsFn,
            EquityStreamIndicativePriceFn, EquityStreamOrderImbalanceFn, EquityStreamQuoteFn,
            EquityStreamRefreshFn, EquityStreamSubscribeFn, EquityStreamTradeFn,
            EquityStreamTradeSummaryFn, EquityStreamTradingActionFn,
        },
    },
    error::{Result, Success},
    field::{self, Field as FieldTrait},
    impl_wrapper_on_newtype,
    misc::OrderRefIdKind,
    object::{Kind as ObjectKind, Wrapper},
    session::TickerSession,
};
use rxegy_sys::{xerr, xhandle};
use std::{
    any::Any,
    ffi::{CString, c_void},
    ptr::{self, NonNull},
};

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

    /// Retrieve the key for the currently subscribed-to instrument.
    pub fn key(&self, slot: u32) -> Result<Key> {
        field::get_xc_key(self, slot, Field::Key).map(Key::new)
    }

    /// Retreive the key string for the subscribed-to instrument.
    pub fn key_string(&self, slot: u32) -> Result<String> {
        field::get_fixedstring(self, slot, Field::KeyString, 80)
    }

    /// Retreive the Exegy timestamp indicating when the Exegy appliance received the last event for
    /// this instrument.
    pub fn update_recv_hitime(&self, slot: u32) -> Result<HiTime> {
        field::get_u64(self, slot, Field::UpdateReceiveHitime).map(HiTime::from)
    }

    /// Retrieve the timestamp when XCAPI received the last event for this instrument.
    pub fn update_xcapi_recv_hitime(&self, slot: u32) -> Result<HiTime> {
        field::get_u64(self, slot, Field::UpdateXcapiReceiveHitime).map(HiTime::from)
    }

    /// Retrieve the first alternate ID (symbol alias) for the subscribed-to instrument, if one is
    /// currently available.
    ///
    /// As background, note that for some feeds, Exegy provides one or more alternate-identifier
    /// symbol sets. In addition, users may define their own alternate IDs programmatically, if
    /// desired, using the WO-EquityInstrument, WO-CommodityInstrument, or WO-Level2Instrument
    /// objects. If no alternate ID has been defined by the user or pre-loaded by Exegy, this field
    /// is empty.
    ///
    /// See the _Feed Handler Reference Guide_ for detailed information on each feed's symbology,
    /// including any pre-loaded alternate IDs. Note that alternate IDs may be used for XCAPI
    /// subscription requests (as the symbol portion of the request key string). For optimized
    /// response time, subscription requests using alternate IDs should include a namespace token in
    /// the key string.
    pub fn alternate_id1(&self, slot: u32) -> Result<AlternateId> {
        field::get_xc_alternate_id(self, slot, Field::AlternateId1).map(AlternateId::new)
    }

    /// Retrieve the second alternate ID (symbol alias) for the subscribed-to instrument, if one is
    /// currently available.
    ///
    /// As background, note that for some feeds, Exegy provides one or more alternate-identifier
    /// symbol sets. In addition, users may define their own alternate IDs programmatically, if
    /// desired, using the WO-EquityInstrument, WO-CommodityInstrument, or WO-Level2Instrument
    /// objects. If no alternate ID has been defined by the user or pre-loaded by Exegy, this field
    /// is empty.
    ///
    /// See the _Feed Handler Reference Guide_ for detailed information on each feed's symbology,
    /// including any pre-loaded alternate IDs. Note that alternate IDs may be used for XCAPI
    /// subscription requests (as the symbol portion of the request key string). For optimized
    /// response time, subscription requests using alternate IDs should include a namespace token in
    /// the key string.
    pub fn alternate_id2(&self, slot: u32) -> Result<AlternateId> {
        field::get_xc_alternate_id(self, slot, Field::AlternateId2).map(AlternateId::new)
    }

    /// Retrieve the enumeration type that denotes the encoding scheme used for order reference IDs
    /// in this object.
    ///
    /// Possible values include: `xoidt_ascii` (ASCII), `xoidt_bcd` (binary-coded decimal), and
    /// `xoidt_uint_pair` (pair of unsigned integers). Note that the API has built-in conversion
    /// routines for converting these (and other) data formats to human-readable strings. (See
    /// "Conversion Routines" in the left-hand Contents menu.)
    pub fn order_ref_id_type(&self, slot: u32) -> Result<OrderRefIdKind> {
        field::get_u8(self, slot, Field::OrderRefIdType).and_then(OrderRefIdKind::try_from)
    }

    /// Retrieve the price type used for price-related fields in the object.
    pub fn price_type(&self, slot: u32) -> Result<u8> {
        field::get_u8(self, slot, Field::PriceType)
    }

    /// Retrieve the type of instrument for this subscription.
    pub fn symbol_type(&self, slot: u32) -> Result<u8> {
        field::get_u8(self, slot, Field::SymbolType)
    }

    /// Retrieve the normalized trading status/substatus for the instrument.
    ///
    /// See the _Exegy Feature Brief: Market and Instrument Status_ for a detailed discussion of how
    /// Exegy normalizes status indications across markets. Note that for composite, depth-of-book
    /// views, the instrument status for each contributing market is provided as an array.
    pub fn instrument_status(&self, slot: u32) -> Result<InstrumentStatus> {
        field::get_xc_trading_state(self, slot, Field::InstrumentStatus).map(InstrumentStatus::new)
    }

    /// Retrieve the normalized trading status/sub-status for the market or instrument group.
    ///
    /// See the _Exegy Feature Brief: Market and Instrument Status_ for a detailed discussion of how
    /// Exegy normalizes status indications across markets. Note that for composite, depth-of-book
    /// views, the trading status/substatus for each contributing market is provided as an array.
    pub fn market_status(&self, slot: u32) -> Result<MarketStatus> {
        field::get_xc_trading_state(self, slot, Field::MarketStatus).map(MarketStatus::new)
    }

    /// Retrieve the prime "exchange" (i.e., listing exchange feed) for the instrument.
    pub fn prime_feed_id(&self, slot: u32) -> Result<FeedId> {
        field::get_xc_exchange_id(self, slot, Field::PrimeExch).map(FeedId::new)
    }

    /// Retrieve the "country" of the prime exchange (i.e., listing exchange feed) for the
    /// instrument.
    pub fn prime_group_id(&self, slot: u32) -> Result<GroupId> {
        field::get_xc_country_id(self, slot, Field::PrimeCountry).map(GroupId::new)
    }

    /// Retrieve the MIC code of the prime "exchange" for the instrument.
    pub fn primary_trade_venue(&self, slot: u32) -> Result<TradeVenue> {
        field::get_xc_trade_venue(self, slot, Field::PrimeTradeVenue).map(TradeVenue::new)
    }

    /// Retrieve the number of shares in a standard lot for the instrument.
    pub fn lot_size(&self, slot: u32) -> Result<u16> {
        field::get_u16(self, slot, Field::LotSize)
    }

    /// Retrieve whether the short sale restricted status is in effect for the instrument.
    pub fn short_sale_restricted(&self, slot: u32) -> Result<bool> {
        Ok(field::get_u8(self, slot, Field::ShortSaleRestricted)? != 0)
    }

    /// Subscribe to a new instrument by Exegy key string, supplying the given boxed value as a user
    /// data pointer to be supplied to callbacks on this instrument, returning the slot.
    ///
    /// This function wraps [xcRequestItemByString](rxegy_sys::xcRequestItemByString) and exhibits
    /// similar semantics and restrictions.
    pub fn subscribe_by_string(&self, key_string: &str, user_data: Box<dyn Any>) -> Result<u32> {
        let key_string = CString::new(key_string)?;

        let thin_ptr = Box::new(user_data);
        let turnkey = Box::into_raw(thin_ptr) as u64;

        let mut slot = rxegy_sys::XC_NEXT_AVAILABLE_SLOT;

        let status = unsafe {
            rxegy_sys::xcRequestItemByString(
                self.as_xhandle(),
                key_string.as_ptr(),
                turnkey,
                &mut slot,
            )
        };

        Success::try_from(status)?;

        Ok(slot)
    }
}

/// A builder which can create an equity stream container
#[derive(Debug, Default)]
pub struct Builder {
    subscribe: Option<EquityStreamSubscribeFn>,
    refresh: Option<EquityStreamRefreshFn>,
    trade: Option<EquityStreamTradeFn>,
    quote: Option<EquityStreamQuoteFn>,
    cancel: Option<EquityStreamCancelFn>,
    correction: Option<EquityStreamCorrectionFn>,
    order_imbalance: Option<EquityStreamOrderImbalanceFn>,
    trading_action: Option<EquityStreamTradingActionFn>,
    indicative_price: Option<EquityStreamIndicativePriceFn>,
    trade_summary: Option<EquityStreamTradeSummaryFn>,
    exchange_statistics: Option<EquityStreamExchangeStatisticsFn>,
}

impl Builder {
    /// Set the callback to be fired when a subscription event occurs.
    ///
    /// Fires when a subscription or retrieval request is made by the client application. Note that
    /// when requesting a "new" (not previously subscribed to) symbol from a session-based feed such
    /// as Bloomberg, the event fires twice: once with a status of
    /// [ExegyError::Pending](crate::ExegyError::Pending), indicating that the Exegy appliance has
    /// requested the item from the feed's server, and a second time with a status of
    /// [Success::Generic], indicating that the Exegy appliance has obtained data for the desired
    /// instrument and the subscription request is fulfilled. In addition, note that the second
    /// subscribe event may have a status of [ExegyError::Access](crate::ExegyError::Access),
    /// instead of [Success::Generic], indicating that the user is not authorized for the requested data.
    pub fn on_subscribe(mut self, func: EquityStreamSubscribeFn) -> Self {
        self.subscribe = Some(func);
        self
    }

    /// Set the callback to be fired when a refresh event occurs.
    ///
    /// The callback will be fired for each slot currently subscribed to the relevant equity
    /// instrument under a variety of circumstances:
    ///
    /// - For the initial data request made by the client application
    /// - When value-added fields (e.g., the day's high price) are updated by the exchange
    /// - At start of day to populate the initial data image
    /// - During a disconnect/reconnect scenario (as XCAPI automatically repopulates previously
    ///   existing subscriptions).
    pub fn on_refresh(mut self, func: EquityStreamRefreshFn) -> Self {
        self.refresh = Some(func);
        self
    }

    /// Set the callback to be fired when a trade event is received from the appliance.
    ///
    /// The callback will be fired for each slot currently subscribed to the relevant equity
    /// instrument.
    pub fn on_trade(mut self, func: EquityStreamTradeFn) -> Self {
        self.trade = Some(func);
        self
    }

    /// Set the callback to be fired when a quote event is received from the appliance.
    ///
    /// The callback will be fired for each slot currently subscribed to the relevant equity
    /// instrument.
    pub fn on_quote(mut self, func: EquityStreamQuoteFn) -> Self {
        self.quote = Some(func);
        self
    }

    /// Set the callback to be fired when a cancel event is received from the appliance.
    ///
    /// The callback will be fired for each slot currently subscribed to the relevant equity
    /// instrument.
    pub fn on_cancel(mut self, func: EquityStreamCancelFn) -> Self {
        self.cancel = Some(func);
        self
    }

    /// Set the callback to be fired when a correction event is received from the appliance.
    ///
    /// The callback will be fired for each slot currently subscribed to the relevant equity
    /// instrument.
    pub fn on_correction(mut self, func: EquityStreamCorrectionFn) -> Self {
        self.correction = Some(func);
        self
    }

    /// Set the callback to be fired when an order imbalance event is received, and when auction
    /// cycles begin or end.
    ///
    /// The callback will be fired for each slot currently subscribed to the relevant equity
    /// instrument.
    pub fn on_order_imbalance(mut self, func: EquityStreamOrderImbalanceFn) -> Self {
        self.order_imbalance = Some(func);
        self
    }

    /// Sets the callback to be fired when a trading action event is received from the appliance
    /// or for instrument status/substatus transitions (including transitions between auction
    /// states).
    ///
    /// The callback will be fired for each slot currently subscribed to the relevant equity
    /// instrument.
    pub fn on_trading_action(mut self, func: EquityStreamTradingActionFn) -> Self {
        self.trading_action = Some(func);
        self
    }

    /// Sets the callback to be fired when a market provides a preliminary or transient indication
    /// of an opening, closing, or settlement price.
    ///
    /// The [IndicativePrice::indicative_price_kind] method will  indicates which type of price
    /// (opening, closing, or settlement) is being provided.
    pub fn on_indicative_price(mut self, func: EquityStreamIndicativePriceFn) -> Self {
        self.indicative_price = Some(func);
        self
    }

    /// Sets the callback to be fired when a trade summary event occurs.
    pub fn on_trade_summary(mut self, func: EquityStreamTradeSummaryFn) -> Self {
        self.trade_summary = Some(func);
        self
    }

    /// Sets the callback to be fired when the appliance conveys statistics information provided
    /// directly by the feed, if any.
    ///
    /// This information may include VWAP, trade count, turnover, and short sale statistics when
    /// these are provided by the feed.
    pub fn on_exchange_statistics(mut self, func: EquityStreamExchangeStatisticsFn) -> Self {
        self.exchange_statistics = Some(func);
        self
    }

    /// Build a new session.
    pub fn build(self, session: &TickerSession, max_slots: u32) -> Result<Stream> {
        let mut object = ptr::null_mut();

        let context = Box::new(Box::new(self) as Box<dyn Any>);
        let turnkey = Box::into_raw(context) as u64;

        let status = unsafe {
            rxegy_sys::xcCreateContainer(
                session.as_xhandle(),
                <Stream as Wrapper>::KIND as u16,
                &mut object,
                Some(_rxegy_equity_stream_callback),
                turnkey,
                max_slots,
            )
        };

        Success::try_from(status)?;

        Stream::from_xhandle(object)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _rxegy_equity_stream_callback(
    _handle: xhandle,
    _slot: u32,
    _event_handle: xhandle,
    _event_type: u16,
    _turnkey: u64,
    _status: xerr,
) {
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
