//! Errors used within this crate

use std::result::Result as StdResult;

#[derive(Copy, Clone, displaydoc::Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// An error created by the XCAPI library
    Exegy(ExegyError),
    /// The group ID does not match one we're expecting
    GroupUnknown,
    /// The feed ID does not match one we're expecting
    FeedUnknown,
}

/// A local result type used to encapsulate a result and an FFI error.
pub type Result<T> = std::result::Result<T, Error>;

/// An enumeration of errors which can be encountered in this crate.
#[derive(Copy, Clone, displaydoc::Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
#[non_exhaustive]
pub enum ExegyError {
    /// Operation not permitted
    NotPermitted = rexegy_sys::XPERM,
    /// No entry found
    NoEntry = rexegy_sys::XNOENT,
    /// I/O error
    Io = rexegy_sys::XIO,
    /// Operation would block
    WouldBlock = rexegy_sys::XWOULDBLOCK,
    /// Permission denied
    Access = rexegy_sys::XACCESS,
    /// Bad address
    BadAddress = rexegy_sys::XADDR,
    /// Device or resource busy
    Busy = rexegy_sys::XBUSY,
    /// (WARNING) Queuing delay detected
    QueueDelay = rexegy_sys::XQUEUEDELAY,
    /// Item already exists
    Exist = rexegy_sys::XEXIST,
    /// No such device
    NoDevice = rexegy_sys::XNODEV,
    /// Not a directory
    NotDir = rexegy_sys::XNOTDIR,
    /// Is a directory
    IsDir = rexegy_sys::XISDIR,
    /// Invalid argument
    InvalidArgument = rexegy_sys::XINVAL,
    /// Not enough memory
    NoMemory = rexegy_sys::XNOMEM,
    /// Function not implemented
    NotImplemented = rexegy_sys::XIMPL,
    /// No data available
    NoData = rexegy_sys::XNODATA,
    /// Timer expired
    Time = rexegy_sys::XTIME,
    /// Link has been severed
    NoLink = rexegy_sys::XNOLINK,
    /// Communication error
    Communication = rexegy_sys::XCOMM,
    /// Protocol error
    Protocol = rexegy_sys::XPROTO,
    /// Address in use
    AddressInUse = rexegy_sys::XADDRINUSE,
    /// Connection reset by peer
    ConnectionReset = rexegy_sys::XCONNRESET,
    /// Connection refused
    ConnectionRefused = rexegy_sys::XCONNREFUSED,
    /// Operation interrupted
    Interrupted = rexegy_sys::XINTR,
    /// End of results
    End = rexegy_sys::XEND,
    /// No available cards
    NoCard = rexegy_sys::XNOCARD,
    /// Invalid operation chain
    BadChain = rexegy_sys::XBADCHAIN,
    /// Capacity exceeded
    Capacity = rexegy_sys::XCAPACITY,
    /// Hardware not loaded
    HardwareNotLoaded = rexegy_sys::XHWNOTLOADED,
    /// Socket error
    Socket = rexegy_sys::XSOCKET,
    /// Version mismatch
    Version = rexegy_sys::XVERSION,
    /// Inconsistent or bad state
    BadState = rexegy_sys::XBADSTATE,
    /// Client was disconnected because it's a slow consumer
    SlowConsumer = rexegy_sys::XSLOWCONSUMER,
    /// Invalid symbol
    BadSymbol = rexegy_sys::XBADSYMBOL,
    /// Overflow
    Overflow = rexegy_sys::XOVERFLOW,
    /// Socket not connected
    NotConnected = rexegy_sys::XNOTCONNECTED,
    /// Duplicate item
    Duplicate = rexegy_sys::XDUPLICATE,
    /// Ignored
    Ignore = rexegy_sys::XIGNORE,
    /// Invalid handle
    BadHandle = rexegy_sys::XBADHANDLE,
    /// Invalid field
    BadField = rexegy_sys::XBADFIELD,
    /// Invalid slot
    BadSlot = rexegy_sys::XBADSLOT,
    /// Login failed
    LoginFailed = rexegy_sys::XLOGINFAILED,
    /// Pending
    Pending = rexegy_sys::XPENDING,
    /// Data is stale
    Stale = rexegy_sys::XSTALE,
    /// Subsystem uninitialized
    Uninitialized = rexegy_sys::XUNINIT,
    /// Connection timed out
    Timeout = rexegy_sys::XTIMEOUT,
    /// Not supported
    NotSupported = rexegy_sys::XNOTSUP,
    /// Type not supported for specified field
    Type = rexegy_sys::XTYPE,
    /// Already connected
    Already = rexegy_sys::XALREADY,
    /// Exceeded maximum number of retries
    Retry = rexegy_sys::XRETRY,
    /// Internal error
    Internal = rexegy_sys::XINTERR,
    /// Item is missing, but shouldn't be
    Missing = rexegy_sys::XMISSING,
    /// Operation was or should be aborted
    Abort = rexegy_sys::XABORT,
    /// Bad data encountered
    BadData = rexegy_sys::XBADDATA,
    /// Ambiguous data or command
    Ambiguous = rexegy_sys::XAMBIG,
    /// No search terms
    NoSearchTerms = rexegy_sys::XNOTERMS,
    /// Configuration data is missing or invalid
    Config = rexegy_sys::XCONFIG,
    /// Nonsensical command or request received
    Nonsense = rexegy_sys::XNONSENSE,
    /// Bad depth
    BadDepth = rexegy_sys::XBADDEPTH,
    /// Bad size
    BadSize = rexegy_sys::XBADSIZE,
    /// Wrong thread for function
    Thread = rexegy_sys::XTHREAD,
    /// Regional exists
    HasRegional = rexegy_sys::XHASREGIONAL,
    /// Server down for maintenance
    Down = rexegy_sys::XDOWN,
    /// General error condition
    Error = rexegy_sys::XERROR,
    /// Unsupported HCA firmware revision
    BadFirmware = rexegy_sys::XBADFIRMWARE,
    /// Unrecoverable loss detected in transport session
    UnrecoverableLoss = rexegy_sys::XUNRECOVERABLELOSS,
    /// Symbol quota has been reached
    SymbolQuota = rexegy_sys::XSYMBOLQUOTA,
    /// Connection limit reached
    ConnnectionLimit = rexegy_sys::XCONNLIMIT,
    /// Invalid call for a read-only field or container
    ReadOnly = rexegy_sys::XREADONLY,
    /// Invalid hardware address
    InvalidAddress = rexegy_sys::XINVALIDADDR,
    /// Expired symbol
    Expired = rexegy_sys::XEXPIRED,
    /// The session is already in use
    SessionInUse = rexegy_sys::XSESSIONINUSE,
    /// The client has hit its session limit
    SessionLimit = rexegy_sys::XSESSIONLIMIT,
    /// Invalid session id
    InvalidSession = rexegy_sys::XINVALIDSESSION,
    /// Target is disabled
    Disabled = rexegy_sys::XDISABLED,
    /// Target should be deleted
    Delete = rexegy_sys::XDELETE,
    /// Deauthorized for content
    Deauthorize = rexegy_sys::XDEAUTHORIZE,
    /// Force disconnect
    ForceOff = rexegy_sys::XFORCEOFF,
}

