//! Session Objects

use std::{ffi::CString, net::IpAddr, path::PathBuf};

use rxegy_sys::xhandle;

use crate::object::Kind as ObjectKind;

/// An enumeration of session object types
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Kind {
    Invalid = ObjectKind::Invalid as u16,
    Ticker = ObjectKind::SessionTicker as u16,
    TickerMonitoring = ObjectKind::SessionTickerMonitoring as u16,
}

/// An enumeration of callback event types
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum EventKind {
    Status = ObjectKind::EventSessionStatus as u16,
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StatusEvent(xhandle);

/// A callback object
pub trait SessionCallbacks {
    fn status(&self, session: &Session, event: &StatusEvent);
}

/// A session object
pub struct Session {
    handle: xhandle,
    callbacks: Box<dyn SessionCallbacks>,
}

/// A session builder
pub struct Builder {
    session: Session,
    server_list: Vec<Server>,
    username: String,
    password: String,
}

enum Server {
    File(PathBuf),
    RoCE(RoCE),
    Ip(Ip),
}

struct RoCE(String);

struct Ip {
    ip: IpAddr,
    port: u16,
}
