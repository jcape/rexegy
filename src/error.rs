//! Errors used within this crate

use std::{
    ffi::{FromVecWithNulError, NulError},
    io::Error as IoError,
    result::Result as StdResult,
    str::Utf8Error,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error created by the XCAPI library
    #[error("XCAPI library error: {0}")]
    Exegy(#[from] ExegyError),

    /// The group ID does not match one we're expecting
    #[error("The group ID provided is not known")]
    GroupUnknown,

    /// The feed ID does not match one we're expecting
    #[error("The feed ID provided is unknown or unexpected")]
    FeedUnknown,

    /// The object kind is unknown or unexpected
    #[error("The object kind is unknown or unexpected")]
    ObjectUnknown,

    /// An I/O error occurred
    #[error("There was an IO error: {0:?}")]
    Io(#[from] IoError),

    /// The source of the C string in question contains an embedded nul already.
    #[error("The string in question contains an embedded nul character: {0}")]
    EmbeddedNull(#[from] NulError),

    /// The source of a new C string was not nul-terminated.
    #[error("The string in question was not nul-terminated: {0}")]
    NoNullTerm(#[from] FromVecWithNulError),

    /// A thread paniced while holding the lock to a session.
    #[error("The mutex locking a session was poisoned")]
    SessionPanic,

    /// Attempted to create a session or container without a callback object.
    #[error("Attempted to create a session or container without a callback object")]
    NoCallbacksSet,

    /// The string in question contains invalid UTF-8 characters.
    #[error("The string in question contains invalid UTF-8: {0}")]
    InvalidUtf8(#[from] Utf8Error),

    /// The session has not been initialized yet, this is a race condition!
    #[error("The session's internal handle has not been set")]
    SessionNotInitialized,

    /// The object is null.
    #[error("The XCAPI object handle is null")]
    NullObject,

    /// The object pointed at by the XCAPI object handle is of an unexpected kind
    #[error("The object pointed at by the XCAPI object handle is of an unexpected kind")]
    UnexpectedKind,
}

/// A local result type used to encapsulate a result and an FFI error.
pub type Result<T> = std::result::Result<T, Error>;

/// An enumeration of errors which can be encountered in this crate.
#[derive(Debug, thiserror::Error)]
#[repr(u32)]
#[non_exhaustive]
pub enum ExegyError {
    /// Operation not permitted
    #[error("Operation not permitted")]
    NotPermitted = rxegy_sys::XPERM,

    /// No entry found
    #[error("No entry found")]
    NoEntry = rxegy_sys::XNOENT,

    /// I/O error
    #[error("I/O error")]
    Io = rxegy_sys::XIO,

    /// Operation would block
    #[error("Operation would block")]
    WouldBlock = rxegy_sys::XWOULDBLOCK,

    /// Permission denied
    #[error("Permission denied")]
    Access = rxegy_sys::XACCESS,

    /// Bad address
    #[error("Bad address")]
    BadAddress = rxegy_sys::XADDR,

    /// Device or resource busy
    #[error("Device or resource busy")]
    Busy = rxegy_sys::XBUSY,

    /// (WARNING) Queuing delay detected
    #[error("WARNING) Queuing delay detected")]
    QueueDelay = rxegy_sys::XQUEUEDELAY,

    /// Item already exists
    #[error("Item already exists")]
    Exist = rxegy_sys::XEXIST,

    /// No such device
    #[error("No such device")]
    NoDevice = rxegy_sys::XNODEV,

    /// Not a directory
    #[error("Not a directory")]
    NotDir = rxegy_sys::XNOTDIR,

    /// Is a directory
    #[error("Is a directory")]
    IsDir = rxegy_sys::XISDIR,

    /// Invalid argument
    #[error("Invalid argument")]
    InvalidArgument = rxegy_sys::XINVAL,

    /// Not enough memory
    #[error("Not enough memory")]
    NoMemory = rxegy_sys::XNOMEM,

    /// Function not implemented
    #[error("Function not implemented")]
    NotImplemented = rxegy_sys::XIMPL,

    /// No data available
    #[error("No data available")]
    NoData = rxegy_sys::XNODATA,

    /// Timer expired
    #[error("Timer expired")]
    Time = rxegy_sys::XTIME,

    /// Link has been severed
    #[error("Link has been severed")]
    NoLink = rxegy_sys::XNOLINK,

    /// Communication error
    #[error("Communication error")]
    Communication = rxegy_sys::XCOMM,

    /// Protocol error
    #[error("Protocol error")]
    Protocol = rxegy_sys::XPROTO,

    /// Address in use
    #[error("Address in use")]
    AddressInUse = rxegy_sys::XADDRINUSE,

    /// Connection reset by peer
    #[error("Connection reset by peer")]
    ConnectionReset = rxegy_sys::XCONNRESET,

    /// Connection refused
    #[error("Connection refused")]
    ConnectionRefused = rxegy_sys::XCONNREFUSED,

    /// Operation interrupted
    #[error("Operation interrupted")]
    Interrupted = rxegy_sys::XINTR,

    /// End of results
    #[error("End of results")]
    End = rxegy_sys::XEND,

    /// No available cards
    #[error("No available cards")]
    NoCard = rxegy_sys::XNOCARD,

    /// Invalid operation chain
    #[error("Invalid operation chain")]
    BadChain = rxegy_sys::XBADCHAIN,

    /// Capacity exceeded
    #[error("Capacity exceeded")]
    Capacity = rxegy_sys::XCAPACITY,

    /// Hardware not loaded
    #[error("Hardware not loaded")]
    HardwareNotLoaded = rxegy_sys::XHWNOTLOADED,

    /// Socket error
    #[error("Socket error")]
    Socket = rxegy_sys::XSOCKET,

    /// Version mismatch
    #[error("Version mismatch")]
    Version = rxegy_sys::XVERSION,

    /// Inconsistent or bad state
    #[error("Inconsistent or bad state")]
    BadState = rxegy_sys::XBADSTATE,

    /// Client was disconnected because it's a slow consumer
    #[error("Client was disconnected because it's a slow consumer")]
    SlowConsumer = rxegy_sys::XSLOWCONSUMER,

    /// Invalid symbol
    #[error("Invalid symbol")]
    BadSymbol = rxegy_sys::XBADSYMBOL,

    /// Overflow
    #[error("Overflow")]
    Overflow = rxegy_sys::XOVERFLOW,

    /// Socket not connected
    #[error("Socket not connected")]
    NotConnected = rxegy_sys::XNOTCONNECTED,

    /// Duplicate item
    #[error("Duplicate item")]
    Duplicate = rxegy_sys::XDUPLICATE,

    /// Ignored
    #[error("Ignored")]
    Ignore = rxegy_sys::XIGNORE,

    /// Invalid handle
    #[error("Invalid handle")]
    BadHandle = rxegy_sys::XBADHANDLE,

    /// Invalid field
    #[error("Invalid field")]
    BadField = rxegy_sys::XBADFIELD,

    /// Invalid slot
    #[error("Invalid slot")]
    BadSlot = rxegy_sys::XBADSLOT,

    /// Login failed
    #[error("Login failed")]
    LoginFailed = rxegy_sys::XLOGINFAILED,

    /// Pending
    #[error("Pending")]
    Pending = rxegy_sys::XPENDING,

    /// Data is stale
    #[error("Data is stale")]
    Stale = rxegy_sys::XSTALE,

    /// Subsystem uninitialized
    #[error("Subsystem uninitialized")]
    Uninitialized = rxegy_sys::XUNINIT,

    /// Connection timed out
    #[error("Connection timed out")]
    Timeout = rxegy_sys::XTIMEOUT,

    /// Not supported
    #[error("Not supported")]
    NotSupported = rxegy_sys::XNOTSUP,

    /// Type not supported for specified field
    #[error("Type not supported for specified field")]
    Type = rxegy_sys::XTYPE,

    /// Already connected
    #[error("Already connected")]
    Already = rxegy_sys::XALREADY,

    /// Exceeded maximum number of retries
    #[error("Exceeded maximum number of retries")]
    Retry = rxegy_sys::XRETRY,

    /// Internal error
    #[error("Internal error")]
    Internal = rxegy_sys::XINTERR,

    /// Item is missing, but shouldn't be
    #[error("Item is missing, but shouldn't be")]
    Missing = rxegy_sys::XMISSING,

    /// Operation was or should be aborted
    #[error("Operation was or should be aborted")]
    Abort = rxegy_sys::XABORT,

    /// Bad data encountered
    #[error("Bad data encountered")]
    BadData = rxegy_sys::XBADDATA,

    /// Ambiguous data or command
    #[error("Ambiguous data or command")]
    Ambiguous = rxegy_sys::XAMBIG,

    /// No search terms
    #[error("No search terms")]
    NoSearchTerms = rxegy_sys::XNOTERMS,

    /// Configuration data is missing or invalid
    #[error("Configuration data is missing or invalid")]
    Config = rxegy_sys::XCONFIG,

    /// Nonsensical command or request received
    #[error("Nonsensical command or request received")]
    Nonsense = rxegy_sys::XNONSENSE,

    /// Bad depth
    #[error("Bad depth")]
    BadDepth = rxegy_sys::XBADDEPTH,

    /// Bad size
    #[error("Bad size")]
    BadSize = rxegy_sys::XBADSIZE,

    /// Wrong thread for function
    #[error("Wrong thread for function")]
    Thread = rxegy_sys::XTHREAD,

    /// Regional exists
    #[error("Regional exists")]
    HasRegional = rxegy_sys::XHASREGIONAL,

    /// Server down for maintenance
    #[error("Server down for maintenance")]
    Down = rxegy_sys::XDOWN,

    /// General error condition
    #[error("General error condition")]
    Error = rxegy_sys::XERROR,

    /// Unsupported HCA firmware revision
    #[error("Unsupported HCA firmware revision")]
    BadFirmware = rxegy_sys::XBADFIRMWARE,

    /// Unrecoverable loss detected in transport session
    #[error("Unrecoverable loss detected in transport session")]
    UnrecoverableLoss = rxegy_sys::XUNRECOVERABLELOSS,

    /// Symbol quota has been reached
    #[error("Symbol quota has been reached")]
    SymbolQuota = rxegy_sys::XSYMBOLQUOTA,

    /// Connection limit reached
    #[error("Connection limit reached")]
    ConnnectionLimit = rxegy_sys::XCONNLIMIT,

    /// Invalid call for a read-only field or container
    #[error("Invalid call for a read-only field or container")]
    ReadOnly = rxegy_sys::XREADONLY,

    /// Invalid hardware address
    #[error("Invalid hardware address")]
    InvalidAddress = rxegy_sys::XINVALIDADDR,

    /// Expired symbol
    #[error("Expired symbol")]
    Expired = rxegy_sys::XEXPIRED,

    /// The session is already in use
    #[error("The session is already in use")]
    SessionInUse = rxegy_sys::XSESSIONINUSE,

    /// The client has hit its session limit
    #[error("The client has hit its session limit")]
    SessionLimit = rxegy_sys::XSESSIONLIMIT,

    /// Invalid session id
    #[error("Invalid session id")]
    InvalidSession = rxegy_sys::XINVALIDSESSION,

    /// Target is disabled
    #[error("Target is disabled")]
    Disabled = rxegy_sys::XDISABLED,

    /// Target should be deleted
    #[error("Target should be deleted")]
    Delete = rxegy_sys::XDELETE,

    /// Deauthorized for content
    #[error("Deauthorized for content")]
    Deauthorize = rxegy_sys::XDEAUTHORIZE,

    /// Force disconnect
    #[error("Force disconnect")]
    ForceOff = rxegy_sys::XFORCEOFF,
}

impl TryFrom<u32> for ExegyError {
    type Error = ();

    fn try_from(value: u32) -> StdResult<Self, <Self as TryFrom<u32>>::Error> {
        match value {
            rxegy_sys::XPERM => Ok(ExegyError::NotPermitted),
            rxegy_sys::XNOENT => Ok(ExegyError::NoEntry),
            rxegy_sys::XIO => Ok(ExegyError::Io),
            rxegy_sys::XWOULDBLOCK => Ok(ExegyError::WouldBlock),
            rxegy_sys::XACCESS => Ok(ExegyError::Access),
            rxegy_sys::XADDR => Ok(ExegyError::BadAddress),
            rxegy_sys::XBUSY => Ok(ExegyError::Busy),
            rxegy_sys::XQUEUEDELAY => Ok(ExegyError::QueueDelay),
            rxegy_sys::XEXIST => Ok(ExegyError::Exist),
            rxegy_sys::XNODEV => Ok(ExegyError::NoDevice),
            rxegy_sys::XNOTDIR => Ok(ExegyError::NotDir),
            rxegy_sys::XISDIR => Ok(ExegyError::IsDir),
            rxegy_sys::XINVAL => Ok(ExegyError::InvalidArgument),
            rxegy_sys::XNOMEM => Ok(ExegyError::NoMemory),
            rxegy_sys::XIMPL => Ok(ExegyError::NotImplemented),
            rxegy_sys::XNODATA => Ok(ExegyError::NoData),
            rxegy_sys::XTIME => Ok(ExegyError::Time),
            rxegy_sys::XNOLINK => Ok(ExegyError::NoLink),
            rxegy_sys::XCOMM => Ok(ExegyError::Communication),
            rxegy_sys::XPROTO => Ok(ExegyError::Protocol),
            rxegy_sys::XADDRINUSE => Ok(ExegyError::AddressInUse),
            rxegy_sys::XCONNRESET => Ok(ExegyError::ConnectionReset),
            rxegy_sys::XCONNREFUSED => Ok(ExegyError::ConnectionRefused),
            rxegy_sys::XINTR => Ok(ExegyError::Interrupted),
            rxegy_sys::XEND => Ok(ExegyError::End),
            rxegy_sys::XNOCARD => Ok(ExegyError::NoCard),
            rxegy_sys::XBADCHAIN => Ok(ExegyError::BadChain),
            rxegy_sys::XCAPACITY => Ok(ExegyError::Capacity),
            rxegy_sys::XHWNOTLOADED => Ok(ExegyError::HardwareNotLoaded),
            rxegy_sys::XSOCKET => Ok(ExegyError::Socket),
            rxegy_sys::XVERSION => Ok(ExegyError::Version),
            rxegy_sys::XBADSTATE => Ok(ExegyError::BadState),
            rxegy_sys::XSLOWCONSUMER => Ok(ExegyError::SlowConsumer),
            rxegy_sys::XBADSYMBOL => Ok(ExegyError::BadSymbol),
            rxegy_sys::XOVERFLOW => Ok(ExegyError::Overflow),
            rxegy_sys::XNOTCONNECTED => Ok(ExegyError::NotConnected),
            rxegy_sys::XDUPLICATE => Ok(ExegyError::Duplicate),
            rxegy_sys::XIGNORE => Ok(ExegyError::Ignore),
            rxegy_sys::XBADHANDLE => Ok(ExegyError::BadHandle),
            rxegy_sys::XBADFIELD => Ok(ExegyError::BadField),
            rxegy_sys::XBADSLOT => Ok(ExegyError::BadSlot),
            rxegy_sys::XLOGINFAILED => Ok(ExegyError::LoginFailed),
            rxegy_sys::XPENDING => Ok(ExegyError::Pending),
            rxegy_sys::XSTALE => Ok(ExegyError::Stale),
            rxegy_sys::XUNINIT => Ok(ExegyError::Uninitialized),
            rxegy_sys::XTIMEOUT => Ok(ExegyError::Timeout),
            rxegy_sys::XNOTSUP => Ok(ExegyError::NotSupported),
            rxegy_sys::XTYPE => Ok(ExegyError::Type),
            rxegy_sys::XALREADY => Ok(ExegyError::Already),
            rxegy_sys::XRETRY => Ok(ExegyError::Retry),
            rxegy_sys::XINTERR => Ok(ExegyError::Interrupted),
            rxegy_sys::XMISSING => Ok(ExegyError::Missing),
            rxegy_sys::XABORT => Ok(ExegyError::Abort),
            rxegy_sys::XBADDATA => Ok(ExegyError::BadData),
            rxegy_sys::XAMBIG => Ok(ExegyError::Ambiguous),
            rxegy_sys::XNOTERMS => Ok(ExegyError::NoSearchTerms),
            rxegy_sys::XCONFIG => Ok(ExegyError::Config),
            rxegy_sys::XNONSENSE => Ok(ExegyError::Nonsense),
            rxegy_sys::XBADDEPTH => Ok(ExegyError::BadDepth),
            rxegy_sys::XBADSIZE => Ok(ExegyError::BadSize),
            rxegy_sys::XTHREAD => Ok(ExegyError::Thread),
            rxegy_sys::XHASREGIONAL => Ok(ExegyError::HasRegional),
            rxegy_sys::XDOWN => Ok(ExegyError::Down),
            rxegy_sys::XERROR => Ok(ExegyError::Error),
            rxegy_sys::XBADFIRMWARE => Ok(ExegyError::BadFirmware),
            rxegy_sys::XUNRECOVERABLELOSS => Ok(ExegyError::UnrecoverableLoss),
            rxegy_sys::XSYMBOLQUOTA => Ok(ExegyError::SymbolQuota),
            rxegy_sys::XCONNLIMIT => Ok(ExegyError::ConnnectionLimit),
            rxegy_sys::XREADONLY => Ok(ExegyError::ReadOnly),
            rxegy_sys::XINVALIDADDR => Ok(ExegyError::InvalidAddress),
            rxegy_sys::XEXPIRED => Ok(ExegyError::Expired),
            rxegy_sys::XSESSIONINUSE => Ok(ExegyError::SessionInUse),
            rxegy_sys::XSESSIONLIMIT => Ok(ExegyError::SessionLimit),
            rxegy_sys::XINVALIDSESSION => Ok(ExegyError::InvalidSession),
            rxegy_sys::XDISABLED => Ok(ExegyError::Disabled),
            rxegy_sys::XDELETE => Ok(ExegyError::Delete),
            rxegy_sys::XDEAUTHORIZE => Ok(ExegyError::Deauthorize),
            rxegy_sys::XFORCEOFF => Ok(ExegyError::ForceOff),
            _ => Err(()),
        }
    }
}

/// An enumeration of status codes which are considered a "success".
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
#[non_exhaustive]
pub enum Success {
    // Generic success
    Success = rxegy_sys::XSUCCESS,
    /// Item Found
    Found = rxegy_sys::XFOUND,
    /// Search completed, item not found
    Complete = rxegy_sys::XCOMPLETE,
    /// Some data returned, more data available
    MoreData = rxegy_sys::XMOREDATA,
    /// Result returned, but is only approximate
    Approximate = rxegy_sys::XAPPROXIMATE,
    /// Result returned, but (other) data was modified
    Modified = rxegy_sys::XMODIFIED,
    /// Some aspects of operation were successful, but others were not
    PartialSuccess = rxegy_sys::XPARTIALSUCCESS,
    /// Data returned is in between boundary events
    InterBoundary = rxegy_sys::XINTERBOUNDARY,
    /// Normal processing has resumed
    Resumed = rxegy_sys::XRESUMED,
    /// Operation ignored, would do nothing
    NoChange = rxegy_sys::XNOCHANGE,
}

impl TryFrom<u32> for Success {
    type Error = ExegyError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            rxegy_sys::XFOUND => Ok(Success::Found),
            rxegy_sys::XCOMPLETE => Ok(Success::Complete),
            rxegy_sys::XMOREDATA => Ok(Success::MoreData),
            rxegy_sys::XAPPROXIMATE => Ok(Success::Approximate),
            rxegy_sys::XMODIFIED => Ok(Success::Modified),
            rxegy_sys::XPARTIALSUCCESS => Ok(Success::PartialSuccess),
            rxegy_sys::XINTERBOUNDARY => Ok(Success::InterBoundary),
            rxegy_sys::XRESUMED => Ok(Success::Resumed),
            rxegy_sys::XNOCHANGE => Ok(Success::NoChange),
            _ => Err(ExegyError::try_from(value).unwrap_or(ExegyError::Error)),
        }
    }
}
