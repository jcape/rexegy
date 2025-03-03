//! Support for the Exegy Keylist Catalog

use crate::{
    error::{Error, Result, Success},
    event::CommonEvent,
    object::Kind as ObjectKind,
    session::Session,
};
use rxegy_sys::{xerr, xhandle};
use std::{
    any::Any,
    ffi::c_void,
    ptr::{self, NonNull},
    sync::Arc,
};

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
    pub fn new(
        session: &Session,
        max_slots: u32,
        callbacks: Arc<dyn Callbacks>,
    ) -> Result<Catalog> {
        let mut object = ptr::null_mut();
        let status = unsafe {
            // Create a fat pointer to a reference to the arc
            let fat_ptr = &callbacks as &dyn Any;
            // Create a pointer to the fat pointer
            let boxed_fat_ptr = Box::new(fat_ptr);
            // Cast the double-pointer to a u64
            let turnkey = Box::into_raw(boxed_fat_ptr) as u64;

            rxegy_sys::xcCreateContainer(
                session.as_ref().as_ptr(),
                ObjectKind::RealtimeKeylistCatalog as u16,
                &mut object,
                Some(_rxegy_catalog_callback),
                turnkey,
                max_slots,
            )
        };

        Success::try_from(status)?;

        let object = NonNull::new(object).ok_or(Error::NullObject)?;

        Ok(Catalog(object))
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
    // TODO: log panics
    let _ = std::panic::catch_unwind(|| {
        tracing::trace_span!("rxegy::keylist::catalog::_rxegy_catalog_callback");

        // Get the catalog handle ptr
        let object = match NonNull::new(handle) {
            Some(ptr) => ptr,
            None => {
                tracing::error!("Callback without a valid container handle");
                return;
            }
        };

        // Get the handle ptr
        let event = match NonNull::new(event_handle) {
            Some(ptr) => ptr,
            None => {
                tracing::error!("Callback without a valid event handle");
                return;
            }
        };

        let catalog = Catalog(object);

        // Get our context
        let boxed_fat_ptr = unsafe {
            let fat_ptr = turnkey as *mut &dyn Any;
            Box::from_raw(fat_ptr)
        };

        if let Some(callbacks) = boxed_fat_ptr.downcast_ref::<Arc<dyn Callbacks>>() {
            // Take a clone here to ensure reference survives
            let cb = callbacks.clone();
            match event_type {
                rxegy_sys::XOBJ_EVENT_SUBSCRIBE => {
                    cb.subscribe(&catalog, &SubscribeEvent(event));
                }
                rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_REFRESH => {
                    cb.refresh(&catalog, &RefreshEvent(event));
                }
                rxegy_sys::XOBJ_EVENT_KEYLIST_CATALOG_UPDATE => {
                    cb.update(&catalog, &UpdateEvent(event));
                }
                _ => {
                    tracing::error!("Received catalog callback with unknown event type")
                }
            }
        }

        let _leaked = Box::into_raw(boxed_fat_ptr);
    });
}
