//! Exegy key support

use crate::{
    error::{Error, Result},
    feed::Id as FeedId,
    group::{Group, Id as GroupId},
};
use ref_cast::RefCast;

/// A wrapper for an Exegy key
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct Key(rxegy_sys::XC_KEY);

impl Key {
    /// Retrieve a reference to the exchange ID in this key
    pub fn feed_id(&self) -> &FeedId {
        FeedId::ref_cast(&self.0.xk_exchange)
    }

    /// Retrieve a reference to the country code in this key
    pub fn group_id(&self) -> &GroupId {
        GroupId::ref_cast(&self.0.xk_country)
    }

    /// Parse the group and return the group if possible.
    pub fn group(&self) -> Result<Group> {
        Group::try_from(*self.group_id()).map_err(|_e| Error::GroupUnknown)
    }

    /// Retrieve a reference to the symbol in this key
    pub fn symbol(&self) -> &Symbol {
        Symbol::ref_cast(&self.0.xk_symbol)
    }
}

/// Exegy symbol data.
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct Symbol(rxegy_sys::XC_SYMBOL);
