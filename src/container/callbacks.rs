//! Container Callback Prototypes

use crate::{
    container::{EquityStream, KeylistCatalog},
    error::Result,
    event::{
        EquityCancel, EquityCorrection, EquityQuote, EquityRefresh, EquityTrade,
        ExchangeStatistics, IndicativePrice, KeylistCatalogRefresh, KeylistCatalogUpdate,
        OrderImbalance, Subscribe, TradeSummary, TradingAction,
    },
};
use std::any::Any;

// EQUITY STREAM CALLBACKS

/// The function prototype for a subscription callback on an equity stream.
pub type EquityStreamSubscribeFn =
    fn(stream: &EquityStream, event: &Subscribe, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a refresh event callback on an equity stream.
pub type EquityStreamRefreshFn =
    fn(stream: &EquityStream, event: &EquityRefresh, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a trade event callback on an equity stream.
pub type EquityStreamTradeFn =
    fn(stream: &EquityStream, event: &EquityTrade, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a quote event callback on an equity stream.
pub type EquityStreamQuoteFn =
    fn(stream: &EquityStream, event: &EquityQuote, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a cancel event callback on an equity stream.
pub type EquityStreamCancelFn =
    fn(stream: &EquityStream, event: &EquityCancel, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a correction event callback on an equity stream.
pub type EquityStreamCorrectionFn =
    fn(stream: &EquityStream, event: &EquityCorrection, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a order imbalance callback on an equity stream.
pub type EquityStreamOrderImbalanceFn =
    fn(stream: &EquityStream, event: &OrderImbalance, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a order imbalance callback on an equity stream.
pub type EquityStreamTradingActionFn =
    fn(stream: &EquityStream, event: &TradingAction, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a order imbalance callback on an equity stream.
pub type EquityStreamIndicativePriceFn =
    fn(stream: &EquityStream, event: &IndicativePrice, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a order imbalance callback on an equity stream.
pub type EquityStreamTradeSummaryFn =
    fn(stream: &EquityStream, event: &TradeSummary, user_data: Option<&dyn Any>) -> Result<()>;

/// The callback function prototype for a order imbalance callback on an equity stream.
pub type EquityStreamExchangeStatisticsFn = fn(
    stream: &EquityStream,
    event: &ExchangeStatistics,
    user_data: Option<&dyn Any>,
) -> Result<()>;

// KEYLIST CATALOG CALLBACKS

/// A callback signature for subscription event handlers.
pub type KeylistCatalogSubscribeFn =
    fn(catalog: &KeylistCatalog, event: &Subscribe, user_data: Option<&dyn Any>) -> Result<()>;

/// A callback signature for catalog refresh event handlers.
pub type KeylistCatalogRefreshFn = fn(
    catalog: &KeylistCatalog,
    event: &KeylistCatalogRefresh,
    user_data: Option<&dyn Any>,
) -> Result<()>;

/// A callback signature for catalog update event handlers.
pub type KeylistCatalogUpdateFn = fn(
    catalog: &KeylistCatalog,
    event: &KeylistCatalogUpdate,
    user_data: Option<&dyn Any>,
) -> Result<()>;
