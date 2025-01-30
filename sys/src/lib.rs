//! Unofficial Rust FFI for XCAPI

/// The generated code will go into gen.
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, clippy::missing_safety_doc, clippy::ptr_offset_with_cast, clippy::useless_transmute, clippy::unnecessary_cast, clippy::too_many_arguments)]
mod gen {
    include!("gen.inc.rs");
}

pub use gen::*;