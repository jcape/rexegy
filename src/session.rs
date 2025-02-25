//! Session Objects

use crate::{
    error::{Error, ExegyError, Result, Success},
    object::Kind as ObjectKind,
};
use rxegy_sys::{XC_SESSION, xcCreateSession, xcGetField, xcSetField, xerr, xhandle};
use secrecy::{ExposeSecret, SecretString};
use std::{
    any::{Any, TypeId},
    ffi::{self, CString},
    fmt::{Display, Formatter, Result as FmtResult},
    mem,
    net::{SocketAddr, ToSocketAddrs},
    os::raw::c_void,
    path::{Path, PathBuf},
    pin::Pin,
    ptr::{self, NonNull},
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

#[repr(u64)]
pub enum Field {
    Turnkey = rxegy_sys::XFLD_SESS_TURNKEY,
    SessionType = rxegy_sys::XFLD_SESS_SESSION_TYPE,
    Status = rxegy_sys::XFLD_SESS_STATUS,
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
    TkrMaxPriceBookDepth = rxegy_sys::XFLD_SESS_TKR_MAX_PRICE_BOOK_DEPTH,
    TkrMarketStatusCallbacks = rxegy_sys::XFLD_SESS_TKR_MARKET_STATUS_CALLBACKS,
    TkrMaxPbRowLevel = rxegy_sys::XFLD_SESS_TKR_MAX_PB_ROW_LEVEL,
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
    /// A rusty version of an xhandle
    handle: Option<NonNull<c_void>>,
    /// An object which we'll fire callbacks on
    callbacks: Box<dyn SessionCallbacks>,
    /// The CPU affinity mask to be set when the callback is fired
    cb_affinity_mask: Option<u64>,
    /// The thread proiority to be set when the callback is fired
    cb_priority: Option<u8>,
}

unsafe impl Send for Session {}

impl Session {
    /// Retrieve the object type of this session.
    pub fn kind(&self) -> Result<Kind> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u16.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::SessionType as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    8,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Kind::try_from(u16::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
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
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::Status as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    mem::size_of_val(&obuf) as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(Success::try_from(u32::from_le_bytes(obuf)))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve the client version string
    pub fn client_version_string(&self) -> Result<String> {
        if let Some(handle) = self.handle {
            let mut obuf = vec![0u8; 512];
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientVersionString as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };
            // if there was an error retrieving the status
            Success::try_from(status)?;
            let cstr = CString::from_vec_with_nul(obuf)?;
            Ok(cstr.to_str()?.to_owned())
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve the client major version
    pub fn client_major_version(&self) -> Result<u32> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientMajorVersion as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u32::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve the client minor version
    pub fn client_minor_version(&self) -> Result<u32> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientMinorVersion as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u32::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve the client version revision
    pub fn client_revision(&self) -> Result<u32> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientRevision as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u32::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve the client version build
    pub fn client_build(&self) -> Result<u32> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientBuild as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u32::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve the client CPU count.
    pub fn client_cpu_count(&self) -> Result<u32> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientCpuCount as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u32::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve the affinity mask for the callback thread on this session
    pub fn client_affinity_mask(&self) -> Result<u64> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u64.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientAffinityMask as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };
            // if there was an error retrieving the status
            Success::try_from(status)?;
            Ok(u64::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Sets the affinity mask for the callback thread on this session.
    ///
    /// The affinity will be set the next time the callback is executed.
    pub fn set_client_affinity(&mut self, mask: u64) {
        if mask == 0 {
            self.cb_affinity_mask = None;
        } else {
            self.cb_affinity_mask = Some(mask);
        }
    }

    fn update_thread(
        &self,
        affinity_field: Field,
        mask: Option<u64>,
        priority_field: Field,
        prio: Option<u8>,
    ) {
        if self.handle.is_none() {
            return;
        }

        if let Some(mask_val) = mask {
            let ibuf = mask_val.to_le_bytes();
            let status = unsafe {
                xcSetField(
                    self.handle.unwrap().as_ptr(),
                    XC_SESSION,
                    affinity_field as u64,
                    ibuf.as_ptr() as *const c_void,
                    ibuf.len() as u32,
                )
            };
            let _retval = Success::try_from(status);
        }

        if let Some(prio_val) = prio {
            let ibuf = (prio_val as u32).to_le_bytes();
            let status = unsafe {
                xcSetField(
                    self.handle.unwrap().as_ptr(),
                    XC_SESSION,
                    priority_field as u64,
                    ibuf.as_ptr() as *const c_void,
                    ibuf.len() as u32,
                )
            };
            let _retval = Success::try_from(status);
        }
    }

    /// Update the client thread affinity and priority.
    fn update_client_thread(&self) {
        if self.handle.is_none() {
            return;
        }

        self.update_thread(
            Field::ClientAffinityMask,
            self.cb_affinity_mask,
            Field::ClientThreadPriority,
            self.cb_priority,
        );
    }

    fn update_bg_threads(
        &self,
        bg_affinity: Option<u64>,
        bg_prio: Option<u8>,
        hb_affinity: Option<u64>,
        hb_prio: Option<u8>,
    ) {
        if self.handle.is_none() {
            return;
        }

        self.update_thread(
            Field::ClientBgThreadAffinityMask,
            bg_affinity,
            Field::ClientBgThreadPriority,
            bg_prio,
        );
        self.update_thread(
            Field::ClientHbThreadAffinityMask,
            hb_affinity,
            Field::ClientHbThreadPriority,
            hb_prio,
        );
    }

    /// Retrieve the affinity mask for the callback thread on this session
    pub fn client_thread_priority(&self) -> Result<u32> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientThreadPriority as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u32::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Sets the priority for the callback thread on this session.
    ///
    /// The priority will be set the next time the callback is executed.
    pub fn set_client_priority(&mut self, priority: u8) {
        if priority == 0 {
            self.cb_priority = None;
        } else {
            self.cb_priority = Some(priority.min(99));
        }
    }

    /// Retrieve CPU affinity mask for the background thread.
    pub fn client_bg_thread_affinity_mask(&self) -> Result<u64> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u64.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientBgThreadAffinityMask as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u64::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve CPU affinity mask for the background thread
    pub fn client_bg_thread_priority(&self) -> Result<u32> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientBgThreadPriority as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u32::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve CPU affinity mask for the HB (?) thread.
    pub fn client_hb_thread_affinity_mask(&self) -> Result<u64> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u64.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientHbThreadAffinityMask as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u64::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }

    /// Retrieve CPU affinity mask for the HB (?) thread
    pub fn client_hb_thread_priority(&self) -> Result<u32> {
        if let Some(handle) = self.handle {
            let mut obuf = 0u32.to_le_bytes();
            let status = unsafe {
                xcGetField(
                    handle.as_ptr(),
                    XC_SESSION,
                    Field::ClientHbThreadPriority as u64,
                    obuf.as_mut_ptr() as *mut ffi::c_void,
                    obuf.len() as u32,
                )
            };

            // if there was an error retrieving the status
            Success::try_from(status)?;

            Ok(u32::from_le_bytes(obuf))
        } else {
            Err(Error::SessionNotInitialized)
        }
    }
}

/// A session builder
#[derive(Default)]
pub struct Builder {
    server_list: Vec<Server>,
    username: String,
    password: SecretString,
    callbacks: Option<Box<dyn SessionCallbacks>>,
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

    /// Set the object which will handle session callbacks
    pub fn callbacks(&mut self, callbacks: Box<dyn SessionCallbacks>) -> &mut Self {
        self.callbacks = Some(callbacks);
        self
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

    /// Set the desired priority for the session's callback thread.
    pub fn priority(&mut self, priority: u8) -> &mut Self {
        if priority == 0 {
            self.cb_priority = None;
        } else {
            self.cb_priority = Some(priority);
        }

        self
    }

    /// Set the CPU affinity mask for the background processing htread to the given value
    pub fn bg_affinity(&mut self, mask: u64) -> &mut Self {
        if mask == 0 {
            self.bg_affinity = None
        } else {
            self.bg_affinity = Some(mask);
        }
        self
    }

    pub fn bg_priority(&mut self, priority: u8) -> &mut Self {
        if priority == 0 {
            self.bg_priority = None;
        } else {
            self.bg_priority = Some(priority);
        }
        self
    }

    /// Set the CPU affinity mask for this session.
    pub fn hb_affinity(&mut self, mask: u64) -> &mut Self {
        if mask == 0 {
            self.hb_affinity = None;
        } else {
            self.hb_affinity = Some(mask);
        }
        self
    }

    /// Set the "HB Thread" priority.
    pub fn hb_priority(&mut self, priority: u8) -> &mut Self {
        if priority == 0 {
            self.hb_priority = None;
        } else {
            self.hb_priority = Some(priority);
        }
        self
    }

    /// Connect to the Exegy appliance and return the session object.
    pub fn build(self) -> Result<Pin<Arc<Mutex<Session>>>> {
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

        // Make our session object in a Pin'd Arc Mutex (probably overkill, but prevents some backdoors into the session)
        let mut retval = Arc::pin(Mutex::new(Session {
            handle: None,
            callbacks: self.callbacks.ok_or(Error::NoCallbacksSet)?,
            cb_affinity_mask: self.cb_affinity,
            cb_priority: self.cb_priority,
        }));

        // Next, we create a trait object around a reference to our arc mutex
        let anyref = &mut retval as &mut dyn Any;
        // Then we box the trait object
        let boxed = Box::new(anyref);
        // Finally, we cast the raw boxed pointer to the u64 that exegy requires as a "turnkey",
        // or user-data pointer
        let turnkey = Box::into_raw(boxed) as u64;

        let mut session = retval.lock().map_err(|_e| Error::SessionPanic)?;
        let mut handle = ptr::null_mut();
        let status = unsafe {
            xcCreateSession(
                Kind::Ticker as u16,
                &mut handle,
                Some(_rxegy_session_callback),
                turnkey,
                server_list.as_ptr(),
                username.as_ptr(),
                password.as_ptr(),
            )
        };
        session.handle = NonNull::new(handle);
        Success::try_from(status)?;

        session.update_bg_threads(
            self.bg_affinity,
            self.bg_priority,
            self.hb_affinity,
            self.hb_priority,
        );

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
    status: xerr,
) {
    let _ = std::panic::catch_unwind(|| {
        // Check the status
        let _result = Success::try_from(status);

        // Check event kind
        let _kind = EventKind::try_from(event_type).expect("Unknown event type received");

        // Get our event object
        let event = StatusEvent(event_handle);

        // Get our session
        let boxed = unsafe {
            let ptr = turnkey as *mut &mut dyn Any;
            Box::from_raw(ptr)
        };

        if boxed.type_id() != TypeId::of::<Pin<Arc<Mutex<Session>>>>() {
            // FIXME: don't just panic.
            panic!("Got something other than an Arc<Mutex<Session>> in _rxegy_session_callback");
        }

        // FIXME: don't panic here either (requires logging framework integration)
        let mutex = (*boxed)
            .downcast_ref::<Pin<Arc<Mutex<Session>>>>()
            .expect("Couldn't downcast turnkey to an Arc<Session>")
            .clone();

        let session = mutex
            .lock()
            .expect("Could not acquire a lock on our session");

        session.update_client_thread();

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
