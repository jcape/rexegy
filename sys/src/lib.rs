/// Unofficial Rust FFI for XCAPI

mod gen {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use gen::*;