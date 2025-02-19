//! Exegy object support

//// An enumeration of Exegy object types
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
#[non_exhaustive]
pub enum Kind {
    /// invalid
    Invalid = rexegy_sys::XOBJ_INVALID,
    /// ticker plant -- standard session
    SessionTicker = rexegy_sys::XOBJ_SESSION_TICKER,
    /// ticker plant monitoring session
    SessionTickerMonitoring = rexegy_sys::XOBJ_SESSION_TICKER_MONITORING,
    /// price summary objects
    RealtimeEquitySummary = rexegy_sys::XOBJ_REALTIME_EQUITY_SUMMARY,
    /// exchange-based subscriptions
    RealtimeExchangeStream = rexegy_sys::XOBJ_REALTIME_EXCHANGE_STREAM,
    /// order book summary objects
    RealtimeOrderBookSummary = rexegy_sys::XOBJ_REALTIME_ORDER_BOOK_SUMMARY,
    /// order book exchange-based subscriptions
    RealtimeOrderBookExchangeStream = rexegy_sys::XOBJ_REALTIME_ORDER_BOOK_EXCHANGE_STREAM,
    /// price book summary objects
    RealtimePriceBookSummary = rexegy_sys::XOBJ_REALTIME_PRICE_BOOK_SUMMARY,
    /// price book exchange-based subscriptions
    RealtimePriceBookExchangeStream = rexegy_sys::XOBJ_REALTIME_PRICE_BOOK_EXCHANGE_STREAM,
    /// basket summary objects
    RealtimeBasketSummary = rexegy_sys::XOBJ_REALTIME_BASKET_SUMMARY,
    /// list of all baskets
    RealtimeBasketCatalog = rexegy_sys::XOBJ_REALTIME_BASKET_CATALOG,
    /// filtered keylist results
    RealtimeKeylistFilter = rexegy_sys::XOBJ_REALTIME_KEYLIST_FILTER,
    /// catalog of keylists
    RealtimeKeylistCatalog = rexegy_sys::XOBJ_REALTIME_KEYLIST_CATALOG,
    /// commodity price summary objects
    RealtimeCommoditySummary = rexegy_sys::XOBJ_REALTIME_COMMODITY_SUMMARY,
    /// equity stream objects
    RealtimeEquityStream = rexegy_sys::XOBJ_REALTIME_EQUITY_STREAM,
    /// commodity stream objects
    RealtimeCommodityStream = rexegy_sys::XOBJ_REALTIME_COMMODITY_STREAM,
    /// order book stream objects
    RealtimeOrderBookStream = rexegy_sys::XOBJ_REALTIME_ORDER_BOOK_STREAM,
    /// price book stream objects
    RealtimePriceBookStream = rexegy_sys::XOBJ_REALTIME_PRICE_BOOK_STREAM,
    /// Ticker Plant feed information
    RealtimeTickerPlantFeeds = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_FEEDS,
    /// Ticker Plant line information
    RealtimeTickerPlantLines = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_LINES,
    /// Ticker Plant socket information
    RealtimeTickerPlantSockets = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SOCKETS,
    /// Ticker Plant client connection information
    RealtimeTickerPlantClients = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_CLIENTS,
    /// Derivative Summary
    RealtimeDerivativeReferenceSummary = rexegy_sys::XOBJ_REALTIME_DERIVATIVE_REFERENCE_SUMMARY,
    /// Derivative exchange stream
    RealtimeDerivativeReferenceExchangeStream =
        rexegy_sys::XOBJ_REALTIME_DERIVATIVE_REFERENCE_EXCHANGE_STREAM,
    /// Foreign Exchange Sopt Stream
    RealtimeFxspotStream = rexegy_sys::XOBJ_REALTIME_FXSPOT_STREAM,
    /// Foreign Exchange Forward Stream
    RealtimeFxforwardStream = rexegy_sys::XOBJ_REALTIME_FXFORWARD_STREAM,
    /// Foreign Exchange SWAP Stream
    RealtimeFxswapStream = rexegy_sys::XOBJ_REALTIME_FXSWAP_STREAM,
    /// Ticker Plant summary
    RealtimeTickerPlantSummary = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SUMMARY,
    /// Ticker Plant latency
    RealtimeTickerPlantLatency = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_LATENCY,
    /// Ticker Plant clients rates
    RealtimeTickerPlantClientsRates = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_CLIENTS_RATES,
    /// Ticker Plant summary rates
    RealtimeTickerPlantSummaryRates = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SUMMARY_RATES,
    /// Ticker Plant sessions (for session sets)
    RealtimeTickerPlantSessions = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SESSIONS,
    /// Ticker Plant sessions rates (for session sets)
    RealtimeTickerPlantSessionsRates = rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SESSIONS_RATES,
    /// Instrument Group Summary
    RealtimeInstrumentGroupSummary = rexegy_sys::XOBJ_REALTIME_INSTRUMENT_GROUP_SUMMARY,
    /// FX summary objects
    RealtimeFxSummary = rexegy_sys::XOBJ_REALTIME_FX_SUMMARY,
    /// FX stream objects
    RealtimeFxStream = rexegy_sys::XOBJ_REALTIME_FX_STREAM,
    /// Volatility summary objects
    RealtimeVolatilitySummary = rexegy_sys::XOBJ_REALTIME_VOLATILITY_SUMMARY,
    /// Symbol reference summary objects
    RealtimeSymbolReferenceSummary = rexegy_sys::XOBJ_REALTIME_SYMBOL_REFERENCE_SUMMARY,
    /// Equity instrument update
    WriteonlyEquityInstrument = rexegy_sys::XOBJ_WRITEONLY_EQUITY_INSTRUMENT,
    /// Commodity instrument update
    WriteonlyCommodityInstrument = rexegy_sys::XOBJ_WRITEONLY_COMMODITY_INSTRUMENT,
    /// Level 2 instrument update
    WriteonlyLevel2Instrument = rexegy_sys::XOBJ_WRITEONLY_LEVEL2_INSTRUMENT,
    /// Keylist edit
    WriteonlyKeylistEdit = rexegy_sys::XOBJ_WRITEONLY_KEYLIST_EDIT,
    /// Subscribe to instrument
    EventSubscribe = rexegy_sys::XOBJ_EVENT_SUBSCRIBE,
    /// Snapshot has been completed
    EventSnapshot = rexegy_sys::XOBJ_EVENT_SNAPSHOT,
    /// Equity trade event
    EventEquityTrade = rexegy_sys::XOBJ_EVENT_EQUITY_TRADE,
    /// Equity quote event
    EventEquityQuote = rexegy_sys::XOBJ_EVENT_EQUITY_QUOTE,
    /// Equity refresh event
    EventEquityRefresh = rexegy_sys::XOBJ_EVENT_EQUITY_REFRESH,
    /// Equity cancellation event
    EventEquityCancel = rexegy_sys::XOBJ_EVENT_EQUITY_CANCEL,
    /// Equity correction event
    EventEquityCorrection = rexegy_sys::XOBJ_EVENT_EQUITY_CORRECTION,
    /// Order book refresh on bid side (equities)
    EventOrderBookRefreshBid = rexegy_sys::XOBJ_EVENT_ORDER_BOOK_REFRESH_BID,
    /// Order book refresh on ask side (equities)
    EventOrderBookRefreshAsk = rexegy_sys::XOBJ_EVENT_ORDER_BOOK_REFRESH_ASK,
    /// Order book update (equities)
    EventOrderBookUpdate = rexegy_sys::XOBJ_EVENT_ORDER_BOOK_UPDATE,
    /// Price book refresh on bid side (equities)
    EventPriceBookRefreshBid = rexegy_sys::XOBJ_EVENT_PRICE_BOOK_REFRESH_BID,
    /// Price book refresh on ask side (equities)
    EventPriceBookRefreshAsk = rexegy_sys::XOBJ_EVENT_PRICE_BOOK_REFRESH_ASK,
    /// Price book update (equities)
    EventPriceBookUpdate = rexegy_sys::XOBJ_EVENT_PRICE_BOOK_UPDATE,
    /// Commodity refresh event
    EventCommodityRefresh = rexegy_sys::XOBJ_EVENT_COMMODITY_REFRESH,
    /// Commodity trade event
    EventCommodityTrade = rexegy_sys::XOBJ_EVENT_COMMODITY_TRADE,
    /// Commodity quote event
    EventCommodityQuote = rexegy_sys::XOBJ_EVENT_COMMODITY_QUOTE,
    /// Commodity cancel event
    EventCommodityCancel = rexegy_sys::XOBJ_EVENT_COMMODITY_CANCEL,
    /// Commodity correction event
    EventCommodityCorrection = rexegy_sys::XOBJ_EVENT_COMMODITY_CORRECTION,
    /// Basket NAV update
    EventNavUpdate = rexegy_sys::XOBJ_EVENT_NAV_UPDATE,
    /// Basket catalog refresh
    EventBasketCatalogRefresh = rexegy_sys::XOBJ_EVENT_BASKET_CATALOG_REFRESH,
    /// Basket catalog update
    EventBasketCatalogUpdate = rexegy_sys::XOBJ_EVENT_BASKET_CATALOG_UPDATE,
    /// Basket definition refresh
    EventBasketDefnRefresh = rexegy_sys::XOBJ_EVENT_BASKET_DEFN_REFRESH,
    /// Basket set/remove definition status
    EventBasketDefnStatus = rexegy_sys::XOBJ_EVENT_BASKET_DEFN_STATUS,
    /// Status of basket constituent update operation
    EventBasketConstituentUpdateStatus = rexegy_sys::XOBJ_EVENT_BASKET_CONSTITUENT_UPDATE_STATUS,
    /// Global basket constituent information
    EventBasketConstituentRefresh = rexegy_sys::XOBJ_EVENT_BASKET_CONSTITUENT_REFRESH,
    /// A one-time response (equities)
    EventStaticEquityRefresh = rexegy_sys::XOBJ_EVENT_STATIC_EQUITY_REFRESH,
    /// A one-time response (commodities)
    EventStaticCommodityRefresh = rexegy_sys::XOBJ_EVENT_STATIC_COMMODITY_REFRESH,
    /// Basket definition deleted
    EventBasketDefnDelete = rexegy_sys::XOBJ_EVENT_BASKET_DEFN_DELETE,
    /// Keylist definition status
    EventKeylistDefnStatus = rexegy_sys::XOBJ_EVENT_KEYLIST_DEFN_STATUS,
    /// Static keylist definition refresh
    EventKeylistDefnRefresh = rexegy_sys::XOBJ_EVENT_KEYLIST_DEFN_REFRESH,
    /// Static keylist definition deleted
    EventKeylistDefnDelete = rexegy_sys::XOBJ_EVENT_KEYLIST_DEFN_DELETE,
    /// Filtered keylist definition status
    EventKeylistFilterDefnStatus = rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_STATUS,
    /// Filtered keylist definition refresh
    EventKeylistFilterDefnRefresh = rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_REFRESH,
    /// Filtered keylist definition deleted
    EventKeylistFilterDefnDelete = rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_DELETE,
    /// Results of a filtered keylist subscription have begun
    EventKeylistFilterMatchStart = rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_START,
    /// Results of a filtered keylist subscription have ended
    EventKeylistFilterMatchEnd = rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_END,
    /// Results of a filtered keylist subscription
    EventKeylistFilterMatch = rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH,
    /// A key has been deleted that matched a filtered keylist subscription
    EventKeylistFilterMatchRemove = rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_REMOVE,
    /// Keylist catalog refresh
    EventKeylistCatalogRefresh = rexegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH,
    /// Keylist catalog update
    EventKeylistCatalogUpdate = rexegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE,
    /// Status of instrument update operation
    EventInstrumentUpdateStatus = rexegy_sys::XOBJ_EVENT_INSTRUMENT_UPDATE_STATUS,
    /// Signum predictive data refresh
    EventSignumPredictiveDataRefresh = rexegy_sys::XOBJ_EVENT_SIGNUM_PREDICTIVE_DATA_REFRESH,
    /// Signum predictive data status
    EventSignumPredictiveDataStatus = rexegy_sys::XOBJ_EVENT_SIGNUM_PREDICTIVE_DATA_STATUS,
    /// Order execution event
    EventOrderExecution = rexegy_sys::XOBJ_EVENT_ORDER_EXECUTION,
    /// Order imbalance notification
    EventOrderImbalance = rexegy_sys::XOBJ_EVENT_ORDER_IMBALANCE,
    /// Market directory event
    EventMarketDirectory = rexegy_sys::XOBJ_EVENT_MARKET_DIRECTORY,
    /// Trading action event
    EventTradingAction = rexegy_sys::XOBJ_EVENT_TRADING_ACTION,
    /// Refresh Ticker Plant feed information
    EventTickerPlantFeedRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_FEED_REFRESH,
    /// Update Ticker Plant feed information
    EventTickerPlantFeedUpdate = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_FEED_UPDATE,
    /// Refresh Ticker Plant line information
    EventTickerPlantLineRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_LINE_REFRESH,
    /// Refresh Ticker Plant socket information
    EventTickerPlantSocketRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SOCKET_REFRESH,
    /// Refresh Ticker Plant client connection information
    EventTickerPlantClientRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_CLIENT_REFRESH,
    /// Refresh Ticker Plant line information
    EventStaticTickerPlantLineRefresh = rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LINE_REFRESH,
    /// Refresh Ticker Plant socket information
    EventStaticTickerPlantSocketRefresh = rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SOCKET_REFRESH,
    /// Refresh Ticker Plant client connection information
    EventStaticTickerPlantClientRefresh = rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_CLIENT_REFRESH,
    /// Ticker Plant line gap list
    EventStaticTickerPlantLineGapsRefresh =
        rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LINE_GAPS_REFRESH,
    /// Derivative reference refresh event
    EventDerivativeReferenceRefresh = rexegy_sys::XOBJ_EVENT_DERIVATIVE_REFERENCE_REFRESH,
    /// Order on book event
    EventOrderOnBook = rexegy_sys::XOBJ_EVENT_ORDER_ON_BOOK,
    /// Foreign exchange spot refresh event
    EventFxspotRefresh = rexegy_sys::XOBJ_EVENT_FXSPOT_REFRESH,
    /// Foreign exchange spot quote event
    EventFxspotQuote = rexegy_sys::XOBJ_EVENT_FXSPOT_QUOTE,
    /// Foreign exchange forward refresh event
    EventFxfwdRefresh = rexegy_sys::XOBJ_EVENT_FXFWD_REFRESH,
    /// Foreign exchange forward quote event
    EventFxfwdQuote = rexegy_sys::XOBJ_EVENT_FXFWD_QUOTE,
    /// Foreign exchange swap refresh event
    EventFxswapRefresh = rexegy_sys::XOBJ_EVENT_FXSWAP_REFRESH,
    /// Foreign exchange swap quote event
    EventFxswapQuote = rexegy_sys::XOBJ_EVENT_FXSWAP_QUOTE,
    /// Ticker Plant summary refresh
    EventTickerPlantSummaryRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SUMMARY_REFRESH,
    /// Ticker Plant latency refresh
    EventTickerPlantLatencyRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_LATENCY_REFRESH,
    /// Ticker Plant clients rates refresh
    EventTickerPlantClientsRatesRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_CLIENTS_RATES_REFRESH,
    /// Ticker Plant summary rates refresh
    EventTickerPlantSummaryRatesRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SUMMARY_RATES_REFRESH,
    /// Ticker Plant summary refresh
    EventStaticTickerPlantSummaryRefresh =
        rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SUMMARY_REFRESH,
    /// Ticker Plant latency refresh
    EventStaticTickerPlantLatencyRefresh =
        rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LATENCY_REFRESH,
    /// Ticker Plant clients rates refresh
    EventStaticTickerPlantClientsRatesRefresh =
        rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_CLIENTS_RATES_REFRESH,
    /// Ticker Plant summary rates refresh
    EventStaticTickerPlantSummaryRatesRefresh =
        rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SUMMARY_RATES_REFRESH,
    /// Ticker Plant market wide circuit breaker update
    EventTickerPlantMwcbUpdate = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_MWCB_UPDATE,
    /// Refresh Ticker Plant sessions rates refresh
    EventTickerPlantSessionsRatesRefresh =
        rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SESSIONS_RATES_REFRESH,
    /// Refresh Ticker Plant sessions rates refresh
    EventStaticTickerPlantSessionsRatesRefresh =
        rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SESSIONS_RATES_REFRESH,
    /// Refresh Ticker Plant sessions stats refresh
    EventTickerPlantSessionsRefresh = rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SESSIONS_REFRESH,
    /// Refresh Ticker Plant sessions stats refresh
    EventStaticTickerPlantSessionsRefresh =
        rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SESSIONS_REFRESH,
    /// Request for quote event
    EventRequestForQuote = rexegy_sys::XOBJ_EVENT_REQUEST_FOR_QUOTE,
    /// Indicative Price event
    EventIndicativePrice = rexegy_sys::XOBJ_EVENT_INDICATIVE_PRICE,
    /// Instrument Group Refresh
    EventInstrumentGroupRefresh = rexegy_sys::XOBJ_EVENT_INSTRUMENT_GROUP_REFRESH,
    /// Instrument Group Refresh
    EventInstrumentGroupUpdate = rexegy_sys::XOBJ_EVENT_INSTRUMENT_GROUP_UPDATE,
    /// Session status has changed (e.g. connected or disconnected)
    EventSessionStatus = rexegy_sys::XOBJ_EVENT_SESSION_STATUS,
    /// FX refresh on bid side
    EventFxRefresh = rexegy_sys::XOBJ_EVENT_FX_REFRESH,
    /// FX update
    EventFxUpdate = rexegy_sys::XOBJ_EVENT_FX_UPDATE,
    /// Volatility refresh
    EventVolatilityRefresh = rexegy_sys::XOBJ_EVENT_VOLATILITY_REFRESH,
    /// Volatility quote risk
    EventVolatilityQuoteRisk = rexegy_sys::XOBJ_EVENT_VOLATILITY_QUOTE_RISK,
    /// Volatility trade risk
    EventVolatilityTradeRisk = rexegy_sys::XOBJ_EVENT_VOLATILITY_TRADE_RISK,
    /// Symbol reference refresh
    EventSymbolReferenceRefresh = rexegy_sys::XOBJ_EVENT_SYMBOL_REFERENCE_REFRESH,
    /// Signum signal status
    EventSignalStatus = rexegy_sys::XOBJ_EVENT_SIGNAL_STATUS,
    /// Trade summary
    EventTradeSummary = rexegy_sys::XOBJ_EVENT_TRADE_SUMMARY,
    /// Exchange statistics
    EventExchangeStatistics = rexegy_sys::XOBJ_EVENT_EXCHANGE_STATISTICS,
    /// Basket definition
    ReadwriteBasketDefinition = rexegy_sys::XOBJ_READWRITE_BASKET_DEFINITION,
    /// Static keylist definition
    ReadwriteKeylistDefinition = rexegy_sys::XOBJ_READWRITE_KEYLIST_DEFINITION,
    /// Filtered keylist definition
    ReadwriteKeylistFilterDefinition = rexegy_sys::XOBJ_READWRITE_KEYLIST_FILTER_DEFINITION,
    /// global basket constituent data
    ReadwriteBasketConstituent = rexegy_sys::XOBJ_READWRITE_BASKET_CONSTITUENT,
    /// Signum predictive data
    ReadwriteSignumPredictiveData = rexegy_sys::XOBJ_READWRITE_SIGNUM_PREDICTIVE_DATA,
    /// Equity summary
    StaticEquitySummary = rexegy_sys::XOBJ_STATIC_EQUITY_SUMMARY,
    /// Commodity summary
    StaticCommoditySummary = rexegy_sys::XOBJ_STATIC_COMMODITY_SUMMARY,
    /// Ticker Plant line information
    StaticTickerPlantLines = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_LINES,
    /// Ticker Plant socket information
    StaticTickerPlantSockets = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SOCKETS,
    /// Ticker Plant client connection information
    StaticTickerPlantClients = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_CLIENTS,
    /// Ticker Plant line gap list
    StaticTickerPlantLineGaps = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_LINE_GAPS,
    /// Ticker Plant summary
    StaticTickerPlantSummary = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SUMMARY,
    /// Ticker Plant latency
    StaticTickerPlantLatency = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_LATENCY,
    /// Ticker Plant clients rates
    StaticTickerPlantClientsRates = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_CLIENTS_RATES,
    /// Ticker Plant summary rates
    StaticTickerPlantSummaryRates = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SUMMARY_RATES,
    /// Ticker Plant sessions (for session sets)
    StaticTickerPlantSessions = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SESSIONS,
    /// Ticker Plant sessions rates (for session sets)
    StaticTickerPlantSessionsRates = rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SESSIONS_RATES,
}

