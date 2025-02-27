//! Session Objects

use crate::{
    error::{Error, ExegyError, Result, Success},
    field::{self, Field},
    object::Kind as ObjectKind,
};
use secrecy::{ExposeSecret, SecretString};
use std::{
    any::{Any, TypeId},
    ffi::CString,
    fmt::{Display, Formatter, Result as FmtResult},
    net::{SocketAddr, ToSocketAddrs},
    os::raw::c_void,
    path::{Path, PathBuf},
    pin::Pin,
    ptr::{self, NonNull},
    result::Result as StdResult,
};

/// An enumeration of session object types
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
enum Kind {
    /// The object type is a ticker
    Ticker = ObjectKind::SessionTicker as u16,
    /// The objecdt type is a ticekr monitor
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
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
enum EventKind {
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

/// A callback object containting status event details
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StatusEvent(NonNull<c_void>);

impl AsRef<NonNull<c_void>> for StatusEvent {
    fn as_ref(&self) -> &NonNull<c_void> {
        &self.0
    }
}

impl TryFrom<rxegy_sys::xhandle> for StatusEvent {
    type Error = Error;

    fn try_from(value: rxegy_sys::xhandle) -> StdResult<Self, Self::Error> {
        Ok(StatusEvent(NonNull::new(value).ok_or(Error::NullObject)?))
    }
}

/// An enumeration of session event objects
enum Event {
    /// A status event
    Status(StatusEvent),
}

impl Event {
    pub fn new(event_type: u16, handle: rxegy_sys::xhandle) -> Result<Event> {
        if handle.is_null() {
            return Err(Error::NullObject);
        }

        let kind = EventKind::try_from(event_type)?;
        match kind {
            EventKind::Status => Ok(Event::Status(StatusEvent::try_from(handle)?)),
        }
    }
}

/// A callback object used with sessions
pub trait Callbacks {
    /// A callback to call when a status event has occurred
    fn status(&self, session: &Session, event: &StatusEvent);
}

/// Session objects
pub enum Session {
    /// A ticker plant session
    TickerPlant(NonNull<c_void>),
    /// A monitoring session
    Monitor(NonNull<c_void>),
}

impl AsRef<NonNull<c_void>> for Session {
    fn as_ref(&self) -> &NonNull<c_void> {
        match self {
            Self::TickerPlant(value) => value,
            Self::Monitor(value) => value,
        }
    }
}

impl TryFrom<rxegy_sys::xhandle> for Session {
    type Error = Error;

    fn try_from(value: rxegy_sys::xhandle) -> StdResult<Self, Self::Error> {
        let handle = NonNull::new(value).ok_or(Error::NullObject)?;
        let session_type = field::get_u16(handle, rxegy_sys::XC_SESSION, Field::SessionType)?;
        let kind = Kind::try_from(session_type)?;
        Session::new(kind, handle)
    }
}

impl Session {
    /// Create a new session object
    fn new(kind: Kind, handle: NonNull<c_void>) -> Result<Session> {
        match kind {
            Kind::Ticker => Ok(Session::TickerPlant(handle)),
            Kind::TickerMonitoring => Ok(Session::Monitor(handle)),
        }
    }

    /// Create a new object from the given kind and handle
    fn from_handle(kind: Kind, handle: rxegy_sys::xhandle) -> Result<Session> {
        let handle = NonNull::new(handle).ok_or(Error::NullObject)?;
        Session::new(kind, handle)
    }

    /// Retrieve the turnkey value of this session
    #[allow(dead_code)]
    fn turnkey(&self) -> Result<u64> {
        field::get_u64(*self.as_ref(), rxegy_sys::XC_SESSION, Field::SessionTurnkey)
    }

    /// Retrieve the status of this session.
    ///
    /// The result is built in the following way:
    ///
    /// ```ignored
    /// Result<
    ///     Result<
    ///         Success,    // This will be set if the status is set to a success code
    ///         ExegyError  // This error will be set if the returned status is set to an error code
    ///     >,
    ///     Error  // This error will be set if the status could not be read from the session object
    /// >
    /// ```
    pub fn status(&self) -> Result<StdResult<Success, ExegyError>> {
        Ok(Success::try_from(field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionStatus,
        )?))
    }

    /// Retrieve the client version string
    pub fn client_version(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientVersionString,
        )
    }

