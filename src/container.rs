//! Container Objects

pub mod callbacks;

pub use self::{
    equity::{Stream as EquityStream, StreamBuilder as EquityStreamBuilder},
    keylist::{
        Catalog as KeylistCatalog, CatalogBuilder as KeylistCatalogBuilder,
        Filter as KeylistFilter, FilterBuilder as KeylistFilterBuilder,
    },
};

mod equity;
mod keylist;

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
    /// The object type of the container.
    fn container_type(&self) -> Result<u16>;

    /// The number of slots in the container.
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

/// A trait used to retrieve the turnkey fro a container
pub(crate) trait InnerCommon: Wrapper {
    /// The turnkey value set on the container's creation.
    fn turnkey(&self) -> Result<u64>;
}

impl<T: RealTime> InnerCommon for T {
    fn turnkey(&self) -> Result<u64> {
        field::get_u64(self, rxegy_sys::XC_CONTAINER, RealTimeField::Turnkey)
    }
}

/// An enumeration of realtime fields.
#[derive(Clone, Copy, Debug)]
#[repr(u64)]
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
