//! Equity Correction Events

use crate::{event::Common, impl_wrapper_on_newtype, object::Kind as ObjectKind};
use std::{ffi::c_void, ptr::NonNull};

/// An equity correction event
#[derive(Debug)]
pub struct Event(NonNull<c_void>);

impl_wrapper_on_newtype!(Event, ObjectKind::EventEquityCorrection);

impl Common for Event {}
