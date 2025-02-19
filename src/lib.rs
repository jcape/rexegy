//! Unofficial Exegy Rust Binding

mod error;
mod key;
mod object;
mod session;

pub use crate::{
    object::Kind,
    error::{Error, Result},
    session::Session,
};
