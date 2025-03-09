//! Exegy object support

use crate::error::{Error, Result, Success};
use rxegy_sys::xhandle;
use std::{ffi::c_void, ptr::NonNull};

/// An enumeration of Exegy object types
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
#[non_exhaustive]
pub enum Kind {
    /// invalid
    Invalid = rxegy_sys::XOBJ_INVALID,
    /// ticker plant -- standard session
    SessionTicker = rxegy_sys::XOBJ_SESSION_TICKER,
    /// ticker plant monitoring session
    SessionTickerMonitoring = rxegy_sys::XOBJ_SESSION_TICKER_MONITORING,
    /// price summary objects
    RealtimeEquitySummary = rxegy_sys::XOBJ_REALTIME_EQUITY_SUMMARY,
    /// exchange-based subscriptions
    RealtimeExchangeStream = rxegy_sys::XOBJ_REALTIME_EXCHANGE_STREAM,
    /// order book summary objects
    RealtimeOrderBookSummary = rxegy_sys::XOBJ_REALTIME_ORDER_BOOK_SUMMARY,
    /// order book exchange-based subscriptions
    RealtimeOrderBookExchangeStream = rxegy_sys::XOBJ_REALTIME_ORDER_BOOK_EXCHANGE_STREAM,
    /// price book summary objects
    RealtimePriceBookSummary = rxegy_sys::XOBJ_REALTIME_PRICE_BOOK_SUMMARY,
    /// price book exchange-based subscriptions
    RealtimePriceBookExchangeStream = rxegy_sys::XOBJ_REALTIME_PRICE_BOOK_EXCHANGE_STREAM,
    /// basket summary objects
    RealtimeBasketSummary = rxegy_sys::XOBJ_REALTIME_BASKET_SUMMARY,
    /// list of all baskets
    RealtimeBasketCatalog = rxegy_sys::XOBJ_REALTIME_BASKET_CATALOG,
    /// filtered keylist results
    RealtimeKeylistFilter = rxegy_sys::XOBJ_REALTIME_KEYLIST_FILTER,
    /// catalog of keylists
    RealtimeKeylistCatalog = rxegy_sys::XOBJ_REALTIME_KEYLIST_CATALOG,
    /// commodity price summary objects
    RealtimeCommoditySummary = rxegy_sys::XOBJ_REALTIME_COMMODITY_SUMMARY,
    /// equity stream objects
    RealtimeEquityStream = rxegy_sys::XOBJ_REALTIME_EQUITY_STREAM,
    /// commodity stream objects
    RealtimeCommodityStream = rxegy_sys::XOBJ_REALTIME_COMMODITY_STREAM,
    /// order book stream objects
    RealtimeOrderBookStream = rxegy_sys::XOBJ_REALTIME_ORDER_BOOK_STREAM,
    /// price book stream objects
    RealtimePriceBookStream = rxegy_sys::XOBJ_REALTIME_PRICE_BOOK_STREAM,
    /// Ticker Plant feed information
    RealtimeTickerPlantFeeds = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_FEEDS,
    /// Ticker Plant line information
    RealtimeTickerPlantLines = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_LINES,
    /// Ticker Plant socket information
    RealtimeTickerPlantSockets = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SOCKETS,
    /// Ticker Plant client connection information
    RealtimeTickerPlantClients = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_CLIENTS,
    /// Derivative Summary
    RealtimeDerivativeReferenceSummary = rxegy_sys::XOBJ_REALTIME_DERIVATIVE_REFERENCE_SUMMARY,
    /// Derivative exchange stream
    RealtimeDerivativeReferenceExchangeStream =
        rxegy_sys::XOBJ_REALTIME_DERIVATIVE_REFERENCE_EXCHANGE_STREAM,
    /// Foreign Exchange Sopt Stream
    RealtimeFxspotStream = rxegy_sys::XOBJ_REALTIME_FXSPOT_STREAM,
    /// Foreign Exchange Forward Stream
    RealtimeFxforwardStream = rxegy_sys::XOBJ_REALTIME_FXFORWARD_STREAM,
    /// Foreign Exchange SWAP Stream
    RealtimeFxswapStream = rxegy_sys::XOBJ_REALTIME_FXSWAP_STREAM,
    /// Ticker Plant summary
    RealtimeTickerPlantSummary = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SUMMARY,
    /// Ticker Plant latency
    RealtimeTickerPlantLatency = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_LATENCY,
    /// Ticker Plant clients rates
    RealtimeTickerPlantClientsRates = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_CLIENTS_RATES,
    /// Ticker Plant summary rates
    RealtimeTickerPlantSummaryRates = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SUMMARY_RATES,
    /// Ticker Plant sessions (for session sets)
    RealtimeTickerPlantSessions = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SESSIONS,
    /// Ticker Plant sessions rates (for session sets)
    RealtimeTickerPlantSessionsRates = rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SESSIONS_RATES,
    /// Instrument Group Summary
    RealtimeInstrumentGroupSummary = rxegy_sys::XOBJ_REALTIME_INSTRUMENT_GROUP_SUMMARY,
    /// FX summary objects
    RealtimeFxSummary = rxegy_sys::XOBJ_REALTIME_FX_SUMMARY,
    /// FX stream objects
    RealtimeFxStream = rxegy_sys::XOBJ_REALTIME_FX_STREAM,
    /// Volatility summary objects
    RealtimeVolatilitySummary = rxegy_sys::XOBJ_REALTIME_VOLATILITY_SUMMARY,
    /// Symbol reference summary objects
    RealtimeSymbolReferenceSummary = rxegy_sys::XOBJ_REALTIME_SYMBOL_REFERENCE_SUMMARY,
    /// Equity instrument update
    WriteonlyEquityInstrument = rxegy_sys::XOBJ_WRITEONLY_EQUITY_INSTRUMENT,
    /// Commodity instrument update
    WriteonlyCommodityInstrument = rxegy_sys::XOBJ_WRITEONLY_COMMODITY_INSTRUMENT,
    /// Level 2 instrument update
    WriteonlyLevel2Instrument = rxegy_sys::XOBJ_WRITEONLY_LEVEL2_INSTRUMENT,
    /// Keylist edit
    WriteonlyKeylistEdit = rxegy_sys::XOBJ_WRITEONLY_KEYLIST_EDIT,
    /// Subscribe to instrument
    EventSubscribe = rxegy_sys::XOBJ_EVENT_SUBSCRIBE,
    /// Snapshot has been completed
    EventSnapshot = rxegy_sys::XOBJ_EVENT_SNAPSHOT,
    /// Equity trade event
    EventEquityTrade = rxegy_sys::XOBJ_EVENT_EQUITY_TRADE,
    /// Equity quote event
    EventEquityQuote = rxegy_sys::XOBJ_EVENT_EQUITY_QUOTE,
    /// Equity refresh event
    EventEquityRefresh = rxegy_sys::XOBJ_EVENT_EQUITY_REFRESH,
    /// Equity cancellation event
    EventEquityCancel = rxegy_sys::XOBJ_EVENT_EQUITY_CANCEL,
    /// Equity correction event
    EventEquityCorrection = rxegy_sys::XOBJ_EVENT_EQUITY_CORRECTION,
    /// Order book refresh on bid side (equities)
    EventOrderBookRefreshBid = rxegy_sys::XOBJ_EVENT_ORDER_BOOK_REFRESH_BID,
    /// Order book refresh on ask side (equities)
    EventOrderBookRefreshAsk = rxegy_sys::XOBJ_EVENT_ORDER_BOOK_REFRESH_ASK,
    /// Order book update (equities)
    EventOrderBookUpdate = rxegy_sys::XOBJ_EVENT_ORDER_BOOK_UPDATE,
    /// Price book refresh on bid side (equities)
    EventPriceBookRefreshBid = rxegy_sys::XOBJ_EVENT_PRICE_BOOK_REFRESH_BID,
    /// Price book refresh on ask side (equities)
    EventPriceBookRefreshAsk = rxegy_sys::XOBJ_EVENT_PRICE_BOOK_REFRESH_ASK,
    /// Price book update (equities)
    EventPriceBookUpdate = rxegy_sys::XOBJ_EVENT_PRICE_BOOK_UPDATE,
    /// Commodity refresh event
    EventCommodityRefresh = rxegy_sys::XOBJ_EVENT_COMMODITY_REFRESH,
    /// Commodity trade event
    EventCommodityTrade = rxegy_sys::XOBJ_EVENT_COMMODITY_TRADE,
    /// Commodity quote event
    EventCommodityQuote = rxegy_sys::XOBJ_EVENT_COMMODITY_QUOTE,
    /// Commodity cancel event
    EventCommodityCancel = rxegy_sys::XOBJ_EVENT_COMMODITY_CANCEL,
    /// Commodity correction event
    EventCommodityCorrection = rxegy_sys::XOBJ_EVENT_COMMODITY_CORRECTION,
    /// Basket NAV update
    EventNavUpdate = rxegy_sys::XOBJ_EVENT_NAV_UPDATE,
    /// Basket catalog refresh
    EventBasketCatalogRefresh = rxegy_sys::XOBJ_EVENT_BASKET_CATALOG_REFRESH,
    /// Basket catalog update
    EventBasketCatalogUpdate = rxegy_sys::XOBJ_EVENT_BASKET_CATALOG_UPDATE,
    /// Basket definition refresh
    EventBasketDefnRefresh = rxegy_sys::XOBJ_EVENT_BASKET_DEFN_REFRESH,
    /// Basket set/remove definition status
    EventBasketDefnStatus = rxegy_sys::XOBJ_EVENT_BASKET_DEFN_STATUS,
    /// Status of basket constituent update operation
    EventBasketConstituentUpdateStatus = rxegy_sys::XOBJ_EVENT_BASKET_CONSTITUENT_UPDATE_STATUS,
    /// Global basket constituent information
    EventBasketConstituentRefresh = rxegy_sys::XOBJ_EVENT_BASKET_CONSTITUENT_REFRESH,
    /// A one-time response (equities)
    EventStaticEquityRefresh = rxegy_sys::XOBJ_EVENT_STATIC_EQUITY_REFRESH,
    /// A one-time response (commodities)
    EventStaticCommodityRefresh = rxegy_sys::XOBJ_EVENT_STATIC_COMMODITY_REFRESH,
    /// Basket definition deleted
    EventBasketDefnDelete = rxegy_sys::XOBJ_EVENT_BASKET_DEFN_DELETE,
    /// Keylist definition status
    EventKeylistDefnStatus = rxegy_sys::XOBJ_EVENT_KEYLIST_DEFN_STATUS,
    /// Static keylist definition refresh
    EventKeylistDefnRefresh = rxegy_sys::XOBJ_EVENT_KEYLIST_DEFN_REFRESH,
    /// Static keylist definition deleted
    EventKeylistDefnDelete = rxegy_sys::XOBJ_EVENT_KEYLIST_DEFN_DELETE,
    /// Filtered keylist definition status
    EventKeylistFilterDefnStatus = rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_STATUS,
    /// Filtered keylist definition refresh
    EventKeylistFilterDefnRefresh = rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_REFRESH,
    /// Filtered keylist definition deleted
    EventKeylistFilterDefnDelete = rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_DELETE,
    /// Results of a filtered keylist subscription have begun
    EventKeylistFilterMatchStart = rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_START,
    /// Results of a filtered keylist subscription have ended
    EventKeylistFilterMatchEnd = rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_END,
    /// Results of a filtered keylist subscription
    EventKeylistFilterMatch = rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH,
    /// A key has been deleted that matched a filtered keylist subscription
    EventKeylistFilterMatchRemove = rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_REMOVE,
    /// Keylist catalog refresh
    EventKeylistCatalogRefresh = rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH,
    /// Keylist catalog update
    EventKeylistCatalogUpdate = rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE,
    /// Status of instrument update operation
    EventInstrumentUpdateStatus = rxegy_sys::XOBJ_EVENT_INSTRUMENT_UPDATE_STATUS,
    /// Signum predictive data refresh
    EventSignumPredictiveDataRefresh = rxegy_sys::XOBJ_EVENT_SIGNUM_PREDICTIVE_DATA_REFRESH,
    /// Signum predictive data status
    EventSignumPredictiveDataStatus = rxegy_sys::XOBJ_EVENT_SIGNUM_PREDICTIVE_DATA_STATUS,
    /// Order execution event
    EventOrderExecution = rxegy_sys::XOBJ_EVENT_ORDER_EXECUTION,
    /// Order imbalance notification
    EventOrderImbalance = rxegy_sys::XOBJ_EVENT_ORDER_IMBALANCE,
    /// Market directory event
    EventMarketDirectory = rxegy_sys::XOBJ_EVENT_MARKET_DIRECTORY,
    /// Trading action event
    EventTradingAction = rxegy_sys::XOBJ_EVENT_TRADING_ACTION,
    /// Refresh Ticker Plant feed information
    EventTickerPlantFeedRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_FEED_REFRESH,
    /// Update Ticker Plant feed information
    EventTickerPlantFeedUpdate = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_FEED_UPDATE,
    /// Refresh Ticker Plant line information
    EventTickerPlantLineRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_LINE_REFRESH,
    /// Refresh Ticker Plant socket information
    EventTickerPlantSocketRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SOCKET_REFRESH,
    /// Refresh Ticker Plant client connection information
    EventTickerPlantClientRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_CLIENT_REFRESH,
    /// Refresh Ticker Plant line information
    EventStaticTickerPlantLineRefresh = rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LINE_REFRESH,
    /// Refresh Ticker Plant socket information
    EventStaticTickerPlantSocketRefresh = rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SOCKET_REFRESH,
    /// Refresh Ticker Plant client connection information
    EventStaticTickerPlantClientRefresh = rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_CLIENT_REFRESH,
    /// Ticker Plant line gap list
    EventStaticTickerPlantLineGapsRefresh =
        rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LINE_GAPS_REFRESH,
    /// Derivative reference refresh event
    EventDerivativeReferenceRefresh = rxegy_sys::XOBJ_EVENT_DERIVATIVE_REFERENCE_REFRESH,
    /// Order on book event
    EventOrderOnBook = rxegy_sys::XOBJ_EVENT_ORDER_ON_BOOK,
    /// Foreign exchange spot refresh event
    EventFxspotRefresh = rxegy_sys::XOBJ_EVENT_FXSPOT_REFRESH,
    /// Foreign exchange spot quote event
    EventFxspotQuote = rxegy_sys::XOBJ_EVENT_FXSPOT_QUOTE,
    /// Foreign exchange forward refresh event
    EventFxfwdRefresh = rxegy_sys::XOBJ_EVENT_FXFWD_REFRESH,
    /// Foreign exchange forward quote event
    EventFxfwdQuote = rxegy_sys::XOBJ_EVENT_FXFWD_QUOTE,
    /// Foreign exchange swap refresh event
    EventFxswapRefresh = rxegy_sys::XOBJ_EVENT_FXSWAP_REFRESH,
    /// Foreign exchange swap quote event
    EventFxswapQuote = rxegy_sys::XOBJ_EVENT_FXSWAP_QUOTE,
    /// Ticker Plant summary refresh
    EventTickerPlantSummaryRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SUMMARY_REFRESH,
    /// Ticker Plant latency refresh
    EventTickerPlantLatencyRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_LATENCY_REFRESH,
    /// Ticker Plant clients rates refresh
    EventTickerPlantClientsRatesRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_CLIENTS_RATES_REFRESH,
    /// Ticker Plant summary rates refresh
    EventTickerPlantSummaryRatesRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SUMMARY_RATES_REFRESH,
    /// Ticker Plant summary refresh
    EventStaticTickerPlantSummaryRefresh =
        rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SUMMARY_REFRESH,
    /// Ticker Plant latency refresh
    EventStaticTickerPlantLatencyRefresh =
        rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LATENCY_REFRESH,
    /// Ticker Plant clients rates refresh
    EventStaticTickerPlantClientsRatesRefresh =
        rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_CLIENTS_RATES_REFRESH,
    /// Ticker Plant summary rates refresh
    EventStaticTickerPlantSummaryRatesRefresh =
        rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SUMMARY_RATES_REFRESH,
    /// Ticker Plant market wide circuit breaker update
    EventTickerPlantMwcbUpdate = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_MWCB_UPDATE,
    /// Refresh Ticker Plant sessions rates refresh
    EventTickerPlantSessionsRatesRefresh =
        rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SESSIONS_RATES_REFRESH,
    /// Refresh Ticker Plant sessions rates refresh
    EventStaticTickerPlantSessionsRatesRefresh =
        rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SESSIONS_RATES_REFRESH,
    /// Refresh Ticker Plant sessions stats refresh
    EventTickerPlantSessionsRefresh = rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SESSIONS_REFRESH,
    /// Refresh Ticker Plant sessions stats refresh
    EventStaticTickerPlantSessionsRefresh =
        rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SESSIONS_REFRESH,
    /// Request for quote event
    EventRequestForQuote = rxegy_sys::XOBJ_EVENT_REQUEST_FOR_QUOTE,
    /// Indicative Price event
    EventIndicativePrice = rxegy_sys::XOBJ_EVENT_INDICATIVE_PRICE,
    /// Instrument Group Refresh
    EventInstrumentGroupRefresh = rxegy_sys::XOBJ_EVENT_INSTRUMENT_GROUP_REFRESH,
    /// Instrument Group Refresh
    EventInstrumentGroupUpdate = rxegy_sys::XOBJ_EVENT_INSTRUMENT_GROUP_UPDATE,
    /// Session status has changed (e.g. connected or disconnected)
    EventSessionStatus = rxegy_sys::XOBJ_EVENT_SESSION_STATUS,
    /// FX refresh on bid side
    EventFxRefresh = rxegy_sys::XOBJ_EVENT_FX_REFRESH,
    /// FX update
    EventFxUpdate = rxegy_sys::XOBJ_EVENT_FX_UPDATE,
    /// Volatility refresh
    EventVolatilityRefresh = rxegy_sys::XOBJ_EVENT_VOLATILITY_REFRESH,
    /// Volatility quote risk
    EventVolatilityQuoteRisk = rxegy_sys::XOBJ_EVENT_VOLATILITY_QUOTE_RISK,
    /// Volatility trade risk
    EventVolatilityTradeRisk = rxegy_sys::XOBJ_EVENT_VOLATILITY_TRADE_RISK,
    /// Symbol reference refresh
    EventSymbolReferenceRefresh = rxegy_sys::XOBJ_EVENT_SYMBOL_REFERENCE_REFRESH,
    /// Signum signal status
    EventSignalStatus = rxegy_sys::XOBJ_EVENT_SIGNAL_STATUS,
    /// Trade summary
    EventTradeSummary = rxegy_sys::XOBJ_EVENT_TRADE_SUMMARY,
    /// Exchange statistics
    EventExchangeStatistics = rxegy_sys::XOBJ_EVENT_EXCHANGE_STATISTICS,
    /// Basket definition
    ReadwriteBasketDefinition = rxegy_sys::XOBJ_READWRITE_BASKET_DEFINITION,
    /// Static keylist definition
    ReadwriteKeylistDefinition = rxegy_sys::XOBJ_READWRITE_KEYLIST_DEFINITION,
    /// Filtered keylist definition
    ReadwriteKeylistFilterDefinition = rxegy_sys::XOBJ_READWRITE_KEYLIST_FILTER_DEFINITION,
    /// global basket constituent data
    ReadwriteBasketConstituent = rxegy_sys::XOBJ_READWRITE_BASKET_CONSTITUENT,
    /// Signum predictive data
    ReadwriteSignumPredictiveData = rxegy_sys::XOBJ_READWRITE_SIGNUM_PREDICTIVE_DATA,
    /// Equity summary
    StaticEquitySummary = rxegy_sys::XOBJ_STATIC_EQUITY_SUMMARY,
    /// Commodity summary
    StaticCommoditySummary = rxegy_sys::XOBJ_STATIC_COMMODITY_SUMMARY,
    /// Ticker Plant line information
    StaticTickerPlantLines = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_LINES,
    /// Ticker Plant socket information
    StaticTickerPlantSockets = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SOCKETS,
    /// Ticker Plant client connection information
    StaticTickerPlantClients = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_CLIENTS,
    /// Ticker Plant line gap list
    StaticTickerPlantLineGaps = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_LINE_GAPS,
    /// Ticker Plant summary
    StaticTickerPlantSummary = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SUMMARY,
    /// Ticker Plant latency
    StaticTickerPlantLatency = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_LATENCY,
    /// Ticker Plant clients rates
    StaticTickerPlantClientsRates = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_CLIENTS_RATES,
    /// Ticker Plant summary rates
    StaticTickerPlantSummaryRates = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SUMMARY_RATES,
    /// Ticker Plant sessions (for session sets)
    StaticTickerPlantSessions = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SESSIONS,
    /// Ticker Plant sessions rates (for session sets)
    StaticTickerPlantSessionsRates = rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SESSIONS_RATES,
}

