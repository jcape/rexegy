//! Exegy Internal

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(i16)]
#[non_exhaustive]
pub enum Feed {
    /// Exegy Baskets (Level 1)
    Baskets = i16::from_le_bytes([b'B', b'X']),
    /// Exegy Filtered Exch Keylists (Level 1)
    FilteredExchangeKeylists = i16::from_le_bytes([b'E', b'X']),
    /// Exegy User Defined Composite Keylist (Level 1)
    UserKeylists = i16::from_le_bytes([b'I', b'X']),
    /// Exegy Options Chain Keylists (Level 1)
    OptionsChainKeylists = i16::from_le_bytes([b'O', b'X']),
    /// HW pass through (Level 1)
    Passthrough = i16::from_le_bytes([b'P', b'X']),
    /// Exegy Filtered Keylists (Level 1)
    FilteredKeylists = i16::from_le_bytes([b'R', b'X']),
    /// Exegy Filtered Subscribed Keylists (Level 1)
    FilteredSubscribedKeylists = i16::from_le_bytes([b'S', b'X']),
    /// Exegy Keylists (Level 1)
    Keylists = i16::from_le_bytes([b'U', b'X']),
    /// (Exegy Broadcast Exchange (Level 1)
    Broadcast = i16::from_le_bytes([b'\\', b'B']),
    /// (User-defined Quote Montage (Level 1)
    Montage = i16::from_le_bytes([b'\\', b'N']),
    /// (User-defined Composite (Level 1 - Exegy generate)
    Composite = i16::from_le_bytes([b'\\', b'U']),
}