    /// Retrieve the client major version
    pub fn client_major_version(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientMajorVersion,
        )
    }

    /// Retrieve the client minor version
    pub fn client_minor_version(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientMinorVersion,
        )
    }

    /// Retrieve the client version revision
    pub fn client_revision(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientRevision,
        )
    }

    /// Retrieve the client version build
    pub fn client_build(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientBuild,
        )
    }

    /// Retrieve the client CPU count.
    pub fn client_cpu_count(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientCpuCount,
        )
    }

    /// Retrieve the affinity mask for the callback thread on this session
    pub fn callback_affinity(&self) -> Result<u64> {
        field::get_u64(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientAffinityMask,
        )
    }

    /// Retrieve the affinity mask for the callback thread on this session
    pub fn callback_priority(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientThreadPriority,
        )
    }

    /// Retrieve CPU affinity mask for the background thread.
    pub fn background_affinity(&self) -> Result<u64> {
        field::get_u64(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientBgThreadAffinityMask,
        )
    }

    /// Retrieve CPU affinity mask for the background thread
    pub fn background_priority(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientBgThreadPriority,
        )
    }

    /// Retrieve CPU affinity mask for the heartbeat thread.
    pub fn heartbeat_affinity(&self) -> Result<u64> {
        field::get_u64(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientHbThreadAffinityMask,
        )
    }

    /// Retrieve priority for the heartbeat thread.
    pub fn hearatbeat_priority(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionClientHbThreadPriority,
        )
    }

    /// Retrieve the name of the server this session is connected to.
    pub fn server_name(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionServerName,
        )
    }

    /// Retrieve the version string of the server this session is connected to.
    pub fn server_version(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionServerVersionString,
        )
    }

    /// Retrieve the major version of the code running on the appliance
    pub fn server_major_version(&self) -> Result<u8> {
        field::get_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionServerMajorVersion,
        )
    }

    /// Retrieve the minor version of the code running on the appliance
    pub fn server_minor_version(&self) -> Result<u8> {
        field::get_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionServerMinorVersion,
        )
    }

    /// Retrieve the software revision (patch) of the code running on the appliance
    pub fn server_revision(&self) -> Result<u8> {
        field::get_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionServerRevision,
        )
    }

    /// Retrieve the build of the code running on the appliance
    pub fn server_build(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionServerBuild,
        )
    }

    /// Retrieve whether reconnection is enabled
    pub fn reconnect_enabled(&self) -> Result<bool> {
        Ok(field::get_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionDisableReconnect,
        )? == 0)
    }

    /// Enable reconnection
    pub fn enable_reconnect(&self) -> Result<()> {
        field::set_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionDisableReconnect,
            false as u8,
        )
    }

    /// Disable reconnection
    pub fn disable_reconnect(&self) -> Result<()> {
        field::set_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionDisableReconnect,
            true as u8,
        )
    }

    /// Get whether the replay has been started
    pub fn replay_start(&self) -> Result<bool> {
        Ok(field::get_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayStart,
        )? == 0)
    }

    /// Start a replay session
    pub fn start_replay(&self) -> Result<()> {
        field::set_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayStart,
            true as u8,
        )
    }

    /// Stop a replay session
    pub fn stop_replay(&self) -> Result<()> {
        field::set_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayStart,
            false as u8,
        )
    }

    /// Retrieve the user-defined quote montage constituents for replay sessions
    pub fn replay_quote_montage(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayQuoteMontage,
        )
    }

    /// Set the user-defined quote montage constituents for replay sessions
    pub fn set_replay_quote_montage(&self, montage: String) -> Result<()> {
        field::set_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayQuoteMontage,
            montage,
        )
    }

    /// Retrieve the user-defined consolidated price book constituents for replay sessions
    pub fn replay_l2_composite(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayL2Composite,
        )
    }

    /// Set the user-defined consolidated price book constituents for replay sessions
    pub fn set_replay_l2_composite(&self, composite: String) -> Result<()> {
        field::set_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayL2Composite,
            composite,
        )
    }

    /// Retrieve the exchange constituents for user-defined BBO on replay sessions
    pub fn replay_ubbo(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayUbbo,
        )
    }

    /// Set the exchange constituents for user-defined BBO on replay sessions.
    pub fn set_replay_ubbo(&self, ubbo: String) -> Result<()> {
        field::set_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionReplayUbbo,
            ubbo,
        )
    }

    /// Retrieve the maximum depth of price-book containers created on this session.
    pub fn max_pricebook_depth(&self) -> Result<u16> {
        field::get_u16(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionTickerMaxPriceBookDepth,
        )
    }

    /// Retrieve the maximum number of rows for price-book update events.
    pub fn max_pricebook_row_level(&self) -> Result<u16> {
        field::get_u16(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::SessionTickerMaxPriceBookRowLevel,
        )
    }
}

