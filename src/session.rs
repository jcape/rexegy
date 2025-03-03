//! Session Objects

use crate::{
    error::{Error, ExegyError, Result, Success},
    event::CommonEvent,
    field::{self, Field as FieldTrait},
    object::Kind as ObjectKind,
};
use rxegy_sys::{xerr, xhandle};
use secrecy::{ExposeSecret, SecretString};
use std::{
    ffi::CString,
    fmt::{Display, Formatter, Result as FmtResult},
    net::{SocketAddr, ToSocketAddrs},
    os::raw::c_void,
    path::{Path, PathBuf},
    ptr::{self, NonNull},
    result::Result as StdResult,
    sync::Arc,
};

/// An enumeration of session object types
#[derive(Copy, Clone, Debug, displaydoc::Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
enum Kind {
    /// Ticker Plant
    Ticker = ObjectKind::SessionTicker as u16,
    /// Monitoring
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
    /// Session Status Event
    Status = ObjectKind::EventSessionStatus as u16,
}

impl TryFrom<u16> for EventKind {
    type Error = Error;

    fn try_from(value: u16) -> StdResult<Self, Self::Error> {
        match value {
            rxegy_sys::XOBJ_EVENT_SESSION_STATUS => Ok(EventKind::Status),
            _ => Err(Error::ObjectUnknown),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
enum Field {
    Turnkey = rxegy_sys::XFLD_SESS_TURNKEY,
    Status = rxegy_sys::XFLD_SESS_STATUS,
    Type = rxegy_sys::XFLD_SESS_SESSION_TYPE,
    ClientVersionString = rxegy_sys::XFLD_SESS_CLIENT_VERSION_STRING,
    ClientMajorVersion = rxegy_sys::XFLD_SESS_CLIENT_MAJOR_VERSION,
    ClientMinorVersion = rxegy_sys::XFLD_SESS_CLIENT_MINOR_VERSION,
    ClientRevision = rxegy_sys::XFLD_SESS_CLIENT_REVISION,
    ClientBuild = rxegy_sys::XFLD_SESS_CLIENT_BUILD,
    ClientCpuCount = rxegy_sys::XFLD_SESS_CLIENT_CPU_COUNT,
    ClientAffinityMask = rxegy_sys::XFLD_SESS_CLIENT_AFFINITY_MASK,
    ClientBgThreadAffinityMask = rxegy_sys::XFLD_SESS_CLIENT_BG_THREAD_AFFINITY_MASK,
    ClientHbThreadAffinityMask = rxegy_sys::XFLD_SESS_CLIENT_HB_THREAD_AFFINITY_MASK,
    ClientThreadPriority = rxegy_sys::XFLD_SESS_CLIENT_THREAD_PRIORITY,
    ClientBgThreadPriority = rxegy_sys::XFLD_SESS_CLIENT_BG_THREAD_PRIORITY,
    ClientHbThreadPriority = rxegy_sys::XFLD_SESS_CLIENT_HB_THREAD_PRIORITY,
    ServerName = rxegy_sys::XFLD_SESS_SERVER_NAME,
    ServerVersionString = rxegy_sys::XFLD_SESS_SERVER_VERSION_STRING,
    ServerMajorVersion = rxegy_sys::XFLD_SESS_SERVER_MAJOR_VERSION,
    ServerMinorVersion = rxegy_sys::XFLD_SESS_SERVER_MINOR_VERSION,
    ServerRevision = rxegy_sys::XFLD_SESS_SERVER_REVISION,
    ServerBuild = rxegy_sys::XFLD_SESS_SERVER_BUILD,
    DisableReconnect = rxegy_sys::XFLD_SESS_DISABLE_RECONNECT,
    ReplayStart = rxegy_sys::XFLD_SESS_REPLAY_START,
    ReplayQuoteMontage = rxegy_sys::XFLD_SESS_REPLAY_QUOTE_MONTAGE,
    ReplayL2Composite = rxegy_sys::XFLD_SESS_REPLAY_L2_COMPOSITE,
    ReplayUbbo = rxegy_sys::XFLD_SESS_REPLAY_UBBO,
    TickerMaxPriceBookDepth = rxegy_sys::XFLD_SESS_TKR_MAX_PRICE_BOOK_DEPTH,
    TickerMarketStatusCallbacks = rxegy_sys::XFLD_SESS_TKR_MARKET_STATUS_CALLBACKS,
    TickerMaxPriceBookRowLevel = rxegy_sys::XFLD_SESS_TKR_MAX_PB_ROW_LEVEL,
}

impl FieldTrait for Field {
    fn to_u64(&self) -> u64 {
        *self as u64
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

impl CommonEvent for StatusEvent {}

/// An enumeration of session event objects
enum Event {
    /// A status event
    Status(StatusEvent),
}

impl Event {
    pub fn new(event_type: u16, handle: xhandle) -> Result<Event> {
        tracing::trace!(event.type = event_type, "Trying to find event kind");

        let obj = NonNull::new(handle).ok_or(Error::NullObject)?;
        let kind = EventKind::try_from(event_type)?;

        tracing::trace!(event.kind = format!("{:?}", kind), "Event kind found!");

        match kind {
            EventKind::Status => Ok(Event::Status(StatusEvent(obj))),
        }
    }
}

/// A callback object used with sessions
pub trait Callbacks {
    /// A callback to call when a status event has occurred
    fn status(&self, session: &Session, event: &StatusEvent);
}

/// Session objects
#[derive(Debug)]
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

impl TryFrom<xhandle> for Session {
    type Error = Error;

    fn try_from(value: xhandle) -> StdResult<Self, Self::Error> {
        let handle = NonNull::new(value).ok_or(Error::NullObject)?;
        let session_type = field::get_u16(handle, rxegy_sys::XC_SESSION, Field::Type)?;
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
    fn from_handle(kind: Kind, handle: xhandle) -> Result<Session> {
        let handle = NonNull::new(handle).ok_or(Error::NullObject)?;
        Session::new(kind, handle)
    }

    /// Retrieve the turnkey value of this session
    #[allow(dead_code)]
    fn turnkey(&self) -> Result<u64> {
        field::get_u64(*self.as_ref(), rxegy_sys::XC_SESSION, Field::Turnkey)
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
            Field::Status,
        )?))
    }

    /// Retrieve the client version string
    pub fn client_version(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientVersionString,
        )
    }

    /// Retrieve the client major version
    pub fn client_major_version(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientMajorVersion,
        )
    }

    /// Retrieve the client minor version
    pub fn client_minor_version(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientMinorVersion,
        )
    }

