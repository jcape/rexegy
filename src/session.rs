//! Session Objects

use crate::{
    error::{Error, Result, Success},
    object::Kind as ObjectKind,
};
use rxegy_sys::{xcCreateSession, xhandle};
use secrecy::{ExposeSecret, SecretString};
use std::{
    any::{Any, TypeId},
    ffi::CString,
    net::{SocketAddr, ToSocketAddrs},
    path::{Path, PathBuf},
    ptr,
    result::Result as StdResult,
    sync::{Arc, Mutex},
};

/// An enumeration of session object types
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Kind {
    Ticker = ObjectKind::SessionTicker as u16,
    TickerMonitoring = ObjectKind::SessionTickerMonitoring as u16,
}

impl TryFrom<u16> for Kind {
    type Error = Error;

    fn try_from(value: u16) -> StdResult<Self, Self::Error> {
        match value {
            rxegy_sys::XOBJ_SESSION_TICKER => Ok(Kind::Ticker),
            rxegy_sys::XOBJ_SESSION_TICKER_MONITORING => Ok(Kind::TickerMonitoring),
            _ => Err(Error::ObjectUnknown),
        }
    }
}

/// An enumeration of callback event types
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum EventKind {
    Status = ObjectKind::EventSessionStatus as u16,
}

impl TryFrom<u16> for EventKind {
    type Error = Error;

    fn try_from(value: u16) -> StdResult<Self, Self::Error> {
        match value {
            rxegy_sys::XOBJ_SESSION_TICKER => Ok(EventKind::Status),
            _ => Err(Error::ObjectUnknown),
        }
    }
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StatusEvent(xhandle);

/// A callback object
pub trait SessionCallbacks {
    /// The session status callback
    fn status(&self, session: &Session, event: &StatusEvent);
}

/// A session object
pub struct Session {
    handle: xhandle,
    callbacks: Box<dyn SessionCallbacks>,
}

/// A session builder
pub struct Builder {
    server_list: Vec<Server>,
    username: String,
    password: SecretString,
    callbacks: Box<dyn SessionCallbacks>,
}

impl Builder {
    /// Set the username to use when connecting to this session.
    pub fn username<U: ToString>(&mut self, username: &U) -> &mut Self {
        self.username = username.to_string();
        self
    }

    /// Set the password to use when connecting to this session.
    pub fn password(&mut self, password: SecretString) -> &mut Self {
        self.password = password;
        self
    }

    /// Add an XTI file to inject into this session.
    pub fn add_xti(&mut self, xti: &Path) -> &mut Self {
        self.server_list.push(Server::Xti(xti.to_owned()));
        self
    }

    /// Add an RoCE1/Infiniband address to connect to.
    pub fn add_ib<I: ToString>(&mut self, ib: &I) -> &mut Self {
        self.server_list.push(Server::Infiniband(ib.to_string()));
        self
    }

    /// Add a RoCE2 address to connect to.
    pub fn add_roce<R: ToString>(&mut self, roce: &R) -> &mut Self {
        self.server_list.push(Server::RoCE(roce.to_string()));
        self
    }

    /// Add a server to connect to
    pub fn add_server<S: ToSocketAddrs>(&mut self, server: &S) -> Result<&mut Self> {
        server
            .to_socket_addrs()?
            .for_each(|addr| self.server_list.push(Server::Ip(addr)));
        Ok(self)
    }

    /// Set the object which will handle session callbacks
    pub fn callbacks(&mut self, callbacks: Box<dyn SessionCallbacks>) -> &mut Self {
        self.callbacks = callbacks;
        self
    }

    /// Connect to the Exegy appliance and return the session object.
    pub fn build(self) -> Result<Arc<Mutex<Session>>> {
        // Build our parameters
        let server_list = CString::new(
            self.server_list
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
        )?;
        let username = CString::new(self.username)?;
        let password = CString::new(self.password.expose_secret())?;

        // Make our session object in an Arc mutex
        let mut retval = Arc::new(Mutex::new(Session {
            handle: ptr::null_mut(),
            callbacks: self.callbacks,
        }));

        // Next, we create a trait object around a reference to our arc mutex
        let anyref = &mut retval as &mut dyn Any;
        // Then we box the trait object
        let boxed = Box::new(anyref);
        // Finally, we cast the raw boxed pointer to the u64 that exegy requires as a "turnkey",
        // or user-data pointer
        let turnkey = Box::into_raw(boxed) as u64;

        unsafe {
            let status = xcCreateSession(
                Kind::Ticker as u16,
                // FIXME: don't just panic, maybe hold a lock throughout create session?
                &mut retval
                    .lock()
                    .expect("Could not acquire lock on just-created mutex?")
                    .handle,
                Some(_rxegy_session_callback),
                turnkey,
                server_list.as_ptr(),
                username.as_ptr(),
                password.as_ptr(),
            );
            Success::try_from(status)?;
        }

        Ok(retval.clone())
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _rxegy_session_callback(
    _handle: xhandle,
    _slot: u32,
    event_handle: xhandle,
    event_type: u16,
    turnkey: u64,
    status: rxegy_sys::xerr,
) {
    let _ = std::panic::catch_unwind(|| {
        // Check the status
        let _result = Success::try_from(status);

        // Check event kind
        let _kind = EventKind::try_from(event_type).expect("Unknown event type received");

        // Get our event object
        let event = StatusEvent(event_handle);

        // Get our session
        let boxed;

        unsafe {
            let ptr = turnkey as *mut &mut dyn Any;
            boxed = Box::from_raw(ptr);
        }

        if boxed.type_id() != TypeId::of::<Arc<Mutex<Session>>>() {
            // FIXME: don't just panic.
            panic!("Got something other than an Arc<Mutex<Session>> in _rxegy_session_callback");
        }

        // FIXME: don't panic here either (requires logging framework integration)
        let mutex = (*boxed)
            .downcast_ref::<Arc<Mutex<Session>>>()
            .expect("Couldn't downcast turnkey to an Arc<Session>")
            .clone();

        let session = mutex
            .lock()
            .expect("Could not acquire a lock on our session");
        (*session.callbacks).status(&session, &event);
    });
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Server {
    Xti(PathBuf),
    Infiniband(String),
    RoCE(String),
    Ip(SocketAddr),
}

impl ToString for Server {
    fn to_string(&self) -> String {
        match self {
            Self::Xti(pathbuf) => pathbuf
                .canonicalize()
                .expect("Could not canonicalize XTI path")
                .to_str()
                .expect("Received a path with non-UTF-8 characters?")
                .to_string(),
            Self::Infiniband(ib) => {
                let mut retval = "ib::".to_string();
                retval.push_str(ib);
                retval
            }
            Self::RoCE(roce) => {
                let mut retval = "roce::".to_string();
                retval.push_str(roce);
                retval
            }
            Self::Ip(sockaddr) => sockaddr.to_string(),
        }
    }
}