/// A representation of all the various address types supported by XCAPI
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Server {
    Xti(PathBuf),
    Infiniband(String),
    RoCE(String),
    Ip(SocketAddr),
}

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Xti(pathbuf) => write!(
                f,
                "{}",
                pathbuf
                    .canonicalize()
                    .expect("Could not canonicalize XTI path")
                    .to_str()
                    .expect("Received a path with non-UTF-8 characters?")
            ),
            Self::Infiniband(ib) => write!(f, "ib::{}", ib),
            Self::RoCE(roce) => write!(f, "roce::{}", roce),
            Self::Ip(sockaddr) => write!(f, "{}", sockaddr),
        }
    }
}

/// A session builder
#[derive(Default)]
pub struct Builder {
    server_list: Vec<Server>,
    username: String,
    password: SecretString,
    cb_affinity: Option<u64>,
    cb_priority: Option<u8>,
    bg_affinity: Option<u64>,
    bg_priority: Option<u8>,
    hb_affinity: Option<u64>,
    hb_priority: Option<u8>,
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

    /// Set the CPU affinity mask for the callback thread to the given value.
    pub fn affinity(&mut self, mask: u64) -> &mut Self {
        if mask == 0 {
            self.cb_affinity = None;
        } else {
            self.cb_affinity = Some(mask);
        }
        self
    }

    /// Set the thread priority for the session's callbacks.
    pub fn priority(&mut self, priority: u8) -> &mut Self {
        if priority == 0 {
            self.cb_priority = None;
        } else {
            self.cb_priority = Some(priority);
        }

        self
    }

    /// Set the CPU affinity mask for the background processing thread.
    pub fn background_affinity(&mut self, mask: u64) -> &mut Self {
        if mask == 0 {
            self.bg_affinity = None
        } else {
            self.bg_affinity = Some(mask);
        }
        self
    }

    /// Set the priority for this session's background thread.
    pub fn background_priority(&mut self, priority: u8) -> &mut Self {
        if priority == 0 {
            self.bg_priority = None;
        } else {
            self.bg_priority = Some(priority);
        }
        self
    }

    /// Set the CPU affinity mask for the heartbeat thread.
    pub fn heartbeat_affinity(&mut self, mask: u64) -> &mut Self {
        if mask == 0 {
            self.hb_affinity = None;
        } else {
            self.hb_affinity = Some(mask);
        }
        self
    }

    /// Set the priority for this session's heartbeat thread.
    pub fn heartbeat_priority(&mut self, priority: u8) -> &mut Self {
        if priority == 0 {
            self.hb_priority = None;
        } else {
            self.hb_priority = Some(priority);
        }
        self
    }

    /// Connect to the Exegy appliance and return a ticker plant session.
    pub fn tickerplant(
        self,
        market_events_per_instrument: bool,
        callbacks: Box<dyn Callbacks>,
    ) -> Result<Session> {
        self.start_session(Kind::Ticker, Some(market_events_per_instrument), callbacks)
    }

    /// Connect to the Exegy appliance and return a monitoring session.
    pub fn monitor(self, callbacks: Box<dyn Callbacks>) -> Result<Session> {
        self.start_session(Kind::TickerMonitoring, None, callbacks)
    }

