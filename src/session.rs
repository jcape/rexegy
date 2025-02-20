//! Session Objects

use crate::object::Kind as ObjectKind;

/// An enumeration of session object types
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Kind {
    Invalid = ObjectKind::Invalid as u16,
    Ticker = ObjectKind::SessionTicker as u16,
    TickerMonitoring = ObjectKind::SessionTickerMonitoring as u16,
}
