//! Generic Subscription Event

use super::Common;
use crate::{impl_wrapper_on_newtype, object::Kind as ObjectKind};
use std::{ffi::c_void, ptr::NonNull};

/// An XCAPI object containg a subscribe event.
#[derive(Debug)]
#[repr(transparent)]
pub struct Event(NonNull<c_void>);

impl_wrapper_on_newtype!(Event, ObjectKind::EventSubscribe);

impl Common for Event {}
