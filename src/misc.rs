use ref_cast::RefCast;
use rxegy_sys::XC_TRADE_VENUE;

use crate::Error;

/// A trade venue, stored as a 4-ascii-character MIC code
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct TradeVenue(XC_TRADE_VENUE);

impl TradeVenue {
    pub(crate) fn new(inner: XC_TRADE_VENUE) -> Self {
        Self(inner)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HiTime(u64);

impl From<u64> for HiTime {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<HiTime> for u64 {
    fn from(value: HiTime) -> Self {
        value.0
    }
}

/// An enumeration of order reference ID encodings
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum OrderRefIdKind {
    /// 7-bit ASCII
    Ascii = rxegy_sys::XOIDT_ASCII,
    /// Binary-coded decimal (4 bits per decimal digit)
    BinaryCodedDecimal = rxegy_sys::XOIDT_BCD,
    /// Pair of unsigned 32-bit integers
    UintPair = rxegy_sys::XOIDT_UINT_PAIR,
    /// 12-character alphanumeric format encoded as a base-36 number
    Alpha32 = rxegy_sys::XOIDT_ALPHA36,
    /// A single unsigned 64-bit integer
    Raw64 = rxegy_sys::XOIDT_RAW64,
    /// Union of instrument ID and binary ref number -- disambiguates per-instrument reference IDs
    SymIdUint = rxegy_sys::XOIDT_SYMID_UINT,
}

impl TryFrom<u8> for OrderRefIdKind {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            rxegy_sys::XOIDT_ASCII => Ok(Self::Ascii),
            rxegy_sys::XOIDT_BCD => Ok(Self::BinaryCodedDecimal),
            rxegy_sys::XOIDT_UINT_PAIR => Ok(Self::UintPair),
            rxegy_sys::XOIDT_ALPHA36 => Ok(Self::Alpha32),
            rxegy_sys::XOIDT_RAW64 => Ok(Self::Raw64),
            rxegy_sys::XOIDT_SYMID_UINT => Ok(Self::SymIdUint),
            _ => Err(Error::KindUnknown),
        }
    }
}
