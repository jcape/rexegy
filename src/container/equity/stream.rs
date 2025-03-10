//! Top-of-book Containers for Equity and Equity Options Streams

use crate::{impl_wrapper_on_newtype, object::Kind as ObjectKind};
use std::{ffi::c_void, ptr::NonNull};

#[derive(Debug)]
pub struct Stream(NonNull<c_void>);

impl_wrapper_on_newtype!(Stream, ObjectKind::RealtimeEquityStream);

impl Stream {}

pub struct Builder {}