    /// Retrieve the client version revision
    pub fn client_revision(&self) -> Result<u32> {
        field::get_u32(*self.as_ref(), rxegy_sys::XC_SESSION, Field::ClientRevision)
    }

    /// Retrieve the client version build
    pub fn client_build(&self) -> Result<u32> {
        field::get_u32(*self.as_ref(), rxegy_sys::XC_SESSION, Field::ClientBuild)
    }

    /// Retrieve the client CPU count.
    pub fn client_cpu_count(&self) -> Result<u32> {
        field::get_u32(*self.as_ref(), rxegy_sys::XC_SESSION, Field::ClientCpuCount)
    }

    /// Retrieve the affinity mask for the callback thread on this session
    pub fn callback_affinity(&self) -> Result<u64> {
        field::get_u64(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientAffinityMask,
        )
    }

    /// Retrieve the affinity mask for the callback thread on this session
    pub fn callback_priority(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientThreadPriority,
        )
    }

    /// Retrieve CPU affinity mask for the background thread.
    pub fn background_affinity(&self) -> Result<u64> {
        field::get_u64(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientBgThreadAffinityMask,
        )
    }

    /// Retrieve CPU affinity mask for the background thread
    pub fn background_priority(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientBgThreadPriority,
        )
    }

    /// Retrieve CPU affinity mask for the timing thread.
    pub fn timing_affinity(&self) -> Result<u64> {
        field::get_u64(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientHbThreadAffinityMask,
        )
    }

    /// Retrieve priority for the timing thread.
    pub fn hearatbeat_priority(&self) -> Result<u32> {
        field::get_u32(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ClientHbThreadPriority,
        )
    }

    /// Retrieve the name of the server this session is connected to.
    pub fn server_name(&self) -> Result<String> {
        field::get_string(*self.as_ref(), rxegy_sys::XC_SESSION, Field::ServerName)
    }

    /// Retrieve the version string of the server this session is connected to.
    pub fn server_version(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ServerVersionString,
        )
    }

    /// Retrieve the major version of the code running on the appliance
    pub fn server_major_version(&self) -> Result<u8> {
        field::get_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ServerMajorVersion,
        )
    }

    /// Retrieve the minor version of the code running on the appliance
    pub fn server_minor_version(&self) -> Result<u8> {
        field::get_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ServerMinorVersion,
        )
    }

    /// Retrieve the software revision (patch) of the code running on the appliance
    pub fn server_revision(&self) -> Result<u8> {
        field::get_u8(*self.as_ref(), rxegy_sys::XC_SESSION, Field::ServerRevision)
    }

    /// Retrieve the build of the code running on the appliance
    pub fn server_build(&self) -> Result<u32> {
        field::get_u32(*self.as_ref(), rxegy_sys::XC_SESSION, Field::ServerBuild)
    }

    /// Retrieve whether reconnection is enabled
    pub fn reconnect_enabled(&self) -> Result<bool> {
        Ok(field::get_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::DisableReconnect,
        )? == 0)
    }

    /// Enable reconnection
    pub fn enable_reconnect(&self) -> Result<()> {
        field::set_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::DisableReconnect,
            false as u8,
        )
    }

    /// Disable reconnection
    pub fn disable_reconnect(&self) -> Result<()> {
        field::set_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::DisableReconnect,
            true as u8,
        )
    }

    /// Get whether the replay has been started
    pub fn replay_start(&self) -> Result<bool> {
        Ok(field::get_u8(*self.as_ref(), rxegy_sys::XC_SESSION, Field::ReplayStart)? == 0)
    }

    /// Start a replay session
    pub fn start_replay(&self) -> Result<()> {
        field::set_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ReplayStart,
            true as u8,
        )
    }

    /// Stop a replay session
    pub fn stop_replay(&self) -> Result<()> {
        field::set_u8(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ReplayStart,
            false as u8,
        )
    }

    /// Retrieve the user-defined quote montage constituents for replay sessions
    pub fn replay_quote_montage(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ReplayQuoteMontage,
        )
    }

    /// Set the user-defined quote montage constituents for replay sessions
    pub fn set_replay_quote_montage(&self, montage: String) -> Result<()> {
        field::set_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ReplayQuoteMontage,
            montage,
        )
    }

    /// Retrieve the user-defined consolidated price book constituents for replay sessions
    pub fn replay_l2_composite(&self) -> Result<String> {
        field::get_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ReplayL2Composite,
        )
    }

    /// Set the user-defined consolidated price book constituents for replay sessions
    pub fn set_replay_l2_composite(&self, composite: String) -> Result<()> {
        field::set_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ReplayL2Composite,
            composite,
        )
    }

    /// Retrieve the exchange constituents for user-defined BBO on replay sessions
    pub fn replay_ubbo(&self) -> Result<String> {
        field::get_string(*self.as_ref(), rxegy_sys::XC_SESSION, Field::ReplayUbbo)
    }

    /// Set the exchange constituents for user-defined BBO on replay sessions.
    pub fn set_replay_ubbo(&self, ubbo: String) -> Result<()> {
        field::set_string(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::ReplayUbbo,
            ubbo,
        )
    }

    /// Retrieve the maximum depth of price-book containers created on this session.
    pub fn max_pricebook_depth(&self) -> Result<u16> {
        field::get_u16(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::TickerMaxPriceBookDepth,
        )
    }

    /// Retrieve the maximum number of rows for price-book update events.
    pub fn max_pricebook_row_level(&self) -> Result<u16> {
        field::get_u16(
            *self.as_ref(),
            rxegy_sys::XC_SESSION,
            Field::TickerMaxPriceBookRowLevel,
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
    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    /// Set the password to use when connecting to this session.
    pub fn password(mut self, password: &SecretString) -> Self {
        self.password = password.clone();
        self
    }

    /// Add an XTI file to inject into this session.
    pub fn add_xti(mut self, xti: &Path) -> Self {
        self.server_list.push(Server::Xti(xti.to_owned()));
        self
    }

    /// Add an RoCE1/Infiniband address to connect to.
    pub fn add_ib<I: ToString>(mut self, ib: &I) -> Self {
        self.server_list.push(Server::Infiniband(ib.to_string()));
        self
    }

    /// Add a RoCE2 address to connect to.
    pub fn add_roce<R: ToString>(mut self, roce: &R) -> Self {
        self.server_list.push(Server::RoCE(roce.to_string()));
        self
    }

    /// Add a server to connect to
    pub fn add_server(mut self, server: &str) -> Result<Self> {
        server
            .to_socket_addrs()?
            .for_each(|addr| self.server_list.push(Server::Ip(addr)));
        Ok(self)
    }

    /// Set the CPU affinity mask for the callback thread to the given value.
    pub fn callback_affinity(mut self, affinity: Option<u64>) -> Self {
        self.cb_affinity = affinity;
        self
    }

    /// Set the thread priority for the session's callbacks.
    pub fn callback_priority(mut self, priority: Option<u8>) -> Self {
        self.cb_priority = priority;
        self
    }

    /// Set the CPU affinity mask for the background processing thread.
    pub fn background_affinity(mut self, affinity: Option<u64>) -> Self {
        self.bg_affinity = affinity;
        self
    }

    /// Set the priority for this session's background thread.
    pub fn background_priority(mut self, priority: Option<u8>) -> Self {
        self.bg_priority = priority;
        self
    }

    /// Set the CPU affinity mask for the timing thread.
    pub fn timing_affinity(mut self, affinity: Option<u64>) -> Self {
        self.hb_affinity = affinity;
        self
    }

    /// Set the priority for this session's timing thread.
    pub fn timing_priority(mut self, priority: Option<u8>) -> Self {
        self.hb_priority = priority;
        self
    }

    /// Connect to the Exegy appliance and return a ticker plant session.
    pub fn tickerplant(
        self,
        market_events_per_instrument: bool,
        callbacks: Arc<dyn Callbacks>,
    ) -> Result<Session> {
        tracing::trace!("Starting tickerplant session");
        self.start_session(Kind::Ticker, Some(market_events_per_instrument), callbacks)
    }

    /// Connect to the Exegy appliance and return a monitoring session.
    pub fn monitor(self, callbacks: Arc<dyn Callbacks>) -> Result<Session> {
        tracing::trace!("Starting monitoring session");
        self.start_session(Kind::TickerMonitoring, None, callbacks)
    }

    /// Actually build a session object
    fn start_session(
        self,
        kind: Kind,
        market_events_per_instrument: Option<bool>,
        callbacks: Arc<dyn Callbacks>,
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
        let context = Box::new(Context {
            callbacks,
            affinity: self.cb_affinity,
            priority: self.cb_priority.map(|v| v as u32),
            market_events_per_instrument,
        });

        let turnkey = Box::into_raw(context) as u64;

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
                Field::ClientBgThreadAffinityMask,
                affin,
            )?;
        }

        if let Some(prio) = self.bg_priority {
            field::set_u32(
                *retval.as_ref(),
                rxegy_sys::XC_SESSION,
                Field::ClientBgThreadPriority,
                prio as u32,
            )?;
        }

        if let Some(affin) = self.hb_affinity {
            field::set_u64(
                *retval.as_ref(),
                rxegy_sys::XC_SESSION,
                Field::ClientHbThreadAffinityMask,
                affin,
            )?;
        }

        if let Some(prio) = self.hb_priority {
            field::set_u32(
                *retval.as_ref(),
                rxegy_sys::XC_SESSION,
                Field::ClientHbThreadPriority,
                prio as u32,
            )?;
        }

        Ok(retval)
    }
}

