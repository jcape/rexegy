//! Exegy Internal

use super::Feed as FeedTrait;
use crate::group::{Corporate, Group};

#[derive(Copy, Clone, Debug, displaydoc::Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
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

impl TryFrom<i16> for Feed {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::try_from(value.to_le_bytes())
    }
}

impl TryFrom<[u8; 2]> for Feed {
    type Error = ();

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [b'B', b'X'] => Ok(Feed::Baskets),
            [b'E', b'X'] => Ok(Feed::FilteredExchangeKeylists),
            [b'I', b'X'] => Ok(Feed::UserKeylists),
            [b'O', b'X'] => Ok(Feed::OptionsChainKeylists),
            [b'P', b'X'] => Ok(Feed::Passthrough),
            [b'R', b'X'] => Ok(Feed::FilteredKeylists),
            [b'S', b'X'] => Ok(Feed::FilteredSubscribedKeylists),
            [b'U', b'X'] => Ok(Feed::Keylists),
            [b'\\', b'B'] => Ok(Feed::Broadcast),
            [b'\\', b'N'] => Ok(Feed::Montage),
            [b'\\', b'U'] => Ok(Feed::Composite),
            _ => Err(()),
        }
    }
}

impl FeedTrait for Feed {
    fn mic(&self) -> Option<i32> {
        None
    }

    fn group(&self) -> Group {
        Group::Corporate(Corporate::ExegyInternal)
    }
}
