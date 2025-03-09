//! Rust helpers for xcGetField, xcSetField

use crate::{
    error::{Result, Success},
    object::Wrapper,
};
use rxegy_sys::{XC_GROUP_EVENT_TIMING, XC_KEY};
use std::ffi::{CStr, CString};

/// A marker trait for field types
pub trait Field {
    /// Get the u64 value for a given field ID.
    fn to_u64(&self) -> u64;
}

macro_rules! impl_getter {
    ($value:ty, $funcname:ident, $xcfunc:ident) => {
        /// Retrieve the contents of the given field of type `$value`
        pub(crate) fn $funcname<O: $crate::object::Wrapper, F: $crate::field::Field>(
            object: &O,
            slot: u32,
            field: F,
        ) -> $crate::error::Result<$value> {
            let mut obuf = <$value as Default>::default();
            let status = unsafe {
                rxegy_sys::$xcfunc(
                    object.as_xhandle(),
                    slot,
                    field.to_u64(),
                    &mut obuf as *mut $value as *mut _,
                    ::std::mem::size_of::<$value>() as u32,
                )
            };

            $crate::error::Success::try_from(status)?;

            Ok(obuf)
        }
    };
}

macro_rules! impl_setter {
    ($value:ty, $funcname:ident, $xcfunc:ident) => {
        /// Retrieve the contents of the given field of type `$value`
        pub(crate) fn $funcname<O: $crate::object::Wrapper, F: $crate::field::Field>(
            object: &O,
            slot: u32,
            field: F,
            ibuf: $value,
        ) -> Result<()> {
            let status = unsafe {
                rxegy_sys::$xcfunc(
                    object.as_xhandle(),
                    slot,
                    field.to_u64(),
                    &ibuf as *const $value as *const _,
                    ::std::mem::size_of::<$value>() as u32,
                )
            };

            $crate::error::Success::try_from(status)?;

            Ok(())
        }
    };
}

/// Retrieve the contents of the given field ID as a UTF-8 string
pub fn get_string<O: Wrapper, F: Field>(object: &O, slot: u32, field: F) -> Result<String> {
    let mut obuf = [0u8; 512];
    let status = unsafe {
        rxegy_sys::xcGetField(
            object.as_xhandle(),
            slot,
            field.to_u64(),
            obuf.as_mut_ptr() as *mut _,
            obuf.len() as u32,
        )
    };

    Success::try_from(status)?;

    Ok(CStr::from_bytes_until_nul(obuf.as_slice())?
        .to_str()?
        .to_owned())
}

/// Set the contents of a field value to the given string
pub fn set_string<O: Wrapper, F: Field>(
    object: &O,
    slot: u32,
    field: F,
    value: String,
) -> Result<()> {
    let ibuf = CString::new(value)?;
    let status = unsafe {
        rxegy_sys::xcSetField(
            object.as_xhandle(),
            slot,
            field.to_u64(),
            ibuf.as_ptr() as *const _,
            ibuf.as_bytes().len() as u32,
        )
    };

    Success::try_from(status)?;
    Ok(())
}

/// Get the contents of a non-nul-terminated fixed length string
pub fn get_fixedstring<O: Wrapper, F: Field>(
    object: &O,
    slot: u32,
    field: F,
    length: usize,
) -> Result<String> {
    let mut obuf = vec![0u8; length];
    let status = unsafe {
        rxegy_sys::xcGetField(
            object.as_xhandle(),
            slot,
            field.to_u64(),
            obuf.as_mut_ptr() as *mut _,
            obuf.len() as u32,
        )
    };

    Success::try_from(status)?;

    Ok(String::from_utf8(obuf)?)
}

impl_getter!(u8, get_u8, xcGetField);
impl_setter!(u8, set_u8, xcSetField);

impl_getter!(u16, get_u16, xcGetField);

impl_getter!(u32, get_u32, xcGetField);
impl_setter!(u32, set_u32, xcSetField);

impl_getter!(u64, get_u64, xcGetField);
impl_setter!(u64, set_u64, xcSetField);

impl_getter!(XC_KEY, get_xc_key, xcGetField);
impl_getter!(
    XC_GROUP_EVENT_TIMING,
    get_xc_group_event_timing,
    xcGetFieldGroup
);