struct Context {
    /// An object which we'll fire callbacks on
    callbacks: Arc<dyn Callbacks>,
    /// The CPU affinity mask to be set when the callback is fired
    affinity: Option<u64>,
    /// The thread proiority to be set when the callback is fired
    priority: Option<u32>,
    /// Whether to fire market event callbacks per instrument
    market_events_per_instrument: Option<bool>,
}

impl Drop for Context {
    fn drop(&mut self) {
        tracing::error!("Drop called on context, this shouldn't happen.")
    }
}

impl Context {
    fn dispatch(
        &self,
        handle: xhandle,
        event_handle: xhandle,
        event_type: u16,
        status: u32,
    ) -> Result<()> {
        tracing::trace_span!("rxegy::session::Context::dispatch");

        // Grab the session handle
        let session = Session::try_from(handle)?;

        tracing::trace!("Session = {:?}", session);

        // Check the event status
        if Success::try_from(status).is_ok() {
            tracing::trace!("Status is good, maybe setting callback affinity.");

            if let Some(affinity) = self.affinity {
                field::set_u64(
                    *session.as_ref(),
                    rxegy_sys::XC_SESSION,
                    Field::ClientAffinityMask,
                    affinity,
                )?;
            }

            if let Some(prio) = self.priority {
                field::set_u32(
                    *session.as_ref(),
                    rxegy_sys::XC_SESSION,
                    Field::ClientThreadPriority,
                    prio,
                )?;
            }

            if let Some(enable) = self.market_events_per_instrument {
                field::set_u8(
                    *session.as_ref(),
                    rxegy_sys::XC_SESSION,
                    Field::TickerMarketStatusCallbacks,
                    enable as u8,
                )?;
            }
        }

        tracing::trace!("Wrapping up event");

        let event = Event::new(event_type, event_handle)?;

        tracing::trace_span!("rxegy::session::Context::dispatch::user");
        match event {
            Event::Status(status_event) => self.callbacks.as_ref().status(&session, &status_event),
        };

        Ok(())
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _rxegy_session_callback(
    handle: xhandle,
    _slot: u32,
    event_handle: xhandle,
    event_type: u16,
    turnkey: u64,
    status: xerr,
) {
    // TODO: log panics
    let _ = std::panic::catch_unwind(|| {
        tracing::trace_span!("rxegy::session::_rxegy_session_callback");

        // Get our context
        let context = unsafe {
            let ptr = turnkey as *mut Context;
            Box::from_raw(ptr)
        };

        tracing::trace!("Dispatching to user callbacks...");
        if let Err(error) = context.dispatch(handle, event_handle, event_type, status) {
            tracing::error!("Dispatch returned an error: {}", error);
        }
        tracing::trace!("Completed dispatching");

        let _leaked = Box::into_raw(context);
    });
}
