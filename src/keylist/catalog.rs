//! Support for the Exegy Keylist Catalog

use crate::{
    error::{Error, Result, Success},
    event::CommonEvent,
    session::Session,
};
use rxegy_sys::xhandle;
use std::{ffi::c_void, ptr::NonNull, result::Result as StdResult};

/// A trait used to provide callback marshalling
pub trait Callbacks {
    /// A subscription result is ready
    fn subscribe(&self, catalog: &Catalog, event: &SubscribeEvent);

    /// A catalog refresh event has been received
    fn refresh(&self, catalog: &Catalog, event: &RefreshEvent);

    /// A catalog update event has been received
    fn update(&self, catalog: &Catalog, event: &UpdateEvent);
}

/// An XCAPI object containing a keylist catalog
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Catalog(NonNull<c_void>);

impl AsRef<NonNull<c_void>> for Catalog {
    fn as_ref(&self) -> &NonNull<c_void> {
        &self.0
    }
}

impl Catalog {
    pub fn new(_session: &Session, _callbacks: Box<dyn Callbacks>) -> Result<Catalog> {
        todo!()
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

impl CommonEvent for SubscribeEvent {}

/// An XCAPI object containing a refresh event.
#[repr(transparent)]
pub struct RefreshEvent(NonNull<c_void>);

impl AsRef<NonNull<c_void>> for RefreshEvent {
    fn as_ref(&self) -> &NonNull<c_void> {
        &self.0
    }
}

impl CommonEvent for RefreshEvent {}

/// An XCAPI object containg an update event.
#[repr(transparent)]
pub struct UpdateEvent(NonNull<c_void>);

impl AsRef<NonNull<c_void>> for UpdateEvent {
    fn as_ref(&self) -> &NonNull<c_void> {
        &self.0
    }
}

impl CommonEvent for UpdateEvent {}

/// An enumeration of event object types.
#[derive(Clone, Copy)]
#[repr(u16)]
enum EventKind {
    Subscribe = rxegy_sys::XOBJ_EVENT_SUBSCRIBE,
    Refresh = rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH,
    Update = rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE,
}

impl TryFrom<u16> for EventKind {
    type Error = Error;

    fn try_from(value: u16) -> StdResult<Self, Self::Error> {
        match value {
            rxegy_sys::XOBJ_EVENT_SUBSCRIBE => Ok(EventKind::Subscribe),
            rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH => Ok(EventKind::Refresh),
            rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE => Ok(EventKind::Update),
            _ => Err(Error::UnexpectedKind),
        }
    }
}

/// An inner enumeration used for callback dispatch
#[allow(dead_code)]
enum Event {
    /// A subscription event
    Subscribe(SubscribeEvent),
    /// A refresh event
    Refresh(RefreshEvent),
    /// An update event
    Update(UpdateEvent),
}

impl TryFrom<xhandle> for Event {
    type Error = Error;

    fn try_from(value: xhandle) -> StdResult<Self, Self::Error> {
        let mut object_type = 0u16;
        let status = unsafe { rxegy_sys::xcObjectType(value, &mut object_type) };

        Success::try_from(status)?;

        let kind = EventKind::try_from(object_type)?;
        let ptr = NonNull::new(value).ok_or(Error::NullObject)?;
        match kind {
            EventKind::Subscribe => Ok(Event::Subscribe(SubscribeEvent(ptr))),
            EventKind::Refresh => Ok(Event::Refresh(RefreshEvent(ptr))),
            EventKind::Update => Ok(Event::Update(UpdateEvent(ptr))),
        }
    }
}
