//! Unofficial Rust FFI for XCAPI

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
    include!("xerr.inc.rs");
    include!("gen.inc.rs");
    include!("xcvt.inc.rs");
}

pub use generated::*;

/// Check whether a code is good or not
pub fn xerr_is_good(code: xerr) -> bool {
    (code & 1) == 0
}

/// Check whether a code is bad or not
pub fn xerr_is_bad(code: xerr) -> bool {
    (code & 1) != 0
}
