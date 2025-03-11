//! Keylist Filters

use crate::{
    container::RealTime as RealTimeContainer, impl_wrapper_on_newtype, object::Kind as ObjectKind,
};
use std::{ffi::c_void, ptr::NonNull};

/// The realtime keylist container
#[derive(Debug)]
pub struct Filter(NonNull<c_void>);

impl_wrapper_on_newtype!(Filter, ObjectKind::RealtimeKeylistFilter);

impl RealTimeContainer for Filter {}

/// The realtime keylist container builder
pub struct Builder {}
