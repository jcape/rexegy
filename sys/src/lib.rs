//! Unofficial Rust FFI for XCAPI

pub use self::generated::*;

/// A module for the generated stubs.
#[allow(
    unsafe_op_in_unsafe_fn, // FIXME: remove when bindgen has been fixed
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc,
    clippy::ptr_offset_with_cast,
    clippy::useless_transmute,
    clippy::unnecessary_cast,
    clippy::too_many_arguments

)]
mod generated {
    #[rustfmt::skip]
    include!("gen.inc.rs");
}

/// Check whether a code is good or not
pub fn xerr_is_good(code: xerr) -> bool {
    (code & 1) == 0
}

/// Check whether a code is bad or not
pub fn xerr_is_bad(code: xerr) -> bool {
    (code & 1) != 0
}

macro_rules! impl_common_for_union {
    ($name:ty, $member:ident) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {{ u8_: {:?} }}", stringify!($name), unsafe {
                    self.$member
                })
            }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                unsafe { self.$member }.hash(state)
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                unsafe { self.$member }.cmp(&unsafe { other.$member })
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                unsafe { self.$member }.eq(&unsafe { other.$member })
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    };
}

impl_common_for_union!(XC_TRADING_STATE, u8_);
impl_common_for_union!(XC_DATE, xdt_raw);
impl_common_for_union!(XC_REFRESH_QUALS, xrq_u32);
