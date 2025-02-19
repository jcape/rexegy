//! Exegy key support

/// A wrapper for an Exegy key
#[repr(transparent)]
pub struct Key(rexegy_sys::XC_KEY);

impl Key {
    pub fn exchange_id(&self) -> ExchangeId {
        ExchangeId::new(self.0.xk_exchange)
    }
}

/// A wrapper for an Exegy exchange ID
pub struct ExchangeId(rexegy_sys::XC_EXCHANGE_ID);

impl ExchangeId {
    pub fn new(inner: rexegy_sys::XC_EXCHANGE_ID) -> ExchangeId {
        ExchangeId(inner)
    }
}

/// A wrapper for an Exegy country ID
pub struct CountryIdView<'a>(&'a rexegy_sys::XC_COUNTRY_ID);

/// A wrapper for an Exegy symbol
pub struct ExegySymbolView<'a>(&'a rexegy_sys::XC_SYMBOL);
