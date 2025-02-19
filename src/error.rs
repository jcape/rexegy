//! Errors used within this crate

use std::result::Result as StdResult;

/// An enumeration of errors which can be encountered in this crate.
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
#[non_exhaustive]
pub enum Error {
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

impl TryFrom<u32> for Error {
    type Error = ();

    fn try_from(value: u32) -> StdResult<Self, <Self as TryFrom<u32>>::Error> {
        match value {
            rexegy_sys::XPERM => Ok(Error::NotPermitted),
            rexegy_sys::XNOENT => Ok(Error::NoEntry),
            rexegy_sys::XIO => Ok(Error::Io),
            rexegy_sys::XWOULDBLOCK => Ok(Error::WouldBlock),
            rexegy_sys::XACCESS => Ok(Error::Access),
            rexegy_sys::XADDR => Ok(Error::BadAddress),
            rexegy_sys::XBUSY => Ok(Error::Busy),
            rexegy_sys::XQUEUEDELAY => Ok(Error::QueueDelay),
            rexegy_sys::XEXIST => Ok(Error::Exist),
            rexegy_sys::XNODEV => Ok(Error::NoDevice),
            rexegy_sys::XNOTDIR => Ok(Error::NotDir),
            rexegy_sys::XISDIR => Ok(Error::IsDir),
            rexegy_sys::XINVAL => Ok(Error::InvalidArgument),
            rexegy_sys::XNOMEM => Ok(Error::NoMemory),
            rexegy_sys::XIMPL => Ok(Error::NotImplemented),
            rexegy_sys::XNODATA => Ok(Error::NoData),
            rexegy_sys::XTIME => Ok(Error::Time),
            rexegy_sys::XNOLINK => Ok(Error::NoLink),
            rexegy_sys::XCOMM => Ok(Error::Communication),
            rexegy_sys::XPROTO => Ok(Error::Protocol),
            rexegy_sys::XADDRINUSE => Ok(Error::AddressInUse),
            rexegy_sys::XCONNRESET => Ok(Error::ConnectionReset),
            rexegy_sys::XCONNREFUSED => Ok(Error::ConnectionRefused),
            rexegy_sys::XINTR => Ok(Error::Interrupted),
            rexegy_sys::XEND => Ok(Error::End),
            rexegy_sys::XNOCARD => Ok(Error::NoCard),
            rexegy_sys::XBADCHAIN => Ok(Error::BadChain),
            rexegy_sys::XCAPACITY => Ok(Error::Capacity),
            rexegy_sys::XHWNOTLOADED => Ok(Error::HardwareNotLoaded),
            rexegy_sys::XSOCKET => Ok(Error::Socket),
            rexegy_sys::XVERSION => Ok(Error::Version),
            rexegy_sys::XBADSTATE => Ok(Error::BadState),
            rexegy_sys::XSLOWCONSUMER => Ok(Error::SlowConsumer),
            rexegy_sys::XBADSYMBOL => Ok(Error::BadSymbol),
            rexegy_sys::XOVERFLOW => Ok(Error::Overflow),
            rexegy_sys::XNOTCONNECTED => Ok(Error::NotConnected),
            rexegy_sys::XDUPLICATE => Ok(Error::Duplicate),
            rexegy_sys::XIGNORE => Ok(Error::Ignore),
            rexegy_sys::XBADHANDLE => Ok(Error::BadHandle),
            rexegy_sys::XBADFIELD => Ok(Error::BadField),
            rexegy_sys::XBADSLOT => Ok(Error::BadSlot),
            rexegy_sys::XLOGINFAILED => Ok(Error::LoginFailed),
            rexegy_sys::XPENDING => Ok(Error::Pending),
            rexegy_sys::XSTALE => Ok(Error::Stale),
            rexegy_sys::XUNINIT => Ok(Error::Uninitialized),
            rexegy_sys::XTIMEOUT => Ok(Error::Timeout),
            rexegy_sys::XNOTSUP => Ok(Error::NotSupported),
            rexegy_sys::XTYPE => Ok(Error::Type),
            rexegy_sys::XALREADY => Ok(Error::Already),
            rexegy_sys::XRETRY => Ok(Error::Retry),
            rexegy_sys::XINTERR => Ok(Error::Interrupted),
            rexegy_sys::XMISSING => Ok(Error::Missing),
            rexegy_sys::XABORT => Ok(Error::Abort),
            rexegy_sys::XBADDATA => Ok(Error::BadData),
            rexegy_sys::XAMBIG => Ok(Error::Ambiguous),
            rexegy_sys::XNOTERMS => Ok(Error::NoSearchTerms),
            rexegy_sys::XCONFIG => Ok(Error::Config),
            rexegy_sys::XNONSENSE => Ok(Error::Nonsense),
            rexegy_sys::XBADDEPTH => Ok(Error::BadDepth),
            rexegy_sys::XBADSIZE => Ok(Error::BadSize),
            rexegy_sys::XTHREAD => Ok(Error::Thread),
            rexegy_sys::XHASREGIONAL => Ok(Error::HasRegional),
            rexegy_sys::XDOWN => Ok(Error::Down),
            rexegy_sys::XERROR => Ok(Error::Error),
            rexegy_sys::XBADFIRMWARE => Ok(Error::BadFirmware),
            rexegy_sys::XUNRECOVERABLELOSS => Ok(Error::UnrecoverableLoss),
            rexegy_sys::XSYMBOLQUOTA => Ok(Error::SymbolQuota),
            rexegy_sys::XCONNLIMIT => Ok(Error::ConnnectionLimit),
            rexegy_sys::XREADONLY => Ok(Error::ReadOnly),
            rexegy_sys::XINVALIDADDR => Ok(Error::InvalidAddress),
            rexegy_sys::XEXPIRED => Ok(Error::Expired),
            rexegy_sys::XSESSIONINUSE => Ok(Error::SessionInUse),
            rexegy_sys::XSESSIONLIMIT => Ok(Error::SessionLimit),
            rexegy_sys::XINVALIDSESSION => Ok(Error::InvalidSession),
            rexegy_sys::XDISABLED => Ok(Error::Disabled),
            rexegy_sys::XDELETE => Ok(Error::Delete),
            rexegy_sys::XDEAUTHORIZE => Ok(Error::Deauthorize),
            rexegy_sys::XFORCEOFF => Ok(Error::ForceOff),
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
    type Error = Error;

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
            _ => Err(Error::try_from(value).unwrap_or(Error::Error)),
        }
    }
}

/// A local result type used to encapsulate a result and an FFI error.
pub type Result<T: ?Sized> = std::result::Result<T, Error>;
