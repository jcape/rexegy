use crate::Error;
use ref_cast::RefCast;
use rxegy_sys::{XC_CURRENCY_ID, XC_DATE, XC_TRADE_VENUE};

/// A currency ID
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct Currency(XC_CURRENCY_ID);

impl Currency {
    pub(crate) fn new(inner: XC_CURRENCY_ID) -> Self {
        Self(inner)
    }
}

impl From<Currency> for [u8; 3] {
    fn from(value: Currency) -> Self {
        [
            value.0.xcur_ch[0] as u8,
            value.0.xcur_ch[1] as u8,
            value.0.xcur_ch[2] as u8,
        ]
    }
}

/// A date
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct Date(XC_DATE);

impl Date {
    pub(crate) fn new(inner: XC_DATE) -> Self {
        Self(inner)
    }

    pub fn year(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1 }.xdt_year()
    }

    pub fn month(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1 }.xdt_month()
    }

    pub fn day(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1 }.xdt_day()
    }
}

impl From<&Date> for u32 {
    fn from(value: &Date) -> Self {
        unsafe { value.0.xdt_raw }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HiTime(u64);

impl From<u64> for HiTime {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<HiTime> for u64 {
    #[inline(always)]
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

/// A value suitable for representing the size associated with a trade or quote
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Size(u32);

impl From<u32> for Size {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Size(value)
    }
}

impl From<Size> for u32 {
    #[inline(always)]
    fn from(value: Size) -> Self {
        value.0
    }
}

/// A enumeration of Exegy symbol types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
#[non_exhaustive]
pub enum SymbolKind {
    /// Invalid/Unknown
    Invalid = rxegy_sys::XSYMTYP_INVALID,
    /// Level I Equity
    Equity = rxegy_sys::XSYMTYP_EQUITY,
    /// Level I Equity option
    EquityOption = rxegy_sys::XSYMTYP_EQUITY_OPTION,
    /// Net Asset Value
    Nav = rxegy_sys::XSYMTYP_NAV,
    /// Fund
    Fund = rxegy_sys::XSYMTYP_FUND,
    /// Warrant
    Warrant = rxegy_sys::XSYMTYP_WARRANT,
    /// Equity Index
    Index = rxegy_sys::XSYMTYP_INDEX,
    /// L1 Foreign exchange
    ForeignExchange = rxegy_sys::XSYMTYP_FOREIGN_EXCHANGE,
    /// Bond
    Bond = rxegy_sys::XSYMTYP_BOND,
    /// Fixed Income
    FixedIncome = rxegy_sys::XSYMTYP_FIXED_INCOME,
    /// Future
    Future = rxegy_sys::XSYMTYP_FUTURE,
    /// Future option
    FutureOption = rxegy_sys::XSYMTYP_FUTURE_OPTION,
    /// Future spread
    FutureSpread = rxegy_sys::XSYMTYP_FUTURE_SPREAD,
    /// Static keylist
    UserKeylist = rxegy_sys::XSYMTYP_USER_KEYLIST,
    /// Filtered keylist
    FilterKeylist = rxegy_sys::XSYMTYP_FILTER_KEYLIST,
    /// Level II Equity
    EquityL2 = rxegy_sys::XSYMTYP_EQUITY_L2,
    /// Level II Equity option
    EquityOptionL2 = rxegy_sys::XSYMTYP_EQUITY_OPTION_L2,
    /// Level II Future
    FutureL2 = rxegy_sys::XSYMTYP_FUTURE_L2,
    /// Level II Future option
    FutureOptionL2 = rxegy_sys::XSYMTYP_FUTURE_OPTION_L2,
    /// Level II Future spread
    FutureSpreadL2 = rxegy_sys::XSYMTYP_FUTURE_SPREAD_L2,
    /// Commodity Index
    CommodityIndex = rxegy_sys::XSYMTYP_COMMODITY_INDEX,
    /// Future Option spread
    FutureOptionSpread = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD,
    /// Level II Future Option spread
    FutureOptionSpreadL2 = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_L2,
    /// Equity Option spread
    EquityOptionSpread = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD,
    /// Level II Equity Option spread
    EquityOptionSpreadL2 = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_L2,
    /// Level 2 Foreign Exchange
    ForeignExchangeL2 = rxegy_sys::XSYMTYP_FOREIGN_EXCHANGE_L2,
    /// Equity Option Stock Combo
    EquityOptionStockCombo = rxegy_sys::XSYMTYP_EQUITY_OPTION_STOCK_COMBO,
    /// Equity Option Stock Combo L2
    EquityOptionStockComboL2 = rxegy_sys::XSYMTYP_EQUITY_OPTION_STOCK_COMBO_L2,
    /// Exchange Traded Funds
    Etf = rxegy_sys::XSYMTYP_ETF,
    /// Currency Options
    Currency = rxegy_sys::XSYMTYP_CURRENCY,
    /// Future Spread Vertical
    FutureSpreadVertical = rxegy_sys::XSYMTYP_FUTURE_SPREAD_VERTICAL,
    /// Future Spread Calendar
    FutureSpreadCalendar = rxegy_sys::XSYMTYP_FUTURE_SPREAD_CALENDAR,
    /// Future Spread Straddle
    FutureSpreadStraddle = rxegy_sys::XSYMTYP_FUTURE_SPREAD_STRADDLE,
    /// Future Spread Non-Standard
    FutureSpreadNonStandard = rxegy_sys::XSYMTYP_FUTURE_SPREAD_NON_STANDARD,
    /// Future Spread Vertical L2
    FutureSpreadVerticalL2 = rxegy_sys::XSYMTYP_FUTURE_SPREAD_VERTICAL_L2,
    /// Future Spread Calendar L2
    FutureSpreadCalendarL2 = rxegy_sys::XSYMTYP_FUTURE_SPREAD_CALENDAR_L2,
    /// Future Spread Straddle L2
    FutureSpreadStraddleL2 = rxegy_sys::XSYMTYP_FUTURE_SPREAD_STRADDLE_L2,
    /// Future Spread Non-Standard L2
    FutureSpreadNonStandardL2 = rxegy_sys::XSYMTYP_FUTURE_SPREAD_NON_STANDARD_L2,
    /// Future Option Spread Vertical
    FutureOptionSpreadVertical = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_VERTICAL,
    /// Future Option Spread Calendar
    FutureOptionSpreadCalendar = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_CALENDAR,
    /// Future Option Spread Straddle
    FutureOptionSpreadStraddle = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_STRADDLE,
    /// Future Option Spread Non-Standard
    FutureOptionSpreadNonStandard = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_NON_STANDARD,
    /// Future Option Spread Vertical L2
    FutureOptionSpreadVerticalL2 = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_VERTICAL_L2,
    /// Future Option Spread Calendar L2
    FutureOptionSpreadCalendarL2 = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_CALENDAR_L2,
    /// Future Option Spread Straddle L2
    FutureOptionSpreadStraddleL2 = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_STRADDLE_L2,
    /// Future Option Spread Non-Standard L2
    FutureOptionSpreadNonStandardL2 = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_NON_STANDARD_L2,
    /// Equity Option Spread Vertical
    EquityOptionSpreadVertical = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_VERTICAL,
    /// Equity Option Spread Calendar
    EquityOptionSpreadCalendar = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_CALENDAR,
    /// Equity Option Spread Straddle
    EquityOptionSpreadStraddle = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_STRADDLE,
    /// Equity Option Spread Non-Standard
    EquityOptionSpreadNonStandard = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_NON_STANDARD,
    /// Equity Option Spread Vertical L2
    EquityOptionSpreadVerticalL2 = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_VERTICAL_L2,
    /// Equity option Spread Calendar L2
    EquityOptionSpreadCalendarL2 = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_CALENDAR_L2,
    /// Equity Option Spread Straddle L2
    EquityOptionSpreadStraddleL2 = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_STRADDLE_L2,
    /// Equity Option Spread Non-Standard L2
    EquityOptionSpreadNonStandardL2 = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_NON_STANDARD_L2,
    /// Future Spread Strangle
    FutureSpreadStrangle = rxegy_sys::XSYMTYP_FUTURE_SPREAD_STRANGLE,
    /// Future Spread Strangle L2
    FutureSpreadStrangleL2 = rxegy_sys::XSYMTYP_FUTURE_SPREAD_STRANGLE_L2,
    /// Future Option Spread Strangle
    FutureOptionSpreadStrangle = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_STRANGLE,
    /// Future Option Spread Strangle L2
    FutureOptionSpreadStrangleL2 = rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_STRANGLE_L2,
    /// Equity Option Spread Strangle
    EquityOptionSpreadStrangle = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_STRANGLE,
    /// Equity Option Spread Strangle L2
    EquityOptionSpreadStrangleL2 = rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_STRANGLE_L2,
    /// Currency Index
    CurrencyIndex = rxegy_sys::XSYMTYP_CURRENCY_INDEX,
    /// ETF Index
    EtfIndex = rxegy_sys::XSYMTYP_ETF_INDEX,
    /// Foreign Exchange Index
    ForeignExchangeIndex = rxegy_sys::XSYMTYP_FOREIGN_EXCHANGE_INDEX,
    /// Interest Rate
    InterestRate = rxegy_sys::XSYMTYP_INTEREST_RATE,
    /// Interest Rate Index
    InterestRateIndex = rxegy_sys::XSYMTYP_INTEREST_RATE_INDEX,
    /// Fund L2
    FundL2 = rxegy_sys::XSYMTYP_FUND_L2,
    /// Warrant L2
    WarrantL2 = rxegy_sys::XSYMTYP_WARRANT_L2,
    /// Equity Index L2
    IndexL2 = rxegy_sys::XSYMTYP_INDEX_L2,
    /// Bond L2
    BondL2 = rxegy_sys::XSYMTYP_BOND_L2,
    /// Fixed Income L2
    FixedIncomeL2 = rxegy_sys::XSYMTYP_FIXED_INCOME_L2,
    /// Commodity Index L2
    CommodityIndexL2 = rxegy_sys::XSYMTYP_COMMODITY_INDEX_L2,
    /// Exchange Traded Funds L2
    EtfL2 = rxegy_sys::XSYMTYP_ETF_L2,
    /// Currency Options L2
    CurrencyL2 = rxegy_sys::XSYMTYP_CURRENCY_L2,
    /// Currency Index L2
    CurrencyIndexL2 = rxegy_sys::XSYMTYP_CURRENCY_INDEX_L2,
    /// ETF Index L2
    EtfIndexL2 = rxegy_sys::XSYMTYP_ETF_INDEX_L2,
    /// Foreign Exchange Index L2
    ForeignExchangeIndexL2 = rxegy_sys::XSYMTYP_FOREIGN_EXCHANGE_INDEX_L2,
    /// Interest Rate L2
    InterestRateL2 = rxegy_sys::XSYMTYP_INTEREST_RATE_L2,
    /// Interest Rate Index L2
    InterestRateIndexL2 = rxegy_sys::XSYMTYP_INTEREST_RATE_INDEX_L2,
    /// Subscriptions to fx spot symbols
    FxSpot = rxegy_sys::XSYMTYP_FX_SPOT,
    /// Subscriptions to fx forward symbols
    FxForward = rxegy_sys::XSYMTYP_FX_FORWARD,
    /// Subscriptions to fx non-deliverable forward symbols
    FxNdf = rxegy_sys::XSYMTYP_FX_NDF,
    /// Subscriptions to fx swap symbols
    FxSwap = rxegy_sys::XSYMTYP_FX_SWAP,
}

impl From<u8> for SymbolKind {
    fn from(value: u8) -> Self {
        match value {
            rxegy_sys::XSYMTYP_EQUITY => Self::Equity,
            rxegy_sys::XSYMTYP_EQUITY_OPTION => Self::EquityOption,
            rxegy_sys::XSYMTYP_NAV => Self::Nav,
            rxegy_sys::XSYMTYP_FUND => Self::Fund,
            rxegy_sys::XSYMTYP_WARRANT => Self::Warrant,
            rxegy_sys::XSYMTYP_INDEX => Self::Index,
            rxegy_sys::XSYMTYP_FOREIGN_EXCHANGE => Self::ForeignExchange,
            rxegy_sys::XSYMTYP_BOND => Self::Bond,
            rxegy_sys::XSYMTYP_FIXED_INCOME => Self::FixedIncome,
            rxegy_sys::XSYMTYP_FUTURE => Self::Future,
            rxegy_sys::XSYMTYP_FUTURE_OPTION => Self::FutureOption,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD => Self::FutureSpread,
            rxegy_sys::XSYMTYP_USER_KEYLIST => Self::UserKeylist,
            rxegy_sys::XSYMTYP_FILTER_KEYLIST => Self::FilterKeylist,
            rxegy_sys::XSYMTYP_EQUITY_L2 => Self::EquityL2,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_L2 => Self::EquityOptionL2,
            rxegy_sys::XSYMTYP_FUTURE_L2 => Self::FutureL2,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_L2 => Self::FutureOptionL2,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_L2 => Self::FutureSpreadL2,
            rxegy_sys::XSYMTYP_COMMODITY_INDEX => Self::CommodityIndex,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD => Self::FutureOptionSpread,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_L2 => Self::FutureOptionSpreadL2,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD => Self::EquityOptionSpread,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_L2 => Self::EquityOptionSpreadL2,
            rxegy_sys::XSYMTYP_FOREIGN_EXCHANGE_L2 => Self::ForeignExchangeL2,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_STOCK_COMBO => Self::EquityOptionStockCombo,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_STOCK_COMBO_L2 => Self::EquityOptionStockComboL2,
            rxegy_sys::XSYMTYP_ETF => Self::Etf,
            rxegy_sys::XSYMTYP_CURRENCY => Self::Currency,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_VERTICAL => Self::FutureSpreadVertical,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_CALENDAR => Self::FutureSpreadCalendar,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_STRADDLE => Self::FutureSpreadStraddle,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_NON_STANDARD => Self::FutureSpreadNonStandard,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_VERTICAL_L2 => Self::FutureSpreadVerticalL2,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_CALENDAR_L2 => Self::FutureSpreadCalendarL2,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_STRADDLE_L2 => Self::FutureSpreadStraddleL2,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_NON_STANDARD_L2 => Self::FutureSpreadNonStandardL2,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_VERTICAL => Self::FutureOptionSpreadVertical,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_CALENDAR => Self::FutureOptionSpreadCalendar,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_STRADDLE => Self::FutureOptionSpreadStraddle,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_NON_STANDARD => {
                Self::FutureOptionSpreadNonStandard
            }
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_VERTICAL_L2 => {
                Self::FutureOptionSpreadVerticalL2
            }
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_CALENDAR_L2 => {
                Self::FutureOptionSpreadCalendarL2
            }
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_STRADDLE_L2 => {
                Self::FutureOptionSpreadStraddleL2
            }
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_NON_STANDARD_L2 => {
                Self::FutureOptionSpreadNonStandardL2
            }
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_VERTICAL => Self::EquityOptionSpreadVertical,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_CALENDAR => Self::EquityOptionSpreadCalendar,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_STRADDLE => Self::EquityOptionSpreadStraddle,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_NON_STANDARD => {
                Self::EquityOptionSpreadNonStandard
            }
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_VERTICAL_L2 => {
                Self::EquityOptionSpreadVerticalL2
            }
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_CALENDAR_L2 => {
                Self::EquityOptionSpreadCalendarL2
            }
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_STRADDLE_L2 => {
                Self::EquityOptionSpreadStraddleL2
            }
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_NON_STANDARD_L2 => {
                Self::EquityOptionSpreadNonStandardL2
            }
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_STRANGLE => Self::FutureSpreadStrangle,
            rxegy_sys::XSYMTYP_FUTURE_SPREAD_STRANGLE_L2 => Self::FutureSpreadStrangleL2,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_STRANGLE => Self::FutureOptionSpreadStrangle,
            rxegy_sys::XSYMTYP_FUTURE_OPTION_SPREAD_STRANGLE_L2 => {
                Self::FutureOptionSpreadStrangleL2
            }
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_STRANGLE => Self::EquityOptionSpreadStrangle,
            rxegy_sys::XSYMTYP_EQUITY_OPTION_SPREAD_STRANGLE_L2 => {
                Self::EquityOptionSpreadStrangleL2
            }
            rxegy_sys::XSYMTYP_CURRENCY_INDEX => Self::CurrencyIndex,
            rxegy_sys::XSYMTYP_ETF_INDEX => Self::EtfIndex,
            rxegy_sys::XSYMTYP_FOREIGN_EXCHANGE_INDEX => Self::ForeignExchangeIndex,
            rxegy_sys::XSYMTYP_INTEREST_RATE => Self::InterestRate,
            rxegy_sys::XSYMTYP_INTEREST_RATE_INDEX => Self::InterestRateIndex,
            rxegy_sys::XSYMTYP_FUND_L2 => Self::FundL2,
            rxegy_sys::XSYMTYP_WARRANT_L2 => Self::WarrantL2,
            rxegy_sys::XSYMTYP_INDEX_L2 => Self::IndexL2,
            rxegy_sys::XSYMTYP_BOND_L2 => Self::BondL2,
            rxegy_sys::XSYMTYP_FIXED_INCOME_L2 => Self::FixedIncomeL2,
            rxegy_sys::XSYMTYP_COMMODITY_INDEX_L2 => Self::CommodityIndexL2,
            rxegy_sys::XSYMTYP_ETF_L2 => Self::EtfL2,
            rxegy_sys::XSYMTYP_CURRENCY_L2 => Self::CurrencyL2,
            rxegy_sys::XSYMTYP_CURRENCY_INDEX_L2 => Self::CurrencyIndexL2,
            rxegy_sys::XSYMTYP_ETF_INDEX_L2 => Self::EtfIndexL2,
            rxegy_sys::XSYMTYP_FOREIGN_EXCHANGE_INDEX_L2 => Self::ForeignExchangeIndexL2,
            rxegy_sys::XSYMTYP_INTEREST_RATE_L2 => Self::InterestRateL2,
            rxegy_sys::XSYMTYP_INTEREST_RATE_INDEX_L2 => Self::InterestRateIndexL2,
            rxegy_sys::XSYMTYP_FX_SPOT => Self::FxSpot,
            rxegy_sys::XSYMTYP_FX_FORWARD => Self::FxForward,
            rxegy_sys::XSYMTYP_FX_NDF => Self::FxNdf,
            rxegy_sys::XSYMTYP_FX_SWAP => Self::FxSwap,
            _ => Self::Invalid,
        }
    }
}

/// A trade venue, stored as a 4-ascii-character MIC code
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct TradeVenue(XC_TRADE_VENUE);

impl TradeVenue {
    #[inline(always)]
    pub(crate) fn new(inner: XC_TRADE_VENUE) -> Self {
        Self(inner)
    }
}

/// A value suitable for representing a daily traded volume
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Volume(u64);

impl From<u64> for Volume {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Volume(value)
    }
}

impl From<Volume> for u64 {
    #[inline(always)]
    fn from(value: Volume) -> Self {
        value.0
    }
}
