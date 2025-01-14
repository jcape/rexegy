//! Unofficial Rust FFI for XCAPI

/// The generated code will go into gen.
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
mod gen {
    include!("gen.inc.rs");
}

pub use gen::*;