impl From<u16> for Kind {
    fn from(value: u16) -> Self {
        match value {
            rexegy_sys::XOBJ_INVALID => Kind::Invalid,
            rexegy_sys::XOBJ_SESSION_TICKER => Kind::SessionTicker,
            rexegy_sys::XOBJ_SESSION_TICKER_MONITORING => Kind::SessionTickerMonitoring,
            rexegy_sys::XOBJ_REALTIME_EQUITY_SUMMARY => Kind::RealtimeEquitySummary,
            rexegy_sys::XOBJ_REALTIME_EXCHANGE_STREAM => Kind::RealtimeExchangeStream,
            rexegy_sys::XOBJ_REALTIME_ORDER_BOOK_SUMMARY => Kind::RealtimeOrderBookSummary,
            rexegy_sys::XOBJ_REALTIME_ORDER_BOOK_EXCHANGE_STREAM => {
                Kind::RealtimeOrderBookExchangeStream
            }
            rexegy_sys::XOBJ_REALTIME_PRICE_BOOK_SUMMARY => Kind::RealtimePriceBookSummary,
            rexegy_sys::XOBJ_REALTIME_PRICE_BOOK_EXCHANGE_STREAM => {
                Kind::RealtimePriceBookExchangeStream
            }
            rexegy_sys::XOBJ_REALTIME_BASKET_SUMMARY => Kind::RealtimeBasketSummary,
            rexegy_sys::XOBJ_REALTIME_BASKET_CATALOG => Kind::RealtimeBasketCatalog,
            rexegy_sys::XOBJ_REALTIME_KEYLIST_FILTER => Kind::RealtimeKeylistFilter,
            rexegy_sys::XOBJ_REALTIME_KEYLIST_CATALOG => Kind::RealtimeKeylistCatalog,
            rexegy_sys::XOBJ_REALTIME_COMMODITY_SUMMARY => Kind::RealtimeCommoditySummary,
            rexegy_sys::XOBJ_REALTIME_EQUITY_STREAM => Kind::RealtimeEquityStream,
            rexegy_sys::XOBJ_REALTIME_COMMODITY_STREAM => Kind::RealtimeCommodityStream,
            rexegy_sys::XOBJ_REALTIME_ORDER_BOOK_STREAM => Kind::RealtimeOrderBookStream,
            rexegy_sys::XOBJ_REALTIME_PRICE_BOOK_STREAM => Kind::RealtimePriceBookStream,
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_FEEDS => Kind::RealtimeTickerPlantFeeds,
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_LINES => Kind::RealtimeTickerPlantLines,
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SOCKETS => Kind::RealtimeTickerPlantSockets,
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_CLIENTS => Kind::RealtimeTickerPlantClients,
            rexegy_sys::XOBJ_REALTIME_DERIVATIVE_REFERENCE_SUMMARY => {
                Kind::RealtimeDerivativeReferenceSummary
            }
            rexegy_sys::XOBJ_REALTIME_DERIVATIVE_REFERENCE_EXCHANGE_STREAM => {
                Kind::RealtimeDerivativeReferenceExchangeStream
            }
            rexegy_sys::XOBJ_REALTIME_FXSPOT_STREAM => Kind::RealtimeFxspotStream,
            rexegy_sys::XOBJ_REALTIME_FXFORWARD_STREAM => Kind::RealtimeFxforwardStream,
            rexegy_sys::XOBJ_REALTIME_FXSWAP_STREAM => Kind::RealtimeFxswapStream,
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SUMMARY => Kind::RealtimeTickerPlantSummary,
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_LATENCY => Kind::RealtimeTickerPlantLatency,
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_CLIENTS_RATES => {
                Kind::RealtimeTickerPlantClientsRates
            }
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SUMMARY_RATES => {
                Kind::RealtimeTickerPlantSummaryRates
            }
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SESSIONS => Kind::RealtimeTickerPlantSessions,
            rexegy_sys::XOBJ_REALTIME_TICKER_PLANT_SESSIONS_RATES => {
                Kind::RealtimeTickerPlantSessionsRates
            }
            rexegy_sys::XOBJ_REALTIME_INSTRUMENT_GROUP_SUMMARY => {
                Kind::RealtimeInstrumentGroupSummary
            }
            rexegy_sys::XOBJ_REALTIME_FX_SUMMARY => Kind::RealtimeFxSummary,
            rexegy_sys::XOBJ_REALTIME_FX_STREAM => Kind::RealtimeFxStream,
            rexegy_sys::XOBJ_REALTIME_VOLATILITY_SUMMARY => Kind::RealtimeVolatilitySummary,
            rexegy_sys::XOBJ_REALTIME_SYMBOL_REFERENCE_SUMMARY => {
                Kind::RealtimeSymbolReferenceSummary
            }
            rexegy_sys::XOBJ_WRITEONLY_EQUITY_INSTRUMENT => Kind::WriteonlyEquityInstrument,
            rexegy_sys::XOBJ_WRITEONLY_COMMODITY_INSTRUMENT => Kind::WriteonlyCommodityInstrument,
            rexegy_sys::XOBJ_WRITEONLY_LEVEL2_INSTRUMENT => Kind::WriteonlyLevel2Instrument,
            rexegy_sys::XOBJ_WRITEONLY_KEYLIST_EDIT => Kind::WriteonlyKeylistEdit,
            rexegy_sys::XOBJ_EVENT_SUBSCRIBE => Kind::EventSubscribe,
            rexegy_sys::XOBJ_EVENT_SNAPSHOT => Kind::EventSnapshot,
            rexegy_sys::XOBJ_EVENT_EQUITY_TRADE => Kind::EventEquityTrade,
            rexegy_sys::XOBJ_EVENT_EQUITY_QUOTE => Kind::EventEquityQuote,
            rexegy_sys::XOBJ_EVENT_EQUITY_REFRESH => Kind::EventEquityRefresh,
            rexegy_sys::XOBJ_EVENT_EQUITY_CANCEL => Kind::EventEquityCancel,
            rexegy_sys::XOBJ_EVENT_EQUITY_CORRECTION => Kind::EventEquityCorrection,
            rexegy_sys::XOBJ_EVENT_ORDER_BOOK_REFRESH_BID => Kind::EventOrderBookRefreshBid,
            rexegy_sys::XOBJ_EVENT_ORDER_BOOK_REFRESH_ASK => Kind::EventOrderBookRefreshAsk,
            rexegy_sys::XOBJ_EVENT_ORDER_BOOK_UPDATE => Kind::EventOrderBookUpdate,
            rexegy_sys::XOBJ_EVENT_PRICE_BOOK_REFRESH_BID => Kind::EventPriceBookRefreshBid,
            rexegy_sys::XOBJ_EVENT_PRICE_BOOK_REFRESH_ASK => Kind::EventPriceBookRefreshAsk,
            rexegy_sys::XOBJ_EVENT_PRICE_BOOK_UPDATE => Kind::EventPriceBookUpdate,
            rexegy_sys::XOBJ_EVENT_COMMODITY_REFRESH => Kind::EventCommodityRefresh,
            rexegy_sys::XOBJ_EVENT_COMMODITY_TRADE => Kind::EventCommodityTrade,
            rexegy_sys::XOBJ_EVENT_COMMODITY_QUOTE => Kind::EventCommodityQuote,
            rexegy_sys::XOBJ_EVENT_COMMODITY_CANCEL => Kind::EventCommodityCancel,
            rexegy_sys::XOBJ_EVENT_COMMODITY_CORRECTION => Kind::EventCommodityCorrection,
            rexegy_sys::XOBJ_EVENT_NAV_UPDATE => Kind::EventNavUpdate,
            rexegy_sys::XOBJ_EVENT_BASKET_CATALOG_REFRESH => Kind::EventBasketCatalogRefresh,
            rexegy_sys::XOBJ_EVENT_BASKET_CATALOG_UPDATE => Kind::EventBasketCatalogUpdate,
            rexegy_sys::XOBJ_EVENT_BASKET_DEFN_REFRESH => Kind::EventBasketDefnRefresh,
            rexegy_sys::XOBJ_EVENT_BASKET_DEFN_STATUS => Kind::EventBasketDefnStatus,
            rexegy_sys::XOBJ_EVENT_BASKET_CONSTITUENT_UPDATE_STATUS => {
                Kind::EventBasketConstituentUpdateStatus
            }
            rexegy_sys::XOBJ_EVENT_BASKET_CONSTITUENT_REFRESH => {
                Kind::EventBasketConstituentRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_EQUITY_REFRESH => Kind::EventStaticEquityRefresh,
            rexegy_sys::XOBJ_EVENT_STATIC_COMMODITY_REFRESH => Kind::EventStaticCommodityRefresh,
            rexegy_sys::XOBJ_EVENT_BASKET_DEFN_DELETE => Kind::EventBasketDefnDelete,
            rexegy_sys::XOBJ_EVENT_KEYLIST_DEFN_STATUS => Kind::EventKeylistDefnStatus,
            rexegy_sys::XOBJ_EVENT_KEYLIST_DEFN_REFRESH => Kind::EventKeylistDefnRefresh,
            rexegy_sys::XOBJ_EVENT_KEYLIST_DEFN_DELETE => Kind::EventKeylistDefnDelete,
            rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_STATUS => Kind::EventKeylistFilterDefnStatus,
            rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_REFRESH => {
                Kind::EventKeylistFilterDefnRefresh
            }
            rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_DEFN_DELETE => Kind::EventKeylistFilterDefnDelete,
            rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_START => Kind::EventKeylistFilterMatchStart,
            rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_END => Kind::EventKeylistFilterMatchEnd,
            rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH => Kind::EventKeylistFilterMatch,
            rexegy_sys::XOBJ_EVENT_KEYLIST_FILTER_MATCH_REMOVE => {
                Kind::EventKeylistFilterMatchRemove
            }
            rexegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH => Kind::EventKeylistCatalogRefresh,
            rexegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE => Kind::EventKeylistCatalogUpdate,
            rexegy_sys::XOBJ_EVENT_INSTRUMENT_UPDATE_STATUS => Kind::EventInstrumentUpdateStatus,
            rexegy_sys::XOBJ_EVENT_SIGNUM_PREDICTIVE_DATA_REFRESH => {
                Kind::EventSignumPredictiveDataRefresh
            }
            rexegy_sys::XOBJ_EVENT_SIGNUM_PREDICTIVE_DATA_STATUS => {
                Kind::EventSignumPredictiveDataStatus
            }
            rexegy_sys::XOBJ_EVENT_ORDER_EXECUTION => Kind::EventOrderExecution,
            rexegy_sys::XOBJ_EVENT_ORDER_IMBALANCE => Kind::EventOrderImbalance,
            rexegy_sys::XOBJ_EVENT_MARKET_DIRECTORY => Kind::EventMarketDirectory,
            rexegy_sys::XOBJ_EVENT_TRADING_ACTION => Kind::EventTradingAction,
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_FEED_REFRESH => Kind::EventTickerPlantFeedRefresh,
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_FEED_UPDATE => Kind::EventTickerPlantFeedUpdate,
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_LINE_REFRESH => Kind::EventTickerPlantLineRefresh,
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SOCKET_REFRESH => {
                Kind::EventTickerPlantSocketRefresh
            }
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_CLIENT_REFRESH => {
                Kind::EventTickerPlantClientRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LINE_REFRESH => {
                Kind::EventStaticTickerPlantLineRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SOCKET_REFRESH => {
                Kind::EventStaticTickerPlantSocketRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_CLIENT_REFRESH => {
                Kind::EventStaticTickerPlantClientRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LINE_GAPS_REFRESH => {
                Kind::EventStaticTickerPlantLineGapsRefresh
            }
            rexegy_sys::XOBJ_EVENT_DERIVATIVE_REFERENCE_REFRESH => {
                Kind::EventDerivativeReferenceRefresh
            }
            rexegy_sys::XOBJ_EVENT_ORDER_ON_BOOK => Kind::EventOrderOnBook,
            rexegy_sys::XOBJ_EVENT_FXSPOT_REFRESH => Kind::EventFxspotRefresh,
            rexegy_sys::XOBJ_EVENT_FXSPOT_QUOTE => Kind::EventFxspotQuote,
            rexegy_sys::XOBJ_EVENT_FXFWD_REFRESH => Kind::EventFxfwdRefresh,
            rexegy_sys::XOBJ_EVENT_FXFWD_QUOTE => Kind::EventFxfwdQuote,
            rexegy_sys::XOBJ_EVENT_FXSWAP_REFRESH => Kind::EventFxswapRefresh,
            rexegy_sys::XOBJ_EVENT_FXSWAP_QUOTE => Kind::EventFxswapQuote,
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SUMMARY_REFRESH => {
                Kind::EventTickerPlantSummaryRefresh
            }
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_LATENCY_REFRESH => {
                Kind::EventTickerPlantLatencyRefresh
            }
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_CLIENTS_RATES_REFRESH => {
                Kind::EventTickerPlantClientsRatesRefresh
            }
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SUMMARY_RATES_REFRESH => {
                Kind::EventTickerPlantSummaryRatesRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SUMMARY_REFRESH => {
                Kind::EventStaticTickerPlantSummaryRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_LATENCY_REFRESH => {
                Kind::EventStaticTickerPlantLatencyRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_CLIENTS_RATES_REFRESH => {
                Kind::EventStaticTickerPlantClientsRatesRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SUMMARY_RATES_REFRESH => {
                Kind::EventStaticTickerPlantSummaryRatesRefresh
            }
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_MWCB_UPDATE => Kind::EventTickerPlantMwcbUpdate,
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SESSIONS_RATES_REFRESH => {
                Kind::EventTickerPlantSessionsRatesRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SESSIONS_RATES_REFRESH => {
                Kind::EventStaticTickerPlantSessionsRatesRefresh
            }
            rexegy_sys::XOBJ_EVENT_TICKER_PLANT_SESSIONS_REFRESH => {
                Kind::EventTickerPlantSessionsRefresh
            }
            rexegy_sys::XOBJ_EVENT_STATIC_TICKER_PLANT_SESSIONS_REFRESH => {
                Kind::EventStaticTickerPlantSessionsRefresh
            }
            rexegy_sys::XOBJ_EVENT_REQUEST_FOR_QUOTE => Kind::EventRequestForQuote,
            rexegy_sys::XOBJ_EVENT_INDICATIVE_PRICE => Kind::EventIndicativePrice,
            rexegy_sys::XOBJ_EVENT_INSTRUMENT_GROUP_REFRESH => Kind::EventInstrumentGroupRefresh,
            rexegy_sys::XOBJ_EVENT_INSTRUMENT_GROUP_UPDATE => Kind::EventInstrumentGroupUpdate,
            rexegy_sys::XOBJ_EVENT_SESSION_STATUS => Kind::EventSessionStatus,
            rexegy_sys::XOBJ_EVENT_FX_REFRESH => Kind::EventFxRefresh,
            rexegy_sys::XOBJ_EVENT_FX_UPDATE => Kind::EventFxUpdate,
            rexegy_sys::XOBJ_EVENT_VOLATILITY_REFRESH => Kind::EventVolatilityRefresh,
            rexegy_sys::XOBJ_EVENT_VOLATILITY_QUOTE_RISK => Kind::EventVolatilityQuoteRisk,
            rexegy_sys::XOBJ_EVENT_VOLATILITY_TRADE_RISK => Kind::EventVolatilityTradeRisk,
            rexegy_sys::XOBJ_EVENT_SYMBOL_REFERENCE_REFRESH => Kind::EventSymbolReferenceRefresh,
            rexegy_sys::XOBJ_EVENT_SIGNAL_STATUS => Kind::EventSignalStatus,
            rexegy_sys::XOBJ_EVENT_TRADE_SUMMARY => Kind::EventTradeSummary,
            rexegy_sys::XOBJ_EVENT_EXCHANGE_STATISTICS => Kind::EventExchangeStatistics,
            rexegy_sys::XOBJ_READWRITE_BASKET_DEFINITION => Kind::ReadwriteBasketDefinition,
            rexegy_sys::XOBJ_READWRITE_KEYLIST_DEFINITION => Kind::ReadwriteKeylistDefinition,
            rexegy_sys::XOBJ_READWRITE_KEYLIST_FILTER_DEFINITION => {
                Kind::ReadwriteKeylistFilterDefinition
            }
            rexegy_sys::XOBJ_READWRITE_BASKET_CONSTITUENT => Kind::ReadwriteBasketConstituent,
            rexegy_sys::XOBJ_READWRITE_SIGNUM_PREDICTIVE_DATA => {
                Kind::ReadwriteSignumPredictiveData
            }
            rexegy_sys::XOBJ_STATIC_EQUITY_SUMMARY => Kind::StaticEquitySummary,
            rexegy_sys::XOBJ_STATIC_COMMODITY_SUMMARY => Kind::StaticCommoditySummary,
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_LINES => Kind::StaticTickerPlantLines,
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SOCKETS => Kind::StaticTickerPlantSockets,
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_CLIENTS => Kind::StaticTickerPlantClients,
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_LINE_GAPS => Kind::StaticTickerPlantLineGaps,
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SUMMARY => Kind::StaticTickerPlantSummary,
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_LATENCY => Kind::StaticTickerPlantLatency,
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_CLIENTS_RATES => {
                Kind::StaticTickerPlantClientsRates
            }
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SUMMARY_RATES => {
                Kind::StaticTickerPlantSummaryRates
            }
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SESSIONS => Kind::StaticTickerPlantSessions,
            rexegy_sys::XOBJ_STATIC_TICKER_PLANT_SESSIONS_RATES => {
                Kind::StaticTickerPlantSessionsRates
            }

            _ => Kind::Invalid,
        }
    }
}
