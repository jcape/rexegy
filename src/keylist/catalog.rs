//! Support for the Exegy Keylist Catalog

use crate::error::Error;
use rxegy_sys::xhandle;
use std::{ffi::c_void, ptr::NonNull};

/// An XCAPI object containing a keylist catalog
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Catalog(NonNull<c_void>);

impl AsRef<NonNull<c_void>> for Catalog {
    fn as_ref(&self) -> &NonNull<c_void> {
        &self.0
    }
}

impl TryFrom<xhandle> for Catalog {
    type Error = Error;

    fn try_from(ptr: xhandle) -> Result<Self, Self::Error> {
        Ok(Catalog(NonNull::new(ptr).ok_or(Error::NullObject)?))
    }
}

/// An XCAPI object containg a subscribe event.
#[repr(transparent)]
pub struct SubscribeEvent(NonNull<c_void>);

impl AsRef<NonNull<c_void>> for SubscribeEvent {
    fn as_ref(&self) -> &NonNull<c_void> {
        &self.0
    }
}

impl TryFrom<xhandle> for SubscribeEvent {
    type Error = Error;

    fn try_from(_ptr: xhandle) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// An XCAPI object containing a refresh event.
#[repr(transparent)]
pub struct RefreshEvent(NonNull<c_void>);

/// An XCAPI object containg an update event.
#[repr(transparent)]
pub struct UpdateEvent(NonNull<c_void>);

/// An inner enumeration used for callback dispatch
pub enum Event {
    /// A subscription event
    Subscribe(SubscribeEvent),
    /// A refresh event
    Refresh(RefreshEvent),
    /// An update event
    Update(UpdateEvent),
}