impl TryFrom<u32> for ExegyError {
    type Error = ();

    fn try_from(value: u32) -> StdResult<Self, <Self as TryFrom<u32>>::Error> {
        match value {
            rexegy_sys::XPERM => Ok(ExegyError::NotPermitted),
            rexegy_sys::XNOENT => Ok(ExegyError::NoEntry),
            rexegy_sys::XIO => Ok(ExegyError::Io),
            rexegy_sys::XWOULDBLOCK => Ok(ExegyError::WouldBlock),
            rexegy_sys::XACCESS => Ok(ExegyError::Access),
            rexegy_sys::XADDR => Ok(ExegyError::BadAddress),
            rexegy_sys::XBUSY => Ok(ExegyError::Busy),
            rexegy_sys::XQUEUEDELAY => Ok(ExegyError::QueueDelay),
            rexegy_sys::XEXIST => Ok(ExegyError::Exist),
            rexegy_sys::XNODEV => Ok(ExegyError::NoDevice),
            rexegy_sys::XNOTDIR => Ok(ExegyError::NotDir),
            rexegy_sys::XISDIR => Ok(ExegyError::IsDir),
            rexegy_sys::XINVAL => Ok(ExegyError::InvalidArgument),
            rexegy_sys::XNOMEM => Ok(ExegyError::NoMemory),
            rexegy_sys::XIMPL => Ok(ExegyError::NotImplemented),
            rexegy_sys::XNODATA => Ok(ExegyError::NoData),
            rexegy_sys::XTIME => Ok(ExegyError::Time),
            rexegy_sys::XNOLINK => Ok(ExegyError::NoLink),
            rexegy_sys::XCOMM => Ok(ExegyError::Communication),
            rexegy_sys::XPROTO => Ok(ExegyError::Protocol),
            rexegy_sys::XADDRINUSE => Ok(ExegyError::AddressInUse),
            rexegy_sys::XCONNRESET => Ok(ExegyError::ConnectionReset),
            rexegy_sys::XCONNREFUSED => Ok(ExegyError::ConnectionRefused),
            rexegy_sys::XINTR => Ok(ExegyError::Interrupted),
            rexegy_sys::XEND => Ok(ExegyError::End),
            rexegy_sys::XNOCARD => Ok(ExegyError::NoCard),
            rexegy_sys::XBADCHAIN => Ok(ExegyError::BadChain),
            rexegy_sys::XCAPACITY => Ok(ExegyError::Capacity),
            rexegy_sys::XHWNOTLOADED => Ok(ExegyError::HardwareNotLoaded),
            rexegy_sys::XSOCKET => Ok(ExegyError::Socket),
            rexegy_sys::XVERSION => Ok(ExegyError::Version),
            rexegy_sys::XBADSTATE => Ok(ExegyError::BadState),
            rexegy_sys::XSLOWCONSUMER => Ok(ExegyError::SlowConsumer),
            rexegy_sys::XBADSYMBOL => Ok(ExegyError::BadSymbol),
            rexegy_sys::XOVERFLOW => Ok(ExegyError::Overflow),
            rexegy_sys::XNOTCONNECTED => Ok(ExegyError::NotConnected),
            rexegy_sys::XDUPLICATE => Ok(ExegyError::Duplicate),
            rexegy_sys::XIGNORE => Ok(ExegyError::Ignore),
            rexegy_sys::XBADHANDLE => Ok(ExegyError::BadHandle),
            rexegy_sys::XBADFIELD => Ok(ExegyError::BadField),
            rexegy_sys::XBADSLOT => Ok(ExegyError::BadSlot),
            rexegy_sys::XLOGINFAILED => Ok(ExegyError::LoginFailed),
            rexegy_sys::XPENDING => Ok(ExegyError::Pending),
            rexegy_sys::XSTALE => Ok(ExegyError::Stale),
            rexegy_sys::XUNINIT => Ok(ExegyError::Uninitialized),
            rexegy_sys::XTIMEOUT => Ok(ExegyError::Timeout),
            rexegy_sys::XNOTSUP => Ok(ExegyError::NotSupported),
            rexegy_sys::XTYPE => Ok(ExegyError::Type),
            rexegy_sys::XALREADY => Ok(ExegyError::Already),
            rexegy_sys::XRETRY => Ok(ExegyError::Retry),
            rexegy_sys::XINTERR => Ok(ExegyError::Interrupted),
            rexegy_sys::XMISSING => Ok(ExegyError::Missing),
            rexegy_sys::XABORT => Ok(ExegyError::Abort),
            rexegy_sys::XBADDATA => Ok(ExegyError::BadData),
            rexegy_sys::XAMBIG => Ok(ExegyError::Ambiguous),
            rexegy_sys::XNOTERMS => Ok(ExegyError::NoSearchTerms),
            rexegy_sys::XCONFIG => Ok(ExegyError::Config),
            rexegy_sys::XNONSENSE => Ok(ExegyError::Nonsense),
            rexegy_sys::XBADDEPTH => Ok(ExegyError::BadDepth),
            rexegy_sys::XBADSIZE => Ok(ExegyError::BadSize),
            rexegy_sys::XTHREAD => Ok(ExegyError::Thread),
            rexegy_sys::XHASREGIONAL => Ok(ExegyError::HasRegional),
            rexegy_sys::XDOWN => Ok(ExegyError::Down),
            rexegy_sys::XERROR => Ok(ExegyError::Error),
            rexegy_sys::XBADFIRMWARE => Ok(ExegyError::BadFirmware),
            rexegy_sys::XUNRECOVERABLELOSS => Ok(ExegyError::UnrecoverableLoss),
            rexegy_sys::XSYMBOLQUOTA => Ok(ExegyError::SymbolQuota),
            rexegy_sys::XCONNLIMIT => Ok(ExegyError::ConnnectionLimit),
            rexegy_sys::XREADONLY => Ok(ExegyError::ReadOnly),
            rexegy_sys::XINVALIDADDR => Ok(ExegyError::InvalidAddress),
            rexegy_sys::XEXPIRED => Ok(ExegyError::Expired),
            rexegy_sys::XSESSIONINUSE => Ok(ExegyError::SessionInUse),
            rexegy_sys::XSESSIONLIMIT => Ok(ExegyError::SessionLimit),
            rexegy_sys::XINVALIDSESSION => Ok(ExegyError::InvalidSession),
            rexegy_sys::XDISABLED => Ok(ExegyError::Disabled),
            rexegy_sys::XDELETE => Ok(ExegyError::Delete),
            rexegy_sys::XDEAUTHORIZE => Ok(ExegyError::Deauthorize),
            rexegy_sys::XFORCEOFF => Ok(ExegyError::ForceOff),
            _ => Err(()),
        }
    }
}