impl From<u16> for Kind {
    fn from(value: u16) -> Self {
        match value {
            rxegy_sys::XOBJ_INVALID => Kind::Invalid,
            rxegy_sys::XOBJ_SESSION_TICKER => Kind::SessionTicker,
            rxegy_sys::XOBJ_SESSION_TICKER_MONITORING => Kind::SessionTickerMonitoring,
            rxegy_sys::XOBJ_REALTIME_EQUITY_SUMMARY => Kind::RealtimeEquitySummary,
            rxegy_sys::XOBJ_REALTIME_EXCHANGE_STREAM => Kind::RealtimeExchangeStream,
            rxegy_sys::XOBJ_REALTIME_ORDER_BOOK_SUMMARY => Kind::RealtimeOrderBookSummary,
            rxegy_sys::XOBJ_REALTIME_ORDER_BOOK_EXCHANGE_STREAM => {
                Kind::RealtimeOrderBookExchangeStream
            }
            rxegy_sys::XOBJ_REALTIME_PRICE_BOOK_SUMMARY => Kind::RealtimePriceBookSummary,
            rxegy_sys::XOBJ_REALTIME_PRICE_BOOK_EXCHANGE_STREAM => {
                Kind::RealtimePriceBookExchangeStream
            }
            rxegy_sys::XOBJ_REALTIME_BASKET_SUMMARY => Kind::RealtimeBasketSummary,
            rxegy_sys::XOBJ_REALTIME_BASKET_CATALOG => Kind::RealtimeBasketCatalog,
            rxegy_sys::XOBJ_REALTIME_KEYLIST_FILTER => Kind::RealtimeKeylistFilter,
            rxegy_sys::XOBJ_REALTIME_KEYLIST_CATALOG => Kind::RealtimeKeylistCatalog,
            rxegy_sys::XOBJ_REALTIME_COMMODITY_SUMMARY => Kind::RealtimeCommoditySummary,
            rxegy_sys::XOBJ_REALTIME_EQUITY_STREAM => Kind::RealtimeEquityStream,
            rxegy_sys::XOBJ_REALTIME_COMMODITY_STREAM => Kind::RealtimeCommodityStream,
            rxegy_sys::XOBJ_REALTIME_ORDER_BOOK_STREAM => Kind::RealtimeOrderBookStream,
            rxegy_sys::XOBJ_REALTIME_PRICE_BOOK_STREAM => Kind::RealtimePriceBookStream,
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_FEEDS => Kind::RealtimeTickerPlantFeeds,
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_LINES => Kind::RealtimeTickerPlantLines,
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SOCKETS => Kind::RealtimeTickerPlantSockets,
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_CLIENTS => Kind::RealtimeTickerPlantClients,
            rxegy_sys::XOBJ_REALTIME_DERIVATIVE_REFERENCE_SUMMARY => {
                Kind::RealtimeDerivativeReferenceSummary
            }
            rxegy_sys::XOBJ_REALTIME_DERIVATIVE_REFERENCE_EXCHANGE_STREAM => {
                Kind::RealtimeDerivativeReferenceExchangeStream
            }
            rxegy_sys::XOBJ_REALTIME_FXSPOT_STREAM => Kind::RealtimeFxspotStream,
            rxegy_sys::XOBJ_REALTIME_FXFORWARD_STREAM => Kind::RealtimeFxforwardStream,
            rxegy_sys::XOBJ_REALTIME_FXSWAP_STREAM => Kind::RealtimeFxswapStream,
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SUMMARY => Kind::RealtimeTickerPlantSummary,
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_LATENCY => Kind::RealtimeTickerPlantLatency,
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_CLIENTS_RATES => {
                Kind::RealtimeTickerPlantClientsRates
            }
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SUMMARY_RATES => {
                Kind::RealtimeTickerPlantSummaryRates
            }
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SESSIONS => Kind::RealtimeTickerPlantSessions,
            rxegy_sys::XOBJ_REALTIME_TICKER_PLANT_SESSIONS_RATES => {
                Kind::RealtimeTickerPlantSessionsRates
            }
            rxegy_sys::XOBJ_REALTIME_INSTRUMENT_GROUP_SUMMARY => {
                Kind::RealtimeInstrumentGroupSummary
            }
            rxegy_sys::XOBJ_REALTIME_FX_SUMMARY => Kind::RealtimeFxSummary,
            rxegy_sys::XOBJ_REALTIME_FX_STREAM => Kind::RealtimeFxStream,
            rxegy_sys::XOBJ_REALTIME_VOLATILITY_SUMMARY => Kind::RealtimeVolatilitySummary,
            rxegy_sys::XOBJ_REALTIME_SYMBOL_REFERENCE_SUMMARY => {
                Kind::RealtimeSymbolReferenceSummary
            }
            rxegy_sys::XOBJ_WRITEONLY_EQUITY_INSTRUMENT => Kind::WriteonlyEquityInstrument,
            rxegy_sys::XOBJ_WRITEONLY_COMMODITY_INSTRUMENT => Kind::WriteonlyCommodityInstrument,
            rxegy_sys::XOBJ_WRITEONLY_LEVEL2_INSTRUMENT => Kind::WriteonlyLevel2Instrument,
            rxegy_sys::XOBJ_WRITEONLY_KEYLIST_EDIT => Kind::WriteonlyKeylistEdit,
            rxegy_sys::XOBJ_EVENT_SUBSCRIBE => Kind::EventSubscribe,
            rxegy_sys::XOBJ_EVENT_SNAPSHOT => Kind::EventSnapshot,
            rxegy_sys::XOBJ_EVENT_EQUITY_TRADE => Kind::EventEquityTrade,
            rxegy_sys::XOBJ_EVENT_EQUITY_QUOTE => Kind::EventEquityQuote,
            rxegy_sys::XOBJ_EVENT_EQUITY_REFRESH => Kind::EventEquityRefresh,
            rxegy_sys::XOBJ_EVENT_EQUITY_CANCEL => Kind::EventEquityCancel,
            rxegy_sys::XOBJ_EVENT_EQUITY_CORRECTION => Kind::EventEquityCorrection,
            rxegy_sys::XOBJ_EVENT_ORDER_BOOK_REFRESH_BID => Kind::EventOrderBookRefreshBid,
            rxegy_sys::XOBJ_EVENT_ORDER_BOOK_REFRESH_ASK => Kind::EventOrderBookRefreshAsk,
            rxegy_sys::XOBJ_EVENT_ORDER_BOOK_UPDATE => Kind::EventOrderBookUpdate,
            rxegy_sys::XOBJ_EVENT_PRICE_BOOK_REFRESH_BID => Kind::EventPriceBookRefreshBid,
            rxegy_sys::XOBJ_EVENT_PRICE_BOOK_REFRESH_ASK => Kind::EventPriceBookRefreshAsk,
            rxegy_sys::XOBJ_EVENT_PRICE_BOOK_UPDATE => Kind::EventPriceBookUpdate,
            rxegy_sys::XOBJ_EVENT_COMMODITY_REFRESH => Kind::EventCommodityRefresh,
            rxegy_sys::XOBJ_EVENT_COMMODITY_TRADE => Kind::EventCommodityTrade,
            rxegy_sys::XOBJ_EVENT_COMMODITY_QUOTE => Kind::EventCommodityQuote,
            rxegy_sys::XOBJ_EVENT_COMMODITY_CANCEL => Kind::EventCommodityCancel,
            rxegy_sys::XOBJ_EVENT_COMMODITY_CORRECTION => Kind::EventCommodityCorrection,
            rxegy_sys::XOBJ_EVENT_NAV_UPDATE => Kind::EventNavUpdate,
            rxegy_sys::XOBJ_EVENT_BASKET_CATALOG_REFRESH => Kind::EventBasketCatalogRefresh,
            rxegy_sys::XOBJ_EVENT_BASKET_CATALOG_UPDATE => Kind::EventBasketCatalogUpdate,
            rxegy_sys::XOBJ_EVENT_BASKET_DEFN_REFRESH => Kind::EventBasketDefnRefresh,
            rxegy_sys::XOBJ_EVENT_BASKET_DEFN_STATUS => Kind::EventBasketDefnStatus,
            rxegy_sys::XOBJ_EVENT_BASKET_CONSTITUENT_UPDATE_STATUS => {
                Kind::EventBasketConstituentUpdateStatus
            }
            rxegy_sys::XOBJ_EVENT_BASKET_CONSTITUENT_REFRESH => Kind::EventBasketConstituentRefresh,
            rxegy_sys::XOBJ_EVENT_STATIC_EQUITY_REFRESH => Kind::EventStaticEquityRefresh,
            rxegy_sys::XOBJ_EVENT_STATIC_COMMODITY_REFRESH => Kind::EventStaticCommodityRefresh,
            rxegy_sys::XOBJ_EVENT_BASKET_DEFN_DELETE => Kind::EventBasketDefnDelete,
            rxegy_sys::XOBJ_EVENT_KEYLIST_DEFN_STATUS => Kind::EventKeylistDefnStatus,
            rxegy_sys::XOBJ_EVENT_KEYLIST_DEFN_REFRESH => Kind::EventKeylistDefnRefresh,
            rxegy_sys::XOBJ_EVENT_KEYLIST_DEFN_DELETE => Kind::EventKeylistDefnDelete,
            rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_STATUS => Kind::EventKeylistFilterDefnStatus,
            rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_REFRESH => {
                Kind::EventKeylistFilterDefnRefresh
            }
            rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_DELETE => Kind::EventKeylistFilterDefnDelete,
            rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_START => Kind::EventKeylistFilterMatchStart,
            rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_END => Kind::EventKeylistFilterMatchEnd,
            rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH => Kind::EventKeylistFilterMatch,
            rxegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_REMOVE => {
                Kind::EventKeylistFilterMatchRemove
            }
            rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH => Kind::EventKeylistCatalogRefresh,
            rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE => Kind::EventKeylistCatalogUpdate,
            rxegy_sys::XOBJ_EVENT_INSTRUMENT_UPDATE_STATUS => Kind::EventInstrumentUpdateStatus,
            rxegy_sys::XOBJ_EVENT_SIGNUM_PREDICTIVE_DATA_REFRESH => {
                Kind::EventSignumPredictiveDataRefresh
            }
            rxegy_sys::XOBJ_EVENT_SIGNUM_PREDICTIVE_DATA_STATUS => {
                Kind::EventSignumPredictiveDataStatus
            }
            rxegy_sys::XOBJ_EVENT_ORDER_EXECUTION => Kind::EventOrderExecution,
            rxegy_sys::XOBJ_EVENT_ORDER_IMBALANCE => Kind::EventOrderImbalance,
            rxegy_sys::XOBJ_EVENT_MARKET_DIRECTORY => Kind::EventMarketDirectory,
            rxegy_sys::XOBJ_EVENT_TRADING_ACTION => Kind::EventTradingAction,
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_FEED_REFRESH => Kind::EventTickerPlantFeedRefresh,
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_FEED_UPDATE => Kind::EventTickerPlantFeedUpdate,
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_LINE_REFRESH => Kind::EventTickerPlantLineRefresh,
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SOCKET_REFRESH => {
                Kind::EventTickerPlantSocketRefresh
            }
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_CLIENT_REFRESH => {
                Kind::EventTickerPlantClientRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LINE_REFRESH => {
                Kind::EventStaticTickerPlantLineRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SOCKET_REFRESH => {
                Kind::EventStaticTickerPlantSocketRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_CLIENT_REFRESH => {
                Kind::EventStaticTickerPlantClientRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LINE_GAPS_REFRESH => {
                Kind::EventStaticTickerPlantLineGapsRefresh
            }
            rxegy_sys::XOBJ_EVENT_DERIVATIVE_REFERENCE_REFRESH => {
                Kind::EventDerivativeReferenceRefresh
            }
            rxegy_sys::XOBJ_EVENT_ORDER_ON_BOOK => Kind::EventOrderOnBook,
            rxegy_sys::XOBJ_EVENT_FXSPOT_REFRESH => Kind::EventFxspotRefresh,
            rxegy_sys::XOBJ_EVENT_FXSPOT_QUOTE => Kind::EventFxspotQuote,
            rxegy_sys::XOBJ_EVENT_FXFWD_REFRESH => Kind::EventFxfwdRefresh,
            rxegy_sys::XOBJ_EVENT_FXFWD_QUOTE => Kind::EventFxfwdQuote,
            rxegy_sys::XOBJ_EVENT_FXSWAP_REFRESH => Kind::EventFxswapRefresh,
            rxegy_sys::XOBJ_EVENT_FXSWAP_QUOTE => Kind::EventFxswapQuote,
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SUMMARY_REFRESH => {
                Kind::EventTickerPlantSummaryRefresh
            }
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_LATENCY_REFRESH => {
                Kind::EventTickerPlantLatencyRefresh
            }
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_CLIENTS_RATES_REFRESH => {
                Kind::EventTickerPlantClientsRatesRefresh
            }
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SUMMARY_RATES_REFRESH => {
                Kind::EventTickerPlantSummaryRatesRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SUMMARY_REFRESH => {
                Kind::EventStaticTickerPlantSummaryRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LATENCY_REFRESH => {
                Kind::EventStaticTickerPlantLatencyRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_CLIENTS_RATES_REFRESH => {
                Kind::EventStaticTickerPlantClientsRatesRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SUMMARY_RATES_REFRESH => {
                Kind::EventStaticTickerPlantSummaryRatesRefresh
            }
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_MWCB_UPDATE => Kind::EventTickerPlantMwcbUpdate,
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SESSIONS_RATES_REFRESH => {
                Kind::EventTickerPlantSessionsRatesRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SESSIONS_RATES_REFRESH => {
                Kind::EventStaticTickerPlantSessionsRatesRefresh
            }
            rxegy_sys::XOBJ_EVENT_TICKER_PLANT_SESSIONS_REFRESH => {
                Kind::EventTickerPlantSessionsRefresh
            }
            rxegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SESSIONS_REFRESH => {
                Kind::EventStaticTickerPlantSessionsRefresh
            }
            rxegy_sys::XOBJ_EVENT_REQUEST_FOR_QUOTE => Kind::EventRequestForQuote,
            rxegy_sys::XOBJ_EVENT_INDICATIVE_PRICE => Kind::EventIndicativePrice,
            rxegy_sys::XOBJ_EVENT_INSTRUMENT_GROUP_REFRESH => Kind::EventInstrumentGroupRefresh,
            rxegy_sys::XOBJ_EVENT_INSTRUMENT_GROUP_UPDATE => Kind::EventInstrumentGroupUpdate,
            rxegy_sys::XOBJ_EVENT_SESSION_STATUS => Kind::EventSessionStatus,
            rxegy_sys::XOBJ_EVENT_FX_REFRESH => Kind::EventFxRefresh,
            rxegy_sys::XOBJ_EVENT_FX_UPDATE => Kind::EventFxUpdate,
            rxegy_sys::XOBJ_EVENT_VOLATILITY_REFRESH => Kind::EventVolatilityRefresh,
            rxegy_sys::XOBJ_EVENT_VOLATILITY_QUOTE_RISK => Kind::EventVolatilityQuoteRisk,
            rxegy_sys::XOBJ_EVENT_VOLATILITY_TRADE_RISK => Kind::EventVolatilityTradeRisk,
            rxegy_sys::XOBJ_EVENT_SYMBOL_REFERENCE_REFRESH => Kind::EventSymbolReferenceRefresh,
            rxegy_sys::XOBJ_EVENT_SIGNAL_STATUS => Kind::EventSignalStatus,
            rxegy_sys::XOBJ_EVENT_TRADE_SUMMARY => Kind::EventTradeSummary,
            rxegy_sys::XOBJ_EVENT_EXCHANGE_STATISTICS => Kind::EventExchangeStatistics,
            rxegy_sys::XOBJ_READWRITE_BASKET_DEFINITION => Kind::ReadwriteBasketDefinition,
            rxegy_sys::XOBJ_READWRITE_KEYLIST_DEFINITION => Kind::ReadwriteKeylistDefinition,
            rxegy_sys::XOBJ_READWRITE_KEYLIST_FILTER_DEFINITION => {
                Kind::ReadwriteKeylistFilterDefinition
            }
            rxegy_sys::XOBJ_READWRITE_BASKET_CONSTITUENT => Kind::ReadwriteBasketConstituent,
            rxegy_sys::XOBJ_READWRITE_SIGNUM_PREDICTIVE_DATA => Kind::ReadwriteSignumPredictiveData,
            rxegy_sys::XOBJ_STATIC_EQUITY_SUMMARY => Kind::StaticEquitySummary,
            rxegy_sys::XOBJ_STATIC_COMMODITY_SUMMARY => Kind::StaticCommoditySummary,
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_LINES => Kind::StaticTickerPlantLines,
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SOCKETS => Kind::StaticTickerPlantSockets,
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_CLIENTS => Kind::StaticTickerPlantClients,
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_LINE_GAPS => Kind::StaticTickerPlantLineGaps,
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SUMMARY => Kind::StaticTickerPlantSummary,
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_LATENCY => Kind::StaticTickerPlantLatency,
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_CLIENTS_RATES => {
                Kind::StaticTickerPlantClientsRates
            }
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SUMMARY_RATES => {
                Kind::StaticTickerPlantSummaryRates
            }
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SESSIONS => Kind::StaticTickerPlantSessions,
            rxegy_sys::XOBJ_STATIC_TICKER_PLANT_SESSIONS_RATES => {
                Kind::StaticTickerPlantSessionsRates
            }

            _ => Kind::Invalid,
        }
    }
}

pub(crate) trait Wrapper: Sized {
    /// The Exegy object type this wrapper will encapsulate.
    const KIND: Kind;

    /// Create from a safe pointer without checking tye type.
    fn from_ptr_unchecked(non_null: NonNull<c_void>) -> Self;

    /// Get the inner pointer contained in this object as a raw handle.
    fn as_xhandle(&self) -> xhandle;

    /// Create from a raw pointer without retrieving the type.
    fn from_xhandle_and_type(ptr: xhandle, object_type: u16) -> Result<Self> {
        if Self::KIND as u16 == object_type {
            Ok(Self::from_ptr_unchecked(
                NonNull::new(ptr).ok_or(Error::NullObject)?,
            ))
        } else {
            Err(Error::UnexpectedKind)
        }
    }

    /// Create from a non-null pointer.
    ///
    /// This will verify the pointer contains an exegy object of the expected type.
    fn from_ptr(ptr: NonNull<c_void>) -> Result<Self> {
        let mut actual = 0;
        let status = unsafe { rxegy_sys::xcObjectType(ptr.as_ptr(), &mut actual) };

        Success::try_from(status)?;

        if Self::KIND as u16 == actual {
            Ok(Self::from_ptr_unchecked(ptr))
        } else {
            Err(Error::ObjectUnknown)
        }
    }

    /// Create from a raw pointer.
    ///
    /// This will verify the pointer is non-null, and is of the expected type.
    fn from_xhandle(handle: xhandle) -> Result<Self> {
        Self::from_ptr(NonNull::new(handle).ok_or(Error::NullObject)?)
    }
}
