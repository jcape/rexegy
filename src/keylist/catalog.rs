//! Support for the Exegy Keylist Catalog

use crate::{
    error::{Error, Result, Success},
    event::{Common as CommonEvent, Subscribe as SubscribeEvent},
    impl_wrapper_on_newtype,
    object::{Kind as ObjectKind, Wrapper},
    session::TickerSession,
};
use rxegy_sys::{xerr, xhandle};
use std::{
    ffi::c_void,
    process,
    ptr::{self, NonNull},
};

/// An XCAPI object containing a refresh event.
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RefreshEvent(NonNull<c_void>);

impl_wrapper_on_newtype!(RefreshEvent, ObjectKind::EventKeylistCatalogRefresh);

impl CommonEvent for RefreshEvent {}

/// An XCAPI object containg an update event.
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UpdateEvent(NonNull<c_void>);

impl_wrapper_on_newtype!(UpdateEvent, ObjectKind::EventKeylistCatalogUpdate);

impl CommonEvent for UpdateEvent {}

/// An XCAPI object containing a keylist catalog
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Catalog(NonNull<c_void>);

impl_wrapper_on_newtype!(Catalog, ObjectKind::RealtimeKeylistCatalog);

/// A callback signature for subscription event handlers.
pub type SubscribeFn = fn(catalog: &Catalog, event: &SubscribeEvent) -> Result<()>;

/// A callback signature for catalog refresh event handlers.
pub type RefreshFn = fn(catalog: &Catalog, event: &RefreshEvent) -> Result<()>;

/// A callback signature for catalog update event handlers.
pub type UpdateFn = fn(catalog: &Catalog, event: &UpdateEvent) -> Result<()>;

/// A builder for constructing a keylist catalog
#[derive(Default)]
pub struct Builder {
    subscribe: Option<SubscribeFn>,
    refresh: Option<RefreshFn>,
    update: Option<UpdateFn>,
}

impl Builder {
    /// Set the callback to be fired when a subscribe event is received
    pub fn on_subscribe(mut self, func: SubscribeFn) -> Self {
        self.subscribe = Some(func);
        self
    }

    /// Set the callback to be fired when a refresh event is received
    pub fn on_refresh(mut self, func: RefreshFn) -> Self {
        self.refresh = Some(func);
        self
    }

    /// Set the callback to be fired when an update event is received
    pub fn on_update(mut self, func: UpdateFn) -> Self {
        self.update = Some(func);
        self
    }

    pub fn build(self, session: &TickerSession) -> Result<Catalog> {
        let context = Box::new(self);
        let turnkey = Box::into_raw(context) as u64;
        let mut object = ptr::null_mut();

        let status = unsafe {
            rxegy_sys::xcCreateContainer(
                session.as_xhandle(),
                ObjectKind::RealtimeKeylistCatalog as u16,
                &mut object,
                Some(_rxegy_catalog_callback),
                turnkey,
                10,
            )
        };

        Success::try_from(status)?;

        Catalog::from_xhandle(object)
    }

    /// Dispatch the event
    fn dispatch(&self, handle: xhandle, event_handle: xhandle, event_type: u16) -> Result<()> {
        let catalog = Catalog::from_xhandle(handle)?;

        match EventKind::try_from(event_type)? {
            EventKind::Subscribe => {
                if let Some(func) = self.subscribe {
                    let event = SubscribeEvent::from_xhandle_and_type(event_handle, event_type)?;
                    return func(&catalog, &event);
                }
            }
            EventKind::Refresh => {
                if let Some(func) = self.refresh {
                    let event = RefreshEvent::from_xhandle_and_type(event_handle, event_type)?;
                    return func(&catalog, &event);
                }
            }
            EventKind::Update => {
                if let Some(func) = self.update {
                    let event = UpdateEvent::from_xhandle_and_type(event_handle, event_type)?;
                    return func(&catalog, &event);
                }
            }
        }

        Ok(())
    }
}

type Context = Builder;

#[repr(u16)]
enum EventKind {
    Subscribe = rxegy_sys::XOBJ_EVENT_SUBSCRIBE,
    Refresh = rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH,
    Update = rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE,
}

impl TryFrom<u16> for EventKind {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        match value {
            rxegy_sys::XOBJ_EVENT_SUBSCRIBE => Ok(Self::Subscribe),
            rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH => Ok(Self::Refresh),
            rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE => Ok(Self::Update),
            _ => Err(Error::ObjectUnknown),
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _rxegy_catalog_callback(
    handle: xhandle,
    _slot: u32,
    event_handle: xhandle,
    event_type: u16,
    turnkey: u64,
    _status: xerr,
) {
    // Get our context outside the panic handler
    let context = if let Ok(ctx) = std::panic::catch_unwind(|| unsafe {
        let ptr = turnkey as *mut Context;
        Box::from_raw(ptr)
    }) {
        ctx
    } else {
        tracing::error!("Panic while retrieving context, halting application");
        process::abort();
    };

    if let Err(_e) = std::panic::catch_unwind(|| {
        tracing::trace_span!("rxegy::keylist::catalog::_rxegy_catalog_callback");

        if let Err(e) = context.dispatch(handle, event_handle, event_type) {
            tracing::error!("The callback returned an error: {}", e);
        }
    }) {
        tracing::error!(
            "Panic at the callback, allowing the application to continue, but user locks may be poisoined, and the context"
        );
    }

    // leak our context so it isn't freed
    let _leaked = Box::into_raw(context);
}
