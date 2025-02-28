//! Rust helpers for xcGetField, xcSetField

use crate::{
    error::{Result, Success},
    key::Key,
    timing::EventTiming,
};
use std::{
    ffi::{CStr, CString, c_void},
    ptr::NonNull,
};

/// A marker trait for field types
pub trait Field {
    /// Get the u64 value for a given field ID.
    fn to_u64(&self) -> u64;
}

macro_rules! impl_struct_group {
    ($value:ty) => {
        paste::item! {
            /// Update a field of type `$value`
            #[allow(dead_code)]
            pub(crate) fn [< set_ $value:lower >]<F: $crate::field::Field>(object: std::ptr::NonNull<std::ffi::c_void>, slot: u32, field: F, value: &$value) -> $crate::error::Result<()> {
                const IBUFSIZ: usize = std::mem::size_of::<$value>();
                let status = unsafe {
                    let ibuf = std::mem::transmute_copy::<$value, [u8; IBUFSIZ]>(value);
                    rxegy_sys::xcSetFieldGroup(
                        object.as_ptr(),
                        slot,
                        field.to_u64(),
                        ibuf.as_ptr() as *const c_void,
                        IBUFSIZ as u32,
                    )
                };

                $crate::error::Success::try_from(status)?;

                Ok(())
            }

            /// Retrieve the contents of the given field of type `$value`
            #[allow(dead_code)]
            pub(crate) fn [< get_ $value:lower >]<F: $crate::field::Field>(object: std::ptr::NonNull<std::ffi::c_void>, slot: u32, field: F) -> $crate::error::Result<$value> {
                const OBUFSIZ: usize = std::mem::size_of::<$value>();
                let mut obuf = [0u8; OBUFSIZ];
                let status = unsafe {
                    rxegy_sys::xcGetFieldGroup(
                        object.as_ptr(),
                        slot,
                        field.to_u64(),
                        obuf.as_mut_ptr() as *mut c_void,
                        OBUFSIZ as u32,
                    )
                };

                $crate::error::Success::try_from(status)?;

                Ok(unsafe { std::mem::transmute::<[u8; OBUFSIZ], $value>(obuf)})
            }
        }
    };
}

macro_rules! impl_scalar {
    ($value:ty) => {
        paste::item! {
            /// Update a field of type `$value`
            #[allow(dead_code)]
            pub(crate) fn [< set_ $value >]<F: $crate::field::Field>(object: std::ptr::NonNull<std::ffi::c_void>, slot: u32, field: F, value: $value) -> Result<()> {
                let ibuf = value.to_le_bytes();
                let status = unsafe {
                    rxegy_sys::xcSetField(
                        object.as_ptr(),
                        slot,
                        field.to_u64(),
                        ibuf.as_ptr() as *const c_void,
                        ibuf.len() as u32,
                    )
                };

                $crate::error::Success::try_from(status)?;

                Ok(())
            }

            /// Retrieve the contents of the given field of type `$value`
            #[allow(dead_code)]
            pub(crate) fn [< get_ $value >]<F: $crate::field::Field>(object: std::ptr::NonNull<c_void>, slot: u32, field: F) -> Result<$value> {
                let mut obuf = $value::default().to_le_bytes();
                let status = unsafe {
                    rxegy_sys::xcGetField(
                        object.as_ptr(),
                        slot,
                        field.to_u64(),
                        obuf.as_mut_ptr() as *mut c_void,
                        obuf.len() as u32,
                    )
                };

                $crate::error::Success::try_from(status)?;

                Ok($value::from_le_bytes(obuf))
            }
        }
    };
}

/// Retrieve the contents of the given field ID as a UTF-8 string
pub fn get_string<F: Field>(object: NonNull<c_void>, slot: u32, field: F) -> Result<String> {
    let mut obuf = [0u8; 512];
    let status = unsafe {
        rxegy_sys::xcGetField(
            object.as_ptr(),
            slot,
            field.to_u64(),
            obuf.as_mut_ptr() as *mut c_void,
            obuf.len() as u32,
        )
    };

    Success::try_from(status)?;

    Ok(CStr::from_bytes_until_nul(obuf.as_slice())?
        .to_str()?
        .to_owned())
}

/// Set the contents of a field value to the given string
pub fn set_string<F: Field>(
    object: NonNull<c_void>,
    slot: u32,
    field: F,
    value: String,
) -> Result<()> {
    let ibuf = CString::new(value)?;
    let status = unsafe {
        rxegy_sys::xcSetField(
            object.as_ptr(),
            slot,
            field.to_u64(),
            ibuf.as_ptr() as *const c_void,
            ibuf.as_bytes().len() as u32,
        )
    };

    Success::try_from(status)?;
    Ok(())
}

/// Get the contents of a non-nul-terminated fixed length string
pub fn get_fixedstring<F: Field>(
    object: NonNull<c_void>,
    slot: u32,
    field: F,
    length: usize,
) -> Result<String> {
    let mut obuf = vec![0u8; length];
    let status = unsafe {
        rxegy_sys::xcGetField(
            object.as_ptr(),
            slot,
            field.to_u64(),
            obuf.as_mut_ptr() as *mut c_void,
            obuf.len() as u32,
        )
    };

    Success::try_from(status)?;

    Ok(String::from_utf8(obuf)?)
}

impl_scalar!(u8);
impl_scalar!(u16);
impl_scalar!(u32);
impl_scalar!(u64);
impl_struct_group!(EventTiming);
impl_struct_group!(Key);