    /// Actually build a session object
    fn start_session(
        self,
        kind: Kind,
        market_events_per_instrument: Option<bool>,
        callbacks: Box<dyn Callbacks>,
    ) -> Result<Session> {
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

        // Make our session context object (used to dispatch callbacks)
        let mut context = Box::pin(Context {
            callbacks,
            affinity: self.cb_affinity,
            priority: self.cb_priority.map(|v| v as u32),
            market_events_per_instrument,
        });

        // Next, we create a trait object around a reference to our arc mutex
        let anyref = &mut context as &mut dyn Any;
        // Then we box the trait object
        let boxed = Box::new(anyref);
        // Finally, we cast the raw boxed pointer to the u64 that exegy requires as a "turnkey",
        // or user-data pointer
        let turnkey = Box::into_raw(boxed) as u64;

        let retval = unsafe {
            let mut handle = ptr::null_mut();
            let status = rxegy_sys::xcCreateSession(
                kind as u16,
                &mut handle,
                Some(_rxegy_session_callback),
                turnkey,
                server_list.as_ptr(),
                username.as_ptr(),
                password.as_ptr(),
            );
            Success::try_from(status)?;
            Session::from_handle(kind, handle)?
        };

        if let Some(affin) = self.bg_affinity {
            field::set_u64(
                *retval.as_ref(),
                rxegy_sys::XC_SESSION,
                Field::SessionClientBgThreadAffinityMask,
                affin,
            )?;
        }

        if let Some(prio) = self.bg_priority {
            field::set_u32(
                *retval.as_ref(),
                rxegy_sys::XC_SESSION,
                Field::SessionClientBgThreadPriority,
                prio as u32,
            )?;
        }

        if let Some(affin) = self.hb_affinity {
            field::set_u64(
                *retval.as_ref(),
                rxegy_sys::XC_SESSION,
                Field::SessionClientHbThreadAffinityMask,
                affin,
            )?;
        }

        if let Some(prio) = self.hb_priority {
            field::set_u32(
                *retval.as_ref(),
                rxegy_sys::XC_SESSION,
                Field::SessionClientHbThreadPriority,
                prio as u32,
            )?;
        }

        Ok(retval)
    }
}

struct Context {
    /// An object which we'll fire callbacks on
    callbacks: Box<dyn Callbacks>,
    /// The CPU affinity mask to be set when the callback is fired
    affinity: Option<u64>,
    /// The thread proiority to be set when the callback is fired
    priority: Option<u32>,
    /// Whether to fire market event callbacks per instrument
    market_events_per_instrument: Option<bool>,
}

impl Context {
    fn dispatch(
        &self,
        handle: rxegy_sys::xhandle,
        event_handle: rxegy_sys::xhandle,
        event_type: u16,
        status: u32,
    ) -> Result<()> {
        // Grab the session handle
        let session = Session::try_from(handle)?;

        // Check the event status
        if Success::try_from(status).is_ok() {
            if let Some(affinity) = self.affinity {
                field::set_u64(
                    *session.as_ref(),
                    rxegy_sys::XC_SESSION,
                    Field::SessionClientAffinityMask,
                    affinity,
                )?;
            }

            if let Some(prio) = self.priority {
                field::set_u32(
                    *session.as_ref(),
                    rxegy_sys::XC_SESSION,
                    Field::SessionClientThreadPriority,
                    prio,
                )?;
            }

            if let Some(enable) = self.market_events_per_instrument {
                field::set_u8(
                    *session.as_ref(),
                    rxegy_sys::XC_SESSION,
                    Field::SessionTickerMarketStatusCallbacks,
                    enable as u8,
                )?;
            }
        }

        match Event::new(event_type, event_handle)? {
            Event::Status(status_event) => (*self.callbacks).status(&session, &status_event),
        };

        Ok(())
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _rxegy_session_callback(
    handle: rxegy_sys::xhandle,
    _slot: u32,
    event_handle: rxegy_sys::xhandle,
    event_type: u16,
    turnkey: u64,
    status: rxegy_sys::xerr,
) {
    // TODO: log panics
    let _ = std::panic::catch_unwind(|| {
        // Get our context
        let boxed = unsafe {
            let ptr = turnkey as *mut &mut dyn Any;
            Box::from_raw(ptr)
        };

        if boxed.type_id() != TypeId::of::<Pin<Box<Context>>>() {
            // TODO: Log this error
            return;
        }

        let context = (*boxed).downcast_ref::<Pin<Box<Context>>>();

        if context.is_none() {
            // TODO: Log this error
            return;
        }

        if let Err(_error) = context
            .unwrap()
            .dispatch(handle, event_handle, event_type, status)
        {
            // TODO: Log the error.
        }
    });
}
