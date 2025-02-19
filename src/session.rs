//! Session Objects

use std::{fmt::Debug, net::IpAddr, path::{Path, PathBuf}, boxed::Box};
use crate::{object::Kind as ObjectKind, error::Result};
use rexegy_sys::xhandle;

// turnkey = Rc<SessionDetails> pointer
// Session details contains rust callback function
// For each instance, session will be re-created
// How to handle arbitrary user data...
// rust any type?

/// A session object.
#[derive(Debug)]
pub struct Session {
    handle: Option<Box<xhandle>>,
    user_data: Box<dyn SessionCallbacks>,
}

impl Session {
    fn new(handle: Option<Box<xhandle>>, user_data: Box<dyn SessionCallbacks>) -> Self {
        Self {
            handle,
            user_data,
        }
    }
}

/// A session builder.
#[derive(Debug)]
pub struct SessionBuilder {
    kind: Kind,
    turnkey: u64,
    server_list: Vec<Server>,
    password: String,
    username: String,
    user_data: Box<dyn SessionCallbacks>,
}

impl SessionBuilder {

    /// Attach user data to the resulting session
    pub fn user_data(&mut self, user_data: Box<dyn SessionCallbacks>) -> &mut Self {
        self.user_data = user_data;
        self
    }

    /// Set the type of session to create
    pub fn kind(&mut self, kind: Kind) -> &mut Self {
        self.kind = kind;
        self
    }

    /// Add an infiniband server address to the list data sources this session will read from.
    pub fn add_ib<S: ToString>(&mut self, ib_addr: S) -> &mut Self {
        self.server_list.push(Server::new_ib(ib_addr));
        self
    }

    /// Add an "XTI" playback file to the list of data sources this session will read from.
    pub fn add_xti(&mut self, xti: &Path) -> &mut Self {
        self.server_list.push(Server::new_path(xti));
        self
    }

    /// Add an IP/port pair to the list of data sources this session will read data from.
    pub fn add_ip(&mut self, ip: IpAddr, port: u16) -> &mut Self {
        self.server_list.push(Server::new_ip(ip, port));
        self
    }

    /// Set the uername we're going to use when connecting to our data sources
    pub fn username<S: ToString>(&mut self, username: S) -> &mut Self {
        self.username = username.to_string();
        self
    }

    /// Set the password we're going to use when connecting to our data sources
    pub fn password<S: ToString>(&mut self, password: S) -> &mut Self {
        self.password = password.to_string();
        self
    }

    /// Construct the session and consume the builder.
    pub fn build(mut self) -> Result<Session> {
        let handle = Box::<xhandle>::new_uninit();
        unsafe {
            let xerr = rexegy_sys::xcCreateSession(self.kind as u16, handle.as_mut_ptr(), 
                _rexegy_session_callback, self.user_data.as_mut(), server_list, username, password);
        }

        let inner = Box::from(handle.as_mut_ptr());
        Session(inner)
    }
}

#[no_mangle]
unsafe extern "C" fn _rexegy_session_callback(
    session: rexegy_sys::xhandle,
    slot: rexegy_sys::xuint32,
    event: rexegy_sys::xhandle,
    object_type: rexegy_sys::XC_OBJECT_TYPE,
    turnkey: rexegy_sys::xuint64,
    status: rexegy_sys::xerr,
) {
    //
}

#[derive(Debug)]
enum Server {
    Infiniband(Ib),
    RoCE(RoCE),
    Ip(Ip),
    Path(PathBuf),
}

impl Server {
    pub fn new_ib<T: ToString>(ib: T) -> Self {
        Server::Infiniband(Ib::new(ib))
    }

    pub fn new_roce<S: ToString>(roce: S) -> Self {
        Server::RoCE(RoCE::new(roce))
    }

    pub fn new_ip(ip: IpAddr, port: u16) -> Self {
        Server::Ip(Ip::new(ip, port))
    }

    pub fn new_path(path: &Path) -> Self {
        Server::Path(PathBuf::from(path))
    }
}

/// An enumeration of session object types
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Kind {
    Invalid = ObjectKind::Invalid as u16,
    Ticker = ObjectKind::SessionTicker as u16,
    TickerMonitoring = ObjectKind::SessionTickerMonitoring as u16,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Invalid
    }
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Ib(String);

impl Ib {
    pub fn new<S: ToString>(ib: S) -> Self {
        Ib(ib.to_string())
    }
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct RoCE(String);

impl RoCE {
    pub fn new<S: ToString>(roce: S) -> Self {
        RoCE(roce.to_string())
    }
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Ip {
    addr: IpAddr,
    port: u16,
}

impl Ip {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            addr,
            port,
        }
    }
}

pub trait SessionCallbacks: Debug {
    fn controller_created(&mut self, session: Session);
    fn session_status(&mut self, session: Session);
}