/// An enumeration of status codes which are considered "success".
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
#[non_exhaustive]
pub enum Success {
    // Generic success
    Success = rexegy_sys::XSUCCESS,
    /// Item Found
    Found = rexegy_sys::XFOUND,
    /// Search completed, item not found
    Complete = rexegy_sys::XCOMPLETE,
    /// Some data returned, more data available
    MoreData = rexegy_sys::XMOREDATA,
    /// Result returned, but is only approximate
    Approximate = rexegy_sys::XAPPROXIMATE,
    /// Result returned, but (other) data was modified
    Modified = rexegy_sys::XMODIFIED,
    /// Some aspects of operation were successful, but others were not
    PartialSuccess = rexegy_sys::XPARTIALSUCCESS,
    /// Data returned is in between boundary events
    InterBoundary = rexegy_sys::XINTERBOUNDARY,
    /// Normal processing has resumed
    Resumed = rexegy_sys::XRESUMED,
    /// Operation ignored, would do nothing
    NoChange = rexegy_sys::XNOCHANGE,
}

impl TryFrom<u32> for Success {
    type Error = ExegyError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            rexegy_sys::XFOUND => Ok(Success::Found),
            rexegy_sys::XCOMPLETE => Ok(Success::Complete),
            rexegy_sys::XMOREDATA => Ok(Success::MoreData),
            rexegy_sys::XAPPROXIMATE => Ok(Success::Approximate),
            rexegy_sys::XMODIFIED => Ok(Success::Modified),
            rexegy_sys::XPARTIALSUCCESS => Ok(Success::PartialSuccess),
            rexegy_sys::XINTERBOUNDARY => Ok(Success::InterBoundary),
            rexegy_sys::XRESUMED => Ok(Success::Resumed),
            rexegy_sys::XNOCHANGE => Ok(Success::NoChange),
            _ => Err(ExegyError::try_from(value).unwrap_or(ExegyError::Error)),
        }
    }
}
