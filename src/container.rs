//! Container Objects

pub use self::equity::{Stream as EquityStream, StreamBuilder as EquityStreamBuilder};

mod equity;

use crate::{
    error::Result,
    field::{self, Field},
    object::Wrapper,
};

/// A marker trait used to identify realtime containers
pub(crate) trait RealTime: Wrapper {}

/// Common fields used by containers
#[allow(private_bounds)]
pub trait Common: Wrapper {
    /// The type ID of the container
    fn container_type(&self) -> Result<u16>;

    /// Retreive the number of slots configured on this container
    fn slot_count(&self) -> Result<u32>;
}

impl<T: RealTime> Common for T {
    fn container_type(&self) -> Result<u16> {
        field::get_u16(self, rxegy_sys::XC_CONTAINER, RealTimeField::Type)
    }

    fn slot_count(&self) -> Result<u32> {
        field::get_u32(self, rxegy_sys::XC_CONTAINER, RealTimeField::SlotCount)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
#[allow(dead_code)]
enum RealTimeField {
    /// The number of slots configured on this container
    SlotCount = rxegy_sys::XFLD_RT_SLOT_COUNT,
    /// The context pointer turnkey set when the container was created
    Turnkey = rxegy_sys::XFLD_RT_TURNKEY,
    /// The type of container
    Type = rxegy_sys::XFLD_RT_CONTAINER_TYPE,
}

impl Field for RealTimeField {
    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
