//! Exegy Price Objects

use crate::Error;
use std::fmt::{Formatter, Result as FmtResult};

/// A price kind
#[derive(Clone, Copy, Debug, displaydoc::Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum PriceKind {
    /// The price is normal: {0}
    Normal(i32),
    /// The price is blank
    Blank = rxegy_sys::XC_BLANK_PRICE,
    /// The price is a market price
    Market = rxegy_sys::XC_MARKET_PRICE,
    /// The price is too large to represent with the corresponding exponent (price type)
    Overflow = rxegy_sys::XC_OVERFLOW_PRICE,
    /// The price is too small to represent with the corresponding exponent (price type)
    Underflow = rxegy_sys::XC_UNDERFLOW_PRICE,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Price(i32);

impl From<i32> for Price {
    fn from(src: i32) -> Price {
        Price(src)
    }
}

impl From<Price> for i32 {
    fn from(src: Price) -> i32 {
        src.0
    }
}

impl Price {
    /// Receive the raw value of the exegy price.
    pub fn raw_value(&self) -> i32 {
        self.0
    }

    /// Retrieve the value of a price mantissa, if there is one
    pub fn value(&self) -> Option<i32> {
        match self.0 {
            rxegy_sys::XC_BLANK_PRICE
            | rxegy_sys::XC_MARKET_PRICE
            | rxegy_sys::XC_OVERFLOW_PRICE
            | rxegy_sys::XC_UNDERFLOW_PRICE => None,
            val => Some(val),
        }
    }

    /// Retrieve the type of value contained in this price object
    pub fn kind(&self) -> PriceKind {
        match self.0 {
            rxegy_sys::XC_BLANK_PRICE => PriceKind::Blank,
            rxegy_sys::XC_MARKET_PRICE => PriceKind::Market,
            rxegy_sys::XC_OVERFLOW_PRICE => PriceKind::Overflow,
            rxegy_sys::XC_UNDERFLOW_PRICE => PriceKind::Underflow,
            val => PriceKind::Normal(val),
        }
    }
}

/// Format the given price and exponent as a decimal string.
pub fn format_price_string(
    f: &mut Formatter,
    price: Price,
    exponent_kind: ExponentKind,
) -> FmtResult {
    match price.kind() {
        PriceKind::Blank => Ok(()),
        PriceKind::Normal(value) => {
            let mut exponent = 1i32;
            for _val in 0..(exponent_kind as u8) {
                exponent *= 10;
            }

            let mantissa = value / exponent;
            let decimal = value % exponent;

            tracing::trace!("{value} to {exponent_kind} and {exponent} => {mantissa}.{decimal}");

            write!(
                f,
                "{mantissa}.{decimal:0exponent$}",
                exponent = exponent_kind as usize
            )
        }
        PriceKind::Market => write!(f, "*"),
        PriceKind::Overflow => write!(f, "(OVER)"),
        PriceKind::Underflow => write!(f, "(UNDER)"),
    }
}

/// An enumeration of Exegy price (exponent) types
#[derive(Clone, Copy, Debug, displaydoc::Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
#[non_exhaustive]
pub enum ExponentKind {
    /// 0 decimal places
    Decimal0 = rxegy_sys::XPT_DECIMAL_0,
    /// 1 decimal place
    Decimal1 = rxegy_sys::XPT_DECIMAL_1,
    /// 2 decimal places
    Decimal2 = rxegy_sys::XPT_DECIMAL_2,
    /// 3 decimal places
    Decimal3 = rxegy_sys::XPT_DECIMAL_3,
    /// 4 decimal places
    Decimal4 = rxegy_sys::XPT_DECIMAL_4,
    /// 5 decimal places
    Decimal5 = rxegy_sys::XPT_DECIMAL_5,
    /// 6 decimal places
    Decimal6 = rxegy_sys::XPT_DECIMAL_6,
    /// 7 decimal places
    Decimal7 = rxegy_sys::XPT_DECIMAL_7,
    /// 8 decimal places
    Decimal8 = rxegy_sys::XPT_DECIMAL_8,
    /// 9 decimal places
    Decimal9 = rxegy_sys::XPT_DECIMAL_9,
    /// fraction, 512ths
    Fractional512 = rxegy_sys::XPT_FRACTIONAL_512,
}

impl TryFrom<u8> for ExponentKind {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            rxegy_sys::XPT_DECIMAL_0 => Ok(Self::Decimal0),
            rxegy_sys::XPT_DECIMAL_1 => Ok(Self::Decimal1),
            rxegy_sys::XPT_DECIMAL_2 => Ok(Self::Decimal2),
            rxegy_sys::XPT_DECIMAL_3 => Ok(Self::Decimal3),
            rxegy_sys::XPT_DECIMAL_4 => Ok(Self::Decimal4),
            rxegy_sys::XPT_DECIMAL_5 => Ok(Self::Decimal5),
            rxegy_sys::XPT_DECIMAL_6 => Ok(Self::Decimal6),
            rxegy_sys::XPT_DECIMAL_7 => Ok(Self::Decimal7),
            rxegy_sys::XPT_DECIMAL_8 => Ok(Self::Decimal8),
            rxegy_sys::XPT_DECIMAL_9 => Ok(Self::Decimal9),
            rxegy_sys::XPT_FRACTIONAL_512 => Ok(Self::Fractional512),
            _ => Err(Error::InvalidExponent),
        }
    }
}
