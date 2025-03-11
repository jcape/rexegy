//! Support for the Exegy Keylist Catalog

use crate::{
    container::{
        InnerCommon, RealTime,
        callbacks::{KeylistCatalogRefreshFn, KeylistCatalogSubscribeFn, KeylistCatalogUpdateFn},
    },
    error::{Error, Result, Success},
    event::{KeylistCatalogRefresh, KeylistCatalogUpdate, Subscribe as SubscribeEvent},
    impl_wrapper_on_newtype,
    object::{Kind as ObjectKind, Wrapper},
    session::TickerSession,
};
use rxegy_sys::{xerr, xhandle};
use std::{
    any::Any,
    ffi::{CStr, c_void},
    process,
    ptr::{self, NonNull},
};

/// An XCAPI object containing a keylist catalog
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Catalog(NonNull<c_void>);

impl_wrapper_on_newtype!(Catalog, ObjectKind::RealtimeKeylistCatalog);

impl RealTime for Catalog {}

/// A builder for constructing a keylist catalog
#[derive(Default)]
pub struct Builder {
    subscribe: Option<KeylistCatalogSubscribeFn>,
    refresh: Option<KeylistCatalogRefreshFn>,
    update: Option<KeylistCatalogUpdateFn>,
}

impl Builder {
    /// Set the callback to be fired when a subscribe event is received
    pub fn on_subscribe(mut self, func: KeylistCatalogSubscribeFn) -> Self {
        self.subscribe = Some(func);
        self
    }

    /// Set the callback to be fired when a refresh event is received
    pub fn on_refresh(mut self, func: KeylistCatalogRefreshFn) -> Self {
        self.refresh = Some(func);
        self
    }

    /// Set the callback to be fired when an update event is received
    pub fn on_update(mut self, func: KeylistCatalogUpdateFn) -> Self {
        self.update = Some(func);
        self
    }

    /// Build a new keylist catalog using the given session, and supplying the given user data to callbacks.
    pub fn build(
        self,
        session: &TickerSession,
        user_data: Option<Box<dyn Any>>,
    ) -> Result<Catalog> {
        let catalog = self.create_catalog(session)?;

        if let Err(e) = Self::subscribe(&catalog, user_data) {
            let mut object = catalog.as_xhandle();
            let status = unsafe { rxegy_sys::xcDestroyObject(&mut object) };
            Success::try_from(status)?;
            return Err(e);
        }

        Ok(catalog)
    }

    /// Create the catalog object, consuming ourseles in the process.
    fn create_catalog(self, session: &TickerSession) -> Result<Catalog> {
        let context = Box::new(self) as Box<dyn Any>;
        let thin_ptr = Box::new(context);
        let turnkey = Box::into_raw(thin_ptr) as u64;
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

        if let Err(e) = Success::try_from(status) {
            // Don't leak the builder if we failed to create the catalog
            let _context = unsafe { Box::from_raw(turnkey as *mut Builder) };
            return Err(e.into());
        };

        Catalog::from_xhandle(object)
    }

    /// Static helper function to subscribe to the empty string and start the catalog sync process.
    fn subscribe(catalog: &Catalog, user_data: Option<Box<dyn Any>>) -> Result<()> {
        let turnkey = if let Some(fat) = user_data {
            let thin = Box::new(fat);
            Box::into_raw(thin) as u64
        } else {
            0u64
        };

        let mut slot = rxegy_sys::XC_NEXT_AVAILABLE_SLOT;
        let key_string = CStr::from_bytes_until_nul(b"")?;

        let status = unsafe {
            rxegy_sys::xcRequestItemByString(
                catalog.as_xhandle(),
                key_string.as_ptr(),
                turnkey,
                &mut slot,
            )
        };

        Success::try_from(status)?;

        Ok(())
    }

    /// Dispatch the event
    fn dispatch(
        &self,
        catalog: &Catalog,
        _slot: u32,
        event_handle: xhandle,
        event_type: u16,
        user_data: Option<&Box<dyn Any>>,
        _status: xerr,
    ) -> Result<()> {
        match EventKind::try_from(event_type)? {
            EventKind::Subscribe => {
                if let Some(func) = self.subscribe {
                    let event = SubscribeEvent::from_xhandle_and_type(event_handle, event_type)?;
                    if let Some(user_data) = user_data {
                        func(catalog, &event, Some(user_data.as_ref()))?;
                    } else {
                        func(catalog, &event, None)?;
                    }
                }
            }
            EventKind::Refresh => {
                if let Some(func) = self.refresh {
                    let event =
                        KeylistCatalogRefresh::from_xhandle_and_type(event_handle, event_type)?;
                    if let Some(user_data) = user_data {
                        func(catalog, &event, Some(user_data.as_ref()))?;
                    } else {
                        func(catalog, &event, None)?;
                    }
                }
            }
            EventKind::Update => {
                if let Some(func) = self.update {
                    let event =
                        KeylistCatalogUpdate::from_xhandle_and_type(event_handle, event_type)?;
                    if let Some(user_data) = user_data {
                        func(catalog, &event, Some(user_data.as_ref()))?;
                    } else {
                        func(catalog, &event, None)?;
                    }
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
    slot: u32,
    event_handle: xhandle,
    event_type: u16,
    turnkey: u64,
    status: xerr,
) {
    if let Err(_e) = std::panic::catch_unwind(|| {
        tracing::trace_span!("_rxegy_catalog_callback");

        let catalog = match Catalog::from_xhandle(handle) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!(
                    "Could not read catalog from handle in catalog callback: {}",
                    e
                );
                return;
            }
        };

        let context_turnkey = match catalog.turnkey() {
            Ok(ctk) => ctk,
            Err(e) => {
                tracing::error!("Could not read catalog turnkey to get context: {}", e);
                return;
            }
        };

        let context_thin_raw = context_turnkey as *mut Box<dyn Any>;
        if context_thin_raw.is_null() {
            tracing::error!("Catalog turnkey was null");
            return;
        }

        let context_thin = unsafe { Box::from_raw(context_thin_raw) };
        let context = match (**context_thin).downcast_ref::<Context>() {
            Some(ctx) => ctx,
            None => {
                tracing::error!("Could not downcast the catalog context from a thin pointer");
                return;
            }
        };

        let user_data = if turnkey == 0 {
            None
        } else {
            let user_data_thin_raw = turnkey as *mut Box<dyn Any>;
            Some(unsafe { Box::from_raw(user_data_thin_raw) })
        };

        if let Err(e) = context.dispatch(
            &catalog,
            slot,
            event_handle,
            event_type,
            user_data.as_deref(),
            status,
        ) {
            tracing::error!("The callback returned an error: {}", e);
        }

        // Release the user data pointer
        if let Some(user_data) = user_data {
            let _leaked = Box::into_raw(user_data);
        }

        // Release the context pointer
        let _leaked = Box::into_raw(context_thin);
    }) {
        tracing::error!("Panic in Keylist Catalog callback, aborting...");
        process::abort()
    }
}
