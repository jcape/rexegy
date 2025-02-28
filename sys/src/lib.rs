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
    use std::ptr;

    #[rustfmt::skip]
    include!("gen.inc.rs");

    impl Default for XC_FORMAT_CONTROL {
        fn default() -> Self {
            Self {
                xfc_obuf: ptr::null_mut(),
                xfc_obufsiz: Default::default(),
                xfc_optr: ptr::null_mut(),
                xfc_oprevptr: ptr::null_mut(),
                XFC_RESERVED_00: Default::default(),
                xfc_no_nul_term: Default::default(),
                xfc_add_space_after: Default::default(),
                xfc_auto_expand: Default::default(),
                xfc_field_width: Default::default(),
                XFC_RESERVED_01: Default::default(),
                xfc_fw_alternate_id: Default::default(),
                xfc_fw_country_id: Default::default(),
                xfc_fw_currency_id: Default::default(),
                xfc_fw_date: Default::default(),
                xfc_fw_enum: Default::default(),
                xfc_fw_exchange_id: Default::default(),
                xfc_fw_key: Default::default(),
                xfc_fw_numeric: Default::default(),
                xfc_fw_price: Default::default(),
                xfc_fw_string: Default::default(),
                xfc_fw_symbol: Default::default(),
                xfc_fw_time: Default::default(),
                xfc_fw_timedate: Default::default(),
                xfc_fw_traded_value: Default::default(),
                xfc_fw_volume: Default::default(),
                xfc_indent_struct: Default::default(),
                xfc_fw_order_ref_id: Default::default(),
                xfc_fw_mmid: Default::default(),
                XFC_RESERVED_02: Default::default(),
                xfc_opt_alternate_id: Default::default(),
                xfc_opt_country_id: Default::default(),
                xfc_opt_currency_id: Default::default(),
                xfc_opt_enum: Default::default(),
                xfc_opt_exchange_id: Default::default(),
                xfc_opt_key: Default::default(),
                xfc_opt_numeric: Default::default(),
                xfc_opt_price: Default::default(),
                xfc_opt_string: Default::default(),
                xfc_opt_symbol: Default::default(),
                xfc_opt_timedate: Default::default(),
                xfc_opt_traded_value: Default::default(),
                xfc_opt_volume: Default::default(),
                xfc_opt_struct: Default::default(),
                xfc_opt_order_ref_id: Default::default(),
                xfc_opt_mmid: Default::default(),
                XFC_RESERVED_03: Default::default(),
                xfc_def_price_type: Default::default(),
                xfc_def_qual_type: Default::default(),
                XFC_RESERVED_04: Default::default(),
                XFC_RESERVED_05: Default::default(),
                xfc_magic_cookie: Default::default(),
            }
        }
    }
}

pub use self::generated::*;

/// Check whether a code is good or not
pub fn xerr_is_good(code: xerr) -> bool {
    (code & 1) == 0
}

/// Check whether a code is bad or not
pub fn xerr_is_bad(code: xerr) -> bool {
    (code & 1) != 0
}
