//! Exegy key support

use crate::{
    error::{Error, Result, Success},
    feed::Id as FeedId,
    group::{Group, Id as GroupId},
};
use ref_cast::RefCast;
use rxegy_sys::{XC_ALTERNATE_ID, XC_FORMAT_CONTROL, XC_KEY, XC_SYMBOL};
use std::{
    ffi::CStr,
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
};

/// A wrapper for an Exegy key
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct Key(XC_KEY);

impl Key {
    /// Wrap a new key
    pub(crate) fn new(inner: XC_KEY) -> Key {
        Key(inner)
    }

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

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut outbuf = [0i8; 81];
        let mut ctrl = XC_FORMAT_CONTROL::default();
        let status = unsafe {
            rxegy_sys::xcFmtInitialize(&mut ctrl, outbuf.as_mut_ptr(), outbuf.len() as u32)
        };

        Success::try_from(status).map_err(|_e| FmtError)?;

        let status = unsafe { rxegy_sys::xcFmtKey(&mut ctrl, &self.0) };

        Success::try_from(status).map_err(|_e| FmtError)?;

        let outbuf = unsafe { CStr::from_ptr(outbuf.as_mut_ptr()) };

        write!(f, "{}", outbuf.to_str().map_err(|_e| FmtError)?)
    }
}

/// Exegy symbol data.
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord, RefCast)]
#[repr(transparent)]
pub struct Symbol(XC_SYMBOL);

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut outbuf = [0i8; 81];
        let mut ctrl = XC_FORMAT_CONTROL::default();
        let status = unsafe {
            rxegy_sys::xcFmtInitialize(&mut ctrl, outbuf.as_mut_ptr(), outbuf.len() as u32)
        };

        Success::try_from(status).map_err(|_e| FmtError)?;

        let status = unsafe { rxegy_sys::xcFmtSymbol(&mut ctrl, &self.0) };

        Success::try_from(status).map_err(|_e| FmtError)?;

        let outbuf = unsafe { CStr::from_ptr(outbuf.as_mut_ptr()) };

        write!(f, "{}", outbuf.to_str().map_err(|_e| FmtError)?)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct AlternateId(XC_ALTERNATE_ID);

impl AlternateId {
    #[inline(always)]
    pub(crate) fn new(inner: XC_ALTERNATE_ID) -> Self {
        AlternateId(inner)
    }
}
