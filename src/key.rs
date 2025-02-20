//! Exegy key support

use ref_cast::RefCast;

/// A wrapper for an Exegy key
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct Key(rexegy_sys::XC_KEY);

impl Key {
    /// Retrieve a reference to the exchange ID in this key
    pub fn exchange_id(&self) -> &ExchangeId {
        ExchangeId::ref_cast(&self.0.xk_exchange)
    }

    /// Retrieve a reference to the country code in this key
    pub fn country_id(&self) -> &CountryId {
        CountryId::ref_cast(&self.0.xk_country)
    }

    /// Retrieve a reference to the symbol in this key
    pub fn symbol(&self) -> &Symbol {
        Symbol::ref_cast(&self.0.xk_symbol)
    }
}

/// Exegy "exchange" (feed) code
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct ExchangeId(rexegy_sys::XC_EXCHANGE_ID);

/// Exegy "country" code
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct CountryId(rexegy_sys::XC_COUNTRY_ID);

/// Exegy symbol data.
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct Symbol(rexegy_sys::XC_SYMBOL);
