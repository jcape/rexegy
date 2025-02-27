//! Rust helpers for xcGetField, xcSetField

use crate::error::{Result, Success};
use std::{
    ffi::{CString, c_void},
    ptr::NonNull,
};

#[repr(u64)]
pub enum Field {
    SessionTurnkey = rxegy_sys::XFLD_SESS_TURNKEY,
    SessionStatus = rxegy_sys::XFLD_SESS_STATUS,
    SessionType = rxegy_sys::XFLD_SESS_SESSION_TYPE,
    SessionClientVersionString = rxegy_sys::XFLD_SESS_CLIENT_VERSION_STRING,
    SessionClientMajorVersion = rxegy_sys::XFLD_SESS_CLIENT_MAJOR_VERSION,
    SessionClientMinorVersion = rxegy_sys::XFLD_SESS_CLIENT_MINOR_VERSION,
    SessionClientRevision = rxegy_sys::XFLD_SESS_CLIENT_REVISION,
    SessionClientBuild = rxegy_sys::XFLD_SESS_CLIENT_BUILD,
    SessionClientCpuCount = rxegy_sys::XFLD_SESS_CLIENT_CPU_COUNT,
    SessionClientAffinityMask = rxegy_sys::XFLD_SESS_CLIENT_AFFINITY_MASK,
    SessionClientBgThreadAffinityMask = rxegy_sys::XFLD_SESS_CLIENT_BG_THREAD_AFFINITY_MASK,
    SessionClientHbThreadAffinityMask = rxegy_sys::XFLD_SESS_CLIENT_HB_THREAD_AFFINITY_MASK,
    SessionClientThreadPriority = rxegy_sys::XFLD_SESS_CLIENT_THREAD_PRIORITY,
    SessionClientBgThreadPriority = rxegy_sys::XFLD_SESS_CLIENT_BG_THREAD_PRIORITY,
    SessionClientHbThreadPriority = rxegy_sys::XFLD_SESS_CLIENT_HB_THREAD_PRIORITY,
    SessionServerName = rxegy_sys::XFLD_SESS_SERVER_NAME,
    SessionServerVersionString = rxegy_sys::XFLD_SESS_SERVER_VERSION_STRING,
    SessionServerMajorVersion = rxegy_sys::XFLD_SESS_SERVER_MAJOR_VERSION,
    SessionServerMinorVersion = rxegy_sys::XFLD_SESS_SERVER_MINOR_VERSION,
    SessionServerRevision = rxegy_sys::XFLD_SESS_SERVER_REVISION,
    SessionServerBuild = rxegy_sys::XFLD_SESS_SERVER_BUILD,
    SessionDisableReconnect = rxegy_sys::XFLD_SESS_DISABLE_RECONNECT,
    SessionReplayStart = rxegy_sys::XFLD_SESS_REPLAY_START,
    SessionReplayQuoteMontage = rxegy_sys::XFLD_SESS_REPLAY_QUOTE_MONTAGE,
    SessionReplayL2Composite = rxegy_sys::XFLD_SESS_REPLAY_L2_COMPOSITE,
    SessionReplayUbbo = rxegy_sys::XFLD_SESS_REPLAY_UBBO,
    SessionTickerMaxPriceBookDepth = rxegy_sys::XFLD_SESS_TKR_MAX_PRICE_BOOK_DEPTH,
    SessionTickerMarketStatusCallbacks = rxegy_sys::XFLD_SESS_TKR_MARKET_STATUS_CALLBACKS,
    SessionTickerMaxPriceBookRowLevel = rxegy_sys::XFLD_SESS_TKR_MAX_PB_ROW_LEVEL,
}

macro_rules! impl_scalar {
    ($value:ty) => {
        paste::item! {
            /// Update a field of type `$value`
            #[allow(dead_code)]
            pub fn [< set_ $value >](object: NonNull<c_void>, slot: u32, field: Field, value: $value) -> Result<()> {
                let ibuf = value.to_le_bytes();
                let status = unsafe {
                    rxegy_sys::xcSetField(
                        object.as_ptr(),
                        slot,
                        field as u64,
                        ibuf.as_ptr() as *const c_void,
                        ibuf.len() as u32,
                    )
                };

                Success::try_from(status)?;

                Ok(())
            }

            /// Retrieve the contents of the given field of type `$value`
            #[allow(dead_code)]
            pub fn [< get_ $value >](object: NonNull<c_void>, slot: u32, field: Field) -> Result<$value> {
                let mut obuf = $value::default().to_le_bytes();
                let status = unsafe {
                    rxegy_sys::xcGetField(
                        object.as_ptr(),
                        slot,
                        field as u64,
                        obuf.as_mut_ptr() as *mut c_void,
                        obuf.len() as u32,
                    )
                };

                Success::try_from(status)?;

                Ok($value::from_le_bytes(obuf))
            }
        }
    };
}

/// Retrieve the contents of the given field ID as a UTF-8 string
pub fn get_string(object: NonNull<c_void>, slot: u32, field: Field) -> Result<String> {
    let mut obuf = vec![0u8; 512];
    let status = unsafe {
        rxegy_sys::xcGetField(
            object.as_ptr(),
            slot,
            field as u64,
            obuf.as_mut_ptr() as *mut c_void,
            obuf.len() as u32,
        )
    };

    Success::try_from(status)?;

    let cstr = CString::from_vec_with_nul(obuf)?;

    Ok(cstr.to_str()?.to_owned())
}

/// Set the contents of a field value to the given string
pub fn set_string(object: NonNull<c_void>, slot: u32, field: Field, value: String) -> Result<()> {
    let ibuf = CString::new(value)?;
    let status = unsafe {
        rxegy_sys::xcSetField(
            object.as_ptr(),
            slot,
            field as u64,
            ibuf.as_ptr() as *const c_void,
            ibuf.as_bytes().len() as u32,
        )
    };

    Success::try_from(status)?;
    Ok(())
}

impl_scalar!(u8);
impl_scalar!(u16);
impl_scalar!(u32);
impl_scalar!(u64